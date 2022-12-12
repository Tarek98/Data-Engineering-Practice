import org.apache.spark.{SparkContext, SparkConf}

object Task3 {
    def main(args: Array[String]): Unit = {
        val conf = new SparkConf().setAppName("Movie Ratings Task 3")
        val sc = new SparkContext(conf)

        val minPartitions = sc.defaultParallelism

        val textFile = sc.textFile(args(0), minPartitions)

        val output = textFile.map(line => line.split(",",-1))
            .map(_.drop(1))
            .map(_.zipWithIndex)
            .flatMap(array => array.map(x => (x._2 + 1, if (x._1.trim().length > 0) 1 else 0)))            
            .reduceByKey(_+_)
            .map(t => s"${t._1},${t._2}")

        output.saveAsTextFile(args(1))
    }
}