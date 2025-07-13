#!/bin/bash

if [ ! -d "dots" ]; then
  echo "Dir dots not found"
  exit 1
fi

if ! command -v dot &> /dev/null; then
  echo "Graphviz not installed."
  echo "sudo apt-get install graphviz"
  echo "or"
  echo "brew install graphviz"
  exit 1
fi

mkdir -p dots/images

find dots/ -type f -name "*.dot" | while read -r dot_file; do
  filename=$(basename -- "$dot_file")
  filename="${filename%.*}"
  subdir=$(dirname "$dot_file" | sed 's|^dots/||')
  
  mkdir -p "dots/images/$subdir"
  
  echo "Creating image for $dot_file..."
  dot -Tpng "$dot_file" -o "dots/images/$subdir/${filename}.png"
done

echo "All images saved in dots/images/"
