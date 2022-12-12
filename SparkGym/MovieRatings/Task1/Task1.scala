import org.apache.spark.{SparkContext, SparkConf}

object Task1 {
    def main(args: Array[String]): Unit = {
        val conf = new SparkConf().setAppName("Movie Ratings Task 1")
        val sc = new SparkContext(conf)

        val textFile = sc.textFile(args(0))

        val output = textFile.map(line => line.split(","))
            .map(line => line.map(x => if (x == "") "0" else x))
            .map(line => (line(0), line.drop(1)))
            .map(t => (t._1, t._2.zipWithIndex, t._2.max))
            .map(t => (t._1, t._2.filter(rating => rating._1 == t._3).map(rating => rating._2.toInt + 1)))
            .map(t => t._1 + "," + t._2.mkString(","))

        output.saveAsTextFile(args(1))
    }
}