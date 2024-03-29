#!/bin/sh

echo --- "Validating user input"
export skipBuild="true"
while getopts :s: flag
do
    case "${flag}" in 
        s) skipBuild=${OPTARG};;
    esac
done
echo "-s : skipBuild : $skipBuild"
echo


echo --- "Ensuring current directory is set to the current path of this script"
SCRIPT_DIR=`cd "$(dirname "$0")" && echo $PWD`;
ROOT_DIR=$SCRIPT_DIR
echo SCRIPT_DIR=$SCRIPT_DIR
echo ROOT_DIR=$ROOT_DIR
cd $SCRIPT_DIR
echo


echo --- "Setting and printing variables"
export OUTPUT=`echo $SCRIPT_DIR/outputs`
export INPUT=$ROOT_DIR/inputs/sample.txt
export NUM_CORES=*
echo OUTPUT=$OUTPUT
echo INPUT=$INPUT
echo NUM_CORES=$NUM_CORES
echo


if [ $skipBuild == "false" ]
then
    # We use SBT to compile and run a Scala project, and package the project as a JAR file.
        # - Reference: https://alvinalexander.com/scala/sbt-how-to-compile-run-package-scala-project/
        # 1.1) NB: "sbt compile" && "sbt package" is just a longer variant of
        # "sbt compile package" which given that "compile" task is a 
        # dependency of "package" is just "sbt package".
        # 1.2) "sbt assembly": The goal is simple: Create a fat JAR of your project with all of its dependencies.
        # Reason: Have all dependencies bundled together in one jar file that is much easier to deploy to production (akin to a Docker image).
            # - Reference:https://stackoverflow.com/questions/53744950/when-to-use-sbt-assembly-and-sbt-compile-sbt-package
    # "$?"" always stores the exit status of the last executed command.
    echo --- "Compiling and Jarring"
    sbt package
    if [ $? -eq 0 ]
    then 
        echo && echo "Build Succeeded!" && echo
        echo
    else 
        echo && echo "Build Failed! Exiting!" && echo
        exit
    fi
else 
    echo --- "Skipping build"
fi
echo


# Output folder here must be recreated by the spark job run.
echo --- "Deleting previous output folder"
rm -R $OUTPUT
echo


# Meaning of the build arguments:
    # - Current setups runs application locally on number of "cores" specified by $NUM_CORES.
    # - In Spark UI, "executors" is the number of "nodes" (local machine is 1 node).
    # - Spark UI also specifies number of "cores" used per "executor".
    # - eventLog configures Spark to log Spark events that encode the information displayed in the UI to persisted storage.
    # - eventLog is needed for spark history server to work.
    # - Driver Program: The process running the main() function of the application and creating the SparkContext
    # - Executor: A process launched for an application on a worker node, that runs tasks and keeps data in memory or disk storage across them. Each application has its own executors.
echo --- "Running newly built spark app"
time $SPARK_HOME/bin/spark-submit \
--class "SparkPartitions" \
--master local[$NUM_CORES] \
--driver-memory 4g \
--executor-memory 4g \
--conf "spark.eventLog.enabled=true" \
--conf "spark.eventLog.dir=file:///tmp/spark-events" \
target/scala-2.11/spark-partitions_2.11-1.0.jar \
$INPUT \
$OUTPUT
echo
# Note: do NOT add new line comments between the spark-submit options, as this breaks the script.


echo --- "Combining output file chunks into 1 output file"
cd $OUTPUT
cat $OUTPUT/* | sort > normalized_output.txt
echo "Done"
echo
