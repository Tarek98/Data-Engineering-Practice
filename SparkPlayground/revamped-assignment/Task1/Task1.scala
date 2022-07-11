import org.apache.spark.{SparkContext, SparkConf}

object Task1 {
    def main(args: Array[String]): Unit = {
        val conf = new SparkConf().setAppName("Task 1")
        val sc = new SparkContext(conf)

        val textFile = sc.textFile(args(0))

        val output = textFile

        output.saveAsTextFile(args(1))
    }
}