/* START: Spark Quick Start Learning */

// Run these on interactive Spark Scala terminal at $SPARK_HOME/bin/spark-shell

val textFile = spark.read.textFile("/Users/tarek.a/code/Practice/Data-Engineering-Practice/SparkPlayground/tutorial-samples/spark-config-compare.txt")

// For multi-line commands you can use `:paste`
val lineParity = textFile.map(line => 
    line.substring(0,Math.min(line.length(),3))
        match {
            case "(-)" => -1
            case "(+)" => +1
            case _ => 0
        }
)

// Expected Output: 
//      minParity: Int = -1
val minParity = lineParity.reduce((x,y) => Math.min(x, y))

// Expected Output: 
//      lineCountPerParity: Array[(Int, Int)] = Array((-1,8), (0,20), (1,10)) 
val lineCountPerParity = lineParity.map(parity => (parity,1)).rdd.reduceByKey(_+_).collect()

/* END: Spark Quick Start Learning */


/* START: Spark SQL Learning */

// TODO: fix this read (broken currently...)

val df = spark.read.parquet("/Users/tarek.a/Library/CloudStorage/OneDrive-Microsoft/Documents/Work Diary/Extra/2022-08/normalized-uks-userbi/part-00000-tid-8075384613682616779-9a25af95-1543-4e01-aaf0-77b805abb28d-25547-c000.snappy.parquet")

val df = spark.read.json("/Users/tarek.a/Library/CloudStorage/OneDrive-Microsoft/Documents/Work Diary/Extra/2022-08/vef-uks-userbi/LN2PEPF000013FD_MicrosoftTeamsUserBI_95_2022-08-22T19_03_37.9497029Z_43fab16f379740c98f69d802943f1738.json")

// Ran some commands from https://spark.apache.org/docs/2.3.0/sql-programming-guide.html to investigate DFs.

/* END: Spark SQL Learning */