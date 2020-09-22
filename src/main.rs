use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let arg1 = &args[1];

    println!("  .intel_syntax noprefix");
    println!("  .global _start");
    println!("_start:");
    println!("    mov rax, 60");
    println!("    mov rdi, {}", arg1);
    println!("    syscall");
}
