#!/bin/bash

sum=0

cat $(dirname $0)/../input.txt | while read line
do
  
  result=""
  for (( i=0; i < ${#line}; i++ )); do
    char="${line:$i:1}"
    if [[ $char =~ [0-9] ]]; then
      if [[ ${#result} == 0 ]]; then
        result="$result$char"
      fi
    fi
  done
  for (( i=${#line}-1; i >= 0; i-- )); do
    char="${line:$i:1}"
    if [[ $char =~ [0-9] ]]; then
      if [[ ${#result} == 1 ]]; then
        result="$result$char"
      fi
    fi
  done
  echo "$line: $result"
  sum=$(($sum + $result))
  echo "Sum: $sum"
done
