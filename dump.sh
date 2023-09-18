#!/bin/bash

input_file="./build/memory/memory.dat"
output_prefix="./build/memory/output_file"

rm output_file*
split -d -b 250m "$input_file" "${output_prefix}_"

chunk_number=0
chunk_size=$((250 * 1024 * 1024)) # 250MB in bytes
file_size=$(stat -f%z "$input_file")
offset=0

while [ $offset -lt $file_size ]; do
  chunk_number_hex=$(printf '%02d' $chunk_number)
  hexdump -C -s $offset "${output_prefix}_${chunk_number_hex}" > "${output_prefix}_${chunk_number_hex}.hexdump"
  offset=$(($offset + $chunk_size))
  chunk_number=$(($chunk_number + 1))
done
