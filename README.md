# bfc

a really bad brainfuck-to-rust transpiler and interpreter written in rust.
addition and subtraction have been "optimized" but i honestly don't know if i broke something in the process.
i'll get around to "optimizing" the rest some day.

# usage

generate the bfc.exe file with ```cargo build --release``` and find it located at .\target\release\bfc.exe <br />

run the file with ```bfc.exe <FILE> [flags]```

# current flags:

-o: allows you to specify the name of the output file <br />
-t, --time: tells you how long the compiler took to generate the exe file from source. <br />
-q, --quiet: prevents the compiler from generating any output to the console <br />
-p, --pretty: runs rustfmt on the generated rust file, only use if you want to look at the source code easily <br />
-i: runs the compiler in interpreter mode. its not optimized yet and was just kind of shoved in there, sooo...

