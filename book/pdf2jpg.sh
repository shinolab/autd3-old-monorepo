#!/bin/sh
for f in $(find . -name "*.pdf")
do
  f2=`echo $f | sed -e "s/pdf$/jpg/"` 
  convert -density 300 -units PixelsPerInch $f $f2
  rm $f
done
