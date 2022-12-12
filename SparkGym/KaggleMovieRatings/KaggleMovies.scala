import org.apache.spark.{SparkContext, SparkConf}
import org.apache.spark.sql.{SparkSession, DataFrame, Encoders}
import org.apache.spark.sql.functions._
import org.apache.spark.sql.types._


// This program is intended for debugging on VS Code & evaluating expressions during runtime for fast experimentation.
// Prod instance of this program will be run on a Spark Synapse Cluster.
object KaggleMovies {    
    def main(args: Array[String]): Unit = {
        val spark = SparkSession.builder().master("local[4]").appName("KaggleMovies").getOrCreate()
        val sc = spark.sparkContext
        // spark.conf.set("spark.serializer", "org.apache.spark.serializer.KryoSerializer")


        var movieDF = spark.read.format("json").option("multiLine","true").load("./big-file-input.json")

        // START: Task 1

        val x = movieDF.where(col("spoiler_tag") === 1).count()


        val subset = movieDF.where(col("movie").contains("Kill Bill"))
            .groupBy(col("movie")).count()
            .select(col("movie"))
            .map(row => row.getString(0), Encoders.STRING)
            .collect()


        // val t1 = movieDF.where(col("movie").isin(subset:_*))
        //     .withColumn("ratingNum", col("rating").cast(IntegerType))
        //     .groupBy(col("movie"))
        //     .agg(collect_list("ratingNum"))

        // TODO: Use select optimization instead of withColumn.
        // val columns = t1.columns 

        val t1 = movieDF.where(col("movie").isin(subset:_*))
            .withColumn("ratingNum", col("rating").cast(IntegerType))
            .groupBy(col("movie"))
            .agg(collect_list(concat(col("reviewer"),lit(" = "),col("rating"))).as("ratingList"), max("ratingNum"))

        val y = 2

        // t1.select(col("movie"), col("ratingList").split(" = ")(0)).show()


        // TODO: COULD try using a join instead --> (movie,maxRating) & (movie,reviewer,rating)
        // BEST to try both approaches to get stronger grasp of Spark SQL though :-)
        

        // val textFile = sc.textFile(args(0))

        // val output = textFile.map(line => line.split(","))
        //     .map(line => line.map(x => if (x == "") "0" else x))
        //     .map(line => (line(0), line.drop(1)))
        //     .map(t => (t._1, t._2.zipWithIndex, t._2.max))
        //     .map(t => (t._1, t._2.filter(rating => rating._1 == t._3).map(rating => rating._2.toInt + 1)))
        //     .map(t => t._1 + "," + t._2.mkString(","))

        // output.saveAsTextFile(args(1))

        // END: Task 1
    }
}