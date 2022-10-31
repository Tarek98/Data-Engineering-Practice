#!/bin/sh

# NOTES: +/-:
    # 1: ...
#


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
SCRIPT_DIR="$( cd -- "$( dirname -- "${BASH_SOURCE[0]:-$0}"; )" &> /dev/null && pwd 2> /dev/null; )";
ROOT_DIR=`cd $SCRIPT_DIR && cd .. && echo $PWD`
echo SCRIPT_DIR=$SCRIPT_DIR
echo ROOT_DIR=$ROOT_DIR
cd $SCRIPT_DIR
echo


echo --- "Setting and printing variables"
export OUTPUT=`echo $SCRIPT_DIR/outputs`
export INPUT=$ROOT_DIR/sample_inputs/smalldata.txt
export NUM_CORES=6
echo OUTPUT=$OUTPUT
echo INPUT=$INPUT
echo NUM_CORES=$NUM_CORES
echo


if [ $skipBuild == "false" ]
then
    echo --- "Compiling and Jarring"
    sbt package
    if [ $? -eq 0 ]
    then 
        echo && echo "Build Succeeded! Proceeding to run spark job!" && echo
        echo
    else 
        echo && echo "Build Failed! Exiting!" && echo
        exit
    fi
else 
    echo --- "Skipping build"
fi
echo


echo --- "Deleting previous output folder"
rm -R $OUTPUT
echo


echo --- "Running newly built spark app"
time $SPARK_HOME/bin/spark-submit \
--class "Task4" \
--master local[$NUM_CORES] \
--driver-memory 4g \
--executor-memory 4g \
--conf "spark.eventLog.enabled=true" \
--conf "spark.eventLog.dir=file:///tmp/spark-events" \
target/scala-2.11/movie-ratings-task-4_2.11-1.0.jar \
$INPUT \
$OUTPUT
echo


echo --- "Combining output file chunks into 1 output file"
cd $OUTPUT
cat $OUTPUT/* | sort > normalized_output.txt
echo "Done"
echo
