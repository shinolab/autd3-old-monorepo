#!/bin/sh
for f in $(find . -name "*.png")
do
  f2=`echo $f | sed -e "s/png$/jpg/"` 
  convert $f $f2
  rm $f
done
