#!/bin/bash
i=0;
while true
do
    i=$(($i + 1))
    echo $i
	cargo run
	# Enter your desired command in this block.
done
