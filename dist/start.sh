#!/bin/bash

port=$1
jar="./bingeAtHome-0.1.0.jar"

if ! [ -f $jar ]; then
    echo "error: $jar does not exist. Please run the setup script or manually build it before running this!" >&2; exit 1
fi

if [ -z $port ]; then
	java -jar $jar
else
	re='^[0-9]+$'
	if ! [[ $port =~ $re ]]; then
	   echo "error: Not a valid port number!" >&2; exit 1
	fi
	java -jar $jar --server.port=$port
fi