#!/usr/bin/env bash
while read line
do 
    echo "a 1 2; b 3 4; $line"
    sleep 1
done < "${1:-/dev/stdin}"
