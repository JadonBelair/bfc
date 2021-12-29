# bfc

a really bad brainfuck "compiler". i kinda just cheat by writing a rust file and then just calling rustc on it, so its more of a brainfuck to rust converter than anything.
there is no optimization, so it can struggle to compile large files with lots of repeating actions. i might come back to this just to try and optimize it but who knows.

# usage

generate the bfc.exe file with ```cargo build --release``` and find it located in ./target/release/ <br />

run the file with ```bfc [filepath] [flags]```

# current flags:

-time: tells you how long the compiler took to generate the exe file from source. <br />
-o: allows you to specify the name of the output file
