import org.apache.spark.{SparkContext, SparkConf}

object SparkWordCount {
  def main(args: Array[String]): Unit = {
    val conf = new SparkConf().setAppName("Spark Word Count Application")
    val sc = new SparkContext(conf)

    // val textFile = sc.textFile(args(0))

    // // TODO: WIP: partitions investigation --> for input & output.
    // println("\nDEBUG 1: Num Partitions: " + textFile.getNumPartitions)
    // textFile.repartition(4)
    // println("DEBUG 2: Num Partitions: " + textFile.getNumPartitions + "\n")

    // val counts = textFile.flatMap(line => line.split(" "))
    //              .map(word => (word, 1 ))
    //              .reduceByKey(_ + _)
		//  .map(x => x._1 + "," + x._2)
    
    // // while (true) {
    // //   // Loop stall spark UI
    // // }

    // counts
		//  .saveAsTextFile(args(1))

    // Start: Testing: Move to other program
    // READ: https://stackoverflow.com/questions/31424396/how-does-hashpartitioner-work
    // Spark will run one task for each partition of the cluster.
    val numPartitions = 8
    val rdd = sc.parallelize(
      Array(1,2,3,4,5,6)
      // Note: If we comment the line below, spark sets # partitions automatically based on the number of cores provided.
      ,numPartitions
    )

    val output = rdd
      .mapPartitionsWithIndex{ 
        (pIndex, pIterator) 
        => pIterator.toList.map(pElement => (pElement, " P#"+pIndex)).iterator
      }

    // Notice that in the saved output files, we get each partition as one file.
    // So if "numPartitions" is 8, we get 8 output files, even if some of these
    // files are empty because a partition was unused.
    val saved_output = output.saveAsTextFile(args(1))

    val printed_output = output.collect().mkString(",") 

    println(
      "\nDEBUG: output : " + 
      printed_output
      + "\n"
    )
    // End: Testing: Move to other program
  }
}
