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

        // This input file was git ignored since it's too big for the GitHub repo: using sample.json from https://www.kaggle.com/datasets/ebiswas/imdb-review-dataset
        var movieDF = spark.read.format("json").option("multiLine","true").load("./big-file-input.json")
            .withColumn("rating", col("rating").cast(IntegerType))

        val testMovies = movieDF.where(col("movie").contains("Kill Bill"))

        // START: Task 1: Get reviewer names with the highest rating for each movie.
        val allRatings = testMovies.select("movie", "reviewer", "rating")

        val maxRatings = testMovies
            .groupBy(col("movie"))
            .agg(max(col("rating")).as("rating"))

        val t1Result = allRatings.join(maxRatings, Seq("movie", "rating"), "inner")
            .groupBy(col("movie"),col("rating").as("maxRating"))
            .agg(collect_list(col("reviewer")).as("maxRatingReviewers"))
            .withColumn("countOfMaxRatingReviewers", size(col("maxRatingReviewers")))
        // END: Task 1

        // Debug breakpoint
        val y = 2
        // t1Result.sort(col("movie").asc).limit(100).show(false)
        // allRatings.sort(col("movie").asc, col("rating").desc).limit(100).show(false)

    }
}


/*
// Show sample table in debug console
movieDF.display(false)

val testMovies = movieDF.where(col("movie").contains("Kill Bill"))
    .groupBy(col("movie")).count()
    .select(col("movie"))
    .map(row => row.getString(0), Encoders.STRING)
    .collect()
*/