#
# Licensed to the Apache Software Foundation (ASF) under one or more
# contributor license agreements.  See the NOTICE file distributed with
# this work for additional information regarding copyright ownership.
# The ASF licenses this file to You under the Apache License, Version 2.0
# (the "License"); you may not use this file except in compliance with
# the License.  You may obtain a copy of the License at
#
#    http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.
#

"""
This is an example implementation of PageRank. For more conventional use,
Please refer to PageRank implementation provided by graphx

Example Usage:
bin/spark-submit examples/src/main/python/pagerank.py data/mllib/pagerank_data.txt 10
"""
import re
import sys
from operator import add

from pyspark.sql import SparkSession


def computeContribs(urls, rank):
    """Calculates URL contributions to the rank of other URLs."""
    num_urls = len(urls)
    for url in urls:
        yield (url, rank / num_urls)


def parseNeighbors(urls):
    """Parses a urls pair string into urls pair."""
    parts = re.split(r'\s+', urls)
    return parts[0], parts[1]


if __name__ == "__main__":
    if len(sys.argv) != 3:
        print("Usage: pagerank <file> <iterations>", file=sys.stderr)
        sys.exit(-1)

    print("WARN: This is a naive implementation of PageRank and is given as an example!\n" +
          "Please refer to PageRank implementation provided by graphx",
          file=sys.stderr)

    # Initialize the spark context.
    spark = SparkSession\
        .builder\
        .appName("PythonPageRank")\
        .getOrCreate()

    # Loads in input file. It should be in format of:
    #     URL         neighbor URL
    #     URL         neighbor URL
    #     URL         neighbor URL
    #     ...
    lines = spark.read.text(sys.argv[1]).rdd.map(lambda r: r[0])
    # Tarek: See sample for debugging below: 
    #   -> Use RDD Actions (e.g. collect() or count()) to view intermediary RDDs at Spark Driver Program 
    #      --> Spark follows a lazy computation model so it doesn’t compute anything until necessary.
    #          (All RDD transformations are executed only when an RDD action is invoked)
    #      --> Not possible to view full picture from executors.
    #      --> When DEBUGGING, you should call count() on your RDDs to see what stage your error occurred.
    #          (+Use it for OPTIMIZATION as it can measure running time of each individual stage)
    # lines = spark.read.text(sys.argv[1]).rdd.collect()

    # Loads all URLs from input file and initialize their neighbors.
    links = lines.map(lambda urls: parseNeighbors(urls)).distinct().groupByKey().cache()

    # Loads all URLs with other URL(s) link to from input file and initialize ranks of them to one.
    ranks = links.map(lambda url_neighbors: (url_neighbors[0], 1.0))
    prev_ranks = None

    # Calculates and updates URL ranks continuously using PageRank algorithm.
    # for iteration in range(int(sys.argv[2])):
    loops = 0
    while True:
        # Calculates URL contributions to the rank of other URLs.
        contribs = links.join(ranks).flatMap(
            lambda url_urls_rank: computeContribs(url_urls_rank[1][0], url_urls_rank[1][1]))

        # Re-calculates URL ranks based on neighbor contributions.
        ranks = contribs.reduceByKey(add).mapValues(lambda rank: rank * 0.85 + 0.15)

        # Tarek: Add logic to exit loop if rank has converged, instead of running fixed iterations from sys.argv[2]
        #        -> Simple convergence test: 2 consecutive rank RDDs have a maximum of 0.0001 difference 
        #        -> More optimal convergence test may be better for real application...
        #        -> TODO: check how to optimize & evaluate this job's performance
        #                 -> TODO (DONE): Investigate Web UI to view how many stages were added by this divergence check (result = 2 stages extra)
        #                 -> TODO (TBD): Attempt removing the 2 extra stages (effectively removing 2 expensive shuffles)
        #                    -> Idea 1: 
        #                       -> Modify the "ranks" RDD: each "URL" key maps to tuple "(new_rank, prev_rank)" instead of just "new_rank" 
        #                       -> Invoke action of countByValue(<divergence_check>) to identify "num_diverged"
        #                    -> Idea 2: 
        #                       -> Research how to minimize time spent in GC in Spark
        #                       -> ... 
        #        -> UW Performance Tuning Tips: minimize # job stages, # data shuffles, time spent in garbage collection.
        if prev_ranks != None:
            num_diverged = ranks.join(prev_ranks).filter(lambda row: abs(row[1][0]-row[1][1]) > 0.0001).count()
            if num_diverged == 0:
                print(f"Success: Ranks have finally converged after {loops} loops!")
                break
        prev_ranks = ranks

        loops += 1

    # Collects all URL ranks and dump them to console.
    for (link, rank) in ranks.collect():
        print("%s has rank: %s." % (link, rank))

    # Tarek: To visualize Spark execution: Access the Web UI:
    #   -> Uncomment the line below        
    #   -> See https://spark.apache.org/docs/latest/monitoring.html#web-interfaces
    #   -> This pause method should only be used for local testing env
    #      --> Why: In a production env, having multiple idle SparkContexts stalling can be troublesome?
    #   -> What you should do in production or expensive envs:
    #      --> Investigate long running jobs using their Web UI during execution.
    #      --> Use SHS (Spark History Server) to reconstruct a past UI, provided that the application’s event logs exist.
    pause = input("Press Enter to resume execution")

    spark.stop()
    