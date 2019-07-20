#!/bin/bash

examples_path=`pwd`/../examples

for file in `ls ${examples_path}`
do
  abs_path=${examples_path}/${file}
  if [ -d $abs_path ]; then
    echo -n "Cleaning project: ${file}"
    cd ${abs_path} &> /dev/null && cargo clean
    if [ $? == 0 ]; then
        echo -e "\t\t\t\t\e[1;32m [OK] \e[0m"
    else
        echo -e "\t\t\t\t\e[1;31m [FAILED] \e[0m"
    fi
  fi
done
