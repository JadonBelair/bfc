use std::process::{Command, Stdio, exit};

use clap::Parser;

use crate::lexer::Lexer;
use crate::lexer::token::{Token, TokenType};

mod lexer;

#[derive(Parser)]
#[clap(author, version, about, long_about)]
struct Args {
    /// the name used for the generated assembly file and executable (no extension)
    #[clap(short = 'o', default_value = "output")]
    output: String,

    /// path to the brainfuck source file
    #[clap(name = "FILE")]
    file: std::path::PathBuf,
}

fn main() -> Result<(), std::io::Error> {
    let args = Args::parse();

    let file_path = args.file;

    let file_contents = std::fs::read_to_string(&file_path)?;

    let mut lexer = Lexer::new(&file_contents);

    let ir = lexer.lex();
    let asm = generate_assembly(ir);
    
    let asm_file = format!("{}.asm", args.output);
    std::fs::write(&asm_file, asm)?;

    let status = Command::new("fasm")
        .arg(&asm_file)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()?;

    if status.success() {
        println!("program compiled successfully");
        exit(0);
    } else {
        eprintln!("failed to compile program");
        exit(1);
    }
}

fn generate_assembly(tokens: Vec<Token>) -> String {
    let mut output = String::from(
r#"format ELF64 executable 3
SYS_read equ 0
SYS_write equ 1
SYS_exit equ 60

stdin equ 0
stdout equ 1

segment readable executable
entry main

write:
    mov rax, SYS_write
    mov rdi, stdout
    mov rsi, r8
    mov rdx, 1
    syscall
    ret

read:
    mov rax, SYS_read
    mov rdi, stdin
    mov rsi, r8
    mov rdx, 1
    syscall
    ret

main:
    lea r8, [bf_stack]
"#
    );

    let mut jump_stack: Vec<u32> = Vec::new();
    let mut jump_counter = 0;

    for token in tokens {
        match token.token_type {
            TokenType::Add => {
                if token.amount > 1 {
                    output.push_str(&format!("    add byte[r8], {}\n", token.amount));
                } else {
                    output.push_str("    inc byte[r8]\n");
                }
            },
            TokenType::Subtract => {
                if token.amount > 1 {
                    output.push_str(&format!("    sub byte[r8], {}\n", token.amount));
                } else {
                    output.push_str("    dec byte[r8]\n");
                }
            },
            TokenType::Left => {
                if token.amount > 1 {
                    output.push_str(&format!("    sub r8, {}\n", token.amount));
                } else {
                    output.push_str("    dec r8\n");
                }
            },
            TokenType::Right => {
                if token.amount > 1 {
                    output.push_str(&format!("    add r8, {}\n", token.amount));
                } else {
                    output.push_str("    inc r8\n");
                }
            },
            TokenType::Output => {
                for _ in 0..token.amount {
                    output.push_str("    call write\n");
                }
            },
            TokenType::Input => {
                for _ in 0..token.amount {
                    output.push_str("    call read\n");
                }
            },
            TokenType::JumpIfZero => {
                output.push_str("    cmp byte [r8], 0\n");
                output.push_str(&format!("    je jump_end_label_{jump_counter}\n"));
                output.push_str(&format!("jump_label_{jump_counter}:\n"));
                jump_stack.push(jump_counter);
                jump_counter += 1;
            },
            TokenType::JumpIfNotZero => {
                if let Some(counter) = jump_stack.pop() {
                    output.push_str("    cmp byte [r8], 0\n");
                    output.push_str(&format!("    jne jump_label_{counter}\n"));
                    output.push_str(&format!("jump_end_label_{counter}:\n"));
                } else {
                    eprintln!("Error: missing matching open bracket");
                    exit(1);
                }
            }
        }

    }

    if jump_stack.len() > 0 {
        eprintln!("Error: missing closing bracket");
        exit(1);
    }

    output.push_str(
r#"
    mov rax, 60
    mov rdi, 0
    syscall

segment readable writable
bf_stack: rb 1000
"#
    );

    return output;
}
