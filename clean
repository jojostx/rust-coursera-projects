#!/bin/bash
echo "Searching for Rust project subfolders..."
for dir in $(find . -type f -name 'Cargo.toml' -exec dirname {} \;); do
  if [ -f "$dir/Makefile" ]; then
    echo "Running make clean in $dir"
    (cd "$dir" && make clean)
  else
    echo "Skipping $dir (no Makefile)"
  fi
done