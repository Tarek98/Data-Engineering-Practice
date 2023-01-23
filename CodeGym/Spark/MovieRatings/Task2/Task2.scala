import org.apache.spark.{SparkContext, SparkConf}

object Task2 {
    def main(args: Array[String]): Unit = {
        val conf = new SparkConf().setAppName("Movie Ratings Task 2")
        val sc = new SparkContext(conf)

        val minPartitions = sc.defaultParallelism /* See diff on spark UI */

        val textFile = sc.textFile(args(0), minPartitions)

        val output = textFile.map(line => line.split(","))
            .map(_.filter(_.trim().length > 0))
            .map(x => ("key",x.length - 1))
            .reduceByKey(_+_,1)
            .map(_._2)

        output.saveAsTextFile(args(1))
    }
}