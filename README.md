# bfc

a brainfuck compiler written in rust for the x86_64 architecture

## about

this compiler writes the assembly directly and then uses [fasm](https://flatassembler.net/) to compile it into an executable

## how to use

```
Usage: bfc [OPTIONS] <FILE>

Arguments:
  <FILE>  path to the brainfuck source file

Options:
  -o <OUTPUT>      the name used for the generated assembly file and executable (no extension) [default: output]
  -h, --help       Print help
  -V, --version    Print version
```
