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
