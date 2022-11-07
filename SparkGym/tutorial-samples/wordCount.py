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

import sys
from operator import add

from pyspark.sql import SparkSession


if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("Usage: wordcount <file>", file=sys.stderr)
        sys.exit(-1)

    spark = SparkSession\
        .builder\
        .appName("PythonWordCount")\
        .getOrCreate()

    lines = spark.read.text(sys.argv[1]).rdd.map(lambda r: r[0])
    counts = lines.flatMap(lambda x: x.split(' ')) \
                  .map(lambda x: (x, 1)) \
                  .reduceByKey(add)
    output = counts.collect()
    for (word, count) in output:
        print("%s: %i" % (word, count))

    # Tarek: To visualize Spark execution: Access the Web UI:
    #   -> Uncomment the line below        
    #   -> See https://spark.apache.org/docs/latest/monitoring.html#web-interfaces
    #   -> This pause method should only be used for local testing env
    #      --> Why: In a production env, having multiple idle SparkContexts stalling can be troublesome?
    #   -> What you should do in production or expensive envs:
    #      --> Investigate long running jobs using their Web UI during execution.
    #      --> Use SHS (Spark History Server) to reconstruct a past UI, provided that the application’s event logs exist.
    # pause = input("Press Enter to resume execution")

    spark.stop()