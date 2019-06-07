#!/usr/bin/env bash
while read line
do 
    echo "$line"
    sleep 1
done < "${1:-/dev/stdin}"
