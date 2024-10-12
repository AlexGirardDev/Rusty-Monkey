# Monkey lang Interpreter in Rust

This is a [Monkey programming language](https://monkeylang.org/) toy interpreter written in rust from the book [Writing An Interpreter In Go](https://interpreterbook.com) by Thorsten Ball


## Monkey

Monkey is a small, interpreted language that supports fundamental programming concepts like variables, functions, and conditionals, while also including advanced features like higher-order functions, closures, and various data types.

From the book: 
> "It supports mathematical expressions, variable bindings, functions and the application of those functions, conditionals, return statements, and even advanced concepts like higher-order functions and closures. And then there are the different data types: integers, booleans, strings, arrays, and hashes."


### Features
- Integers, booleans, strings, arrays, hash maps
- A REPL
- Arithmetic expressions
- Let statements
- First-class and higher-order functions
- Built-in functions
- Recursion
- Closures
  
![repl](https://github.com/user-attachments/assets/14c52d1c-90d7-4501-9f0f-5bcb19050a3c)
## Why Rust?
After going through the Rust Book, I wanted a bigger project to apply what I’d learned. I also wanted to understand how programming languages work under the hood, so building this interpreter felt like the perfect way to explore both Rust and language design at the same time.

## Code Structure
The project is organized into four main modules: eval, lexer, parser, and repl, each responsible for different components of the interpreter. Since this was a Rust learning project, some design choices were made to try out various Rust features and explore how they work, rather than adhering strictly to best practices. This includes experimenting with ownership, pattern matching, and error handling to better understand Rust’s capabilities.

## Compiler and Virtual Machine (In Progress)
I'm also working on a stack-based compiler and virtual machine (VM) for Monkey. The VM currently supports most basic commands, including arithmetic operations and conditional jumps.  This work is ongoing in the compiler branch of the repository.


## Installation
1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/monkey-lang-rust.git
   cd monkey-lang-rust
   ```
2. Build and run the interpreter:
   ```bash
   cargo run
   ```

