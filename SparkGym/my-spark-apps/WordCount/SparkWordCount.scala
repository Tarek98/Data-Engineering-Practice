import org.apache.spark.{SparkContext, SparkConf}

object SparkWordCount {
  def main(args: Array[String]): Unit = {
    val conf = new SparkConf().setAppName("SparkWordCount")
    val sc = new SparkContext(conf)

    val textFile = sc.textFile(args(0))

    val counts = textFile.flatMap(line => line.split(" "))
      .map(word => word.filter(_.isLetter).trim)
      .filter(word => word.length > 2)
      .map(word => (word, 1))
      .reduceByKey(_ + _)
		  .map(x => x._1 + "," + x._2)

    counts.saveAsTextFile(args(1))
  }
}
