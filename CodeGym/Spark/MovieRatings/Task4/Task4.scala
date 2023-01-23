import org.apache.spark.{SparkContext, SparkConf}
// import org.apache.spark.sql.SparkSession

object Task4 {
    def main(args: Array[String]): Unit = {
        val conf = new SparkConf().setAppName("Movie Ratings Task 4")
        val sc = new SparkContext(conf)

        val minPartitions = sc.defaultParallelism

        val textFile = sc.textFile(args(0), minPartitions)
        val allMovies = sc.broadcast(textFile.map(line => line.split(",", -1)).collect())

        val output = textFile.map(line => line.split(",",-1))
            // Cross-product: takes a single list, turns it into a list of tuples of tuples (one tuple for each pair of movies)
            .map(firstLine => allMovies.value.map(secondLine => ((firstLine(0), secondLine(0)), (firstLine.drop(1), secondLine.drop(1)))))
            // A single line now looks like: [ (("LotR", "LotR"), (["5", "4"], ["5", "4"])), (("LotR","Hype"), (["5", "4"], ["5","2"]))]
            // Remove duplicates and those with the wrong lexicographical ordering
            .map(line => line.filter(tuple => tuple._1._1 < tuple._1._2))
            // .zipped function takes a tuple of lists and transposes it into a list of tuples, each tuple of which we convert to 1 or 0 before summing
            .map(line => line.map(tuple => (tuple._1, tuple._2.zipped.map((a, b) => if (a == b && a != "" && b != "") 1 else 0).sum)))
            // Format output correctly
            .flatMap(line => line.map(tuple => s"${tuple._1._1},${tuple._1._2},${tuple._2}"))

        output.saveAsTextFile(args(1))

        // val spark = SparkSession.builder().getOrCreate()
    }
}