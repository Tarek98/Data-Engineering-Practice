// IDE Integration: How to use intellisense: 
//    1. Install Metals
//    2. Open dir containing build.sbt file
//    3. From the menubar, run View > Command Palette… (Cmd-Shift-P on macOS) “Metals: Switch build server”, and select “sbt”
//    4. Once the import process is complete, open a Scala file to see that code completion works.
//    Reference: https://www.scala-sbt.org/1.x/docs/IDE.html

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
