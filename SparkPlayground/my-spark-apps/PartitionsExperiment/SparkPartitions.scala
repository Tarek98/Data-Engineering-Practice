import org.apache.spark.{SparkContext, SparkConf}
import java.io._

object SparkPartitions {
  def main(args: Array[String]): Unit = {
    val conf = new SparkConf().setAppName("Spark Partitions")
    val sc = new SparkContext(conf)

    val logFile =  new File("debug_log.txt")
    val logger = new BufferedWriter(new FileWriter(logFile))
    logger.write("Debug Log \n\n")

    val minPartitions = sc.defaultParallelism /* usually equals to total number of cores across all executors for current spark app */
    val minReadFile = sc.textFile(args(0), minPartitions)
    val defaultReadFile = sc.textFile(args(0))  /* "By default, Spark creates one partition for each block of the file (blocks being 128MB by default in HDFS)" */

    logger.write(s"minReadFile.partitions.length = ${minReadFile.partitions.length} \n")
    logger.write(s"defaultReadFile.partitions.length = ${defaultReadFile.partitions.length} \n")

    // READ: https://stackoverflow.com/questions/31424396/how-does-hashpartitioner-work
    // Spark will run one task for each partition of the dataset/RDD.
    val numPartitions = 8
    val rdd = sc.parallelize( 
      Array(1,2,3,4,5,6)
      ,numPartitions
    )
    // ^Note: If we comment numPartitions, spark sets # partitions automatically based on the number of cores provided.


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

    logger.write(
      "\nprinted_output : " + 
      printed_output
      + "\n"
    )

    logger.close()
  }
}
