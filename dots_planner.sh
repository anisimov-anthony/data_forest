#!/bin/bash

if [ ! -d "dots" ]; then
  echo "Dir dots not found"
  exit 1
fi

if ! command -v dot &> /dev/null; then
  echo "Graphviz not installes."
  echo "sudo apt-get install graphviz"
  echo "or"
  echo "brew install graphviz"
  exit 1
fi

mkdir -p dots/images

for dot_file in dots/*.dot; do
  if [ -f "$dot_file" ]; then
    filename=$(basename -- "$dot_file")
    filename="${filename%.*}"

    echo "Creating image for $dot_file..."
    dot -Tpng "$dot_file" -o "dots/images/${filename}.png"
  fi
done

echo "All images saved in dots/images/"
