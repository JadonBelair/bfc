use crate::lexer::Lexer;
use crate::lexer::token::{Token, TokenType};

mod lexer;

fn main() {
    println!("Hello, world!");
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

; this read section actually reads 3 bytes from stdin in order to get through the \r\n, so piping a file into stdin wont work as expected
read:
    mov rax, SYS_read
    mov rdi, stdin
    mov rsi, read_temp
    mov rdx, 3
    syscall
    mov rax, [read_temp] ; we use read_temp in order to not contaminate the bytes in the neighbouring cells with the \r\n
    mov byte [r8], al
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
                output.push_str(&format!("    add byte[r8], {}\n", token.amount));
            },
            TokenType::Subtract => {
                output.push_str(&format!("    sub byte[r8], {}\n", token.amount));
            },
            TokenType::Left => {
                output.push_str(&format!("    sub r8, {}\n", token.amount));
            },
            TokenType::Right => {
                output.push_str(&format!("    add r8, {}\n", token.amount));
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
                output.push_str(&format!("    jump_label_{jump_counter}:\n"));
                jump_stack.push(jump_counter);
                jump_counter += 1;
            },
            TokenType::JumpIfNotZero => {
                if let Some(counter) = jump_stack.pop() {
                    output.push_str("    cmp byte [r8], 0\n");
                    output.push_str(&format!("    jne jump_label_{counter}\n"));
                    output.push_str(&format!("    jump_end_label_{counter}:\n"));
                }
            }
        }
    }

    output.push_str(
r#"
    mov rax, 60
    mov rdi, 0
    syscall

segment readable writable
bf_stack: rb 1000
read_temp: rb 5
"#
    );

    return output;
}
