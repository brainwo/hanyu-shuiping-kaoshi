#!/bin/bash

rustc remove_cl.rs && ./remove_cl &&

echo "✔ Done Converting" &&

for i in {1..6}
do
    sed -i ":begin;$!N;s/,\n    ]/\n    ]/g;tbegin;P;D" "../json/hsk-level-${i}-no-cl.json" > /dev/null
done &&

echo "✔ Done Formatting"

