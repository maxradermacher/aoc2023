#!/bin/sh

git commit -m "Day ${1#0}

$(cat $1.txt | python3 $1-1.py)
$(cat $1.txt | python3 $1-2.py)
"
