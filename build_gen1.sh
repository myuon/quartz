rm build/compiler/compiler.qz ; for file in ./quartz/*.qz; do
  if [ "$file" != "./quartz/std.qz" ]; then
    cat "$file" >> build/compiler/compiler.qz
  fi
done && just run_compiler < build/compiler/compiler.qz > ./build/compiler/gen1.wat

