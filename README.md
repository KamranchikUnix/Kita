# Kita Programming Language

![License](https://img.shields.io/github/license/KamranchikUnix/Kita)
![Build Status](https://github.com/KamranchikUnix/Kita/actions/workflows/rust.yml/badge.svg)

**Kita is a new, lightweight, and high-performance systems programming language with a clean, humane syntax inspired by Lua.**

It is designed for developers who want the power and raw performance of C but with the readability and simplicity of a scripting language. Kita achieves this by transpiling its simple syntax into highly optimized C code, giving you the best of both worlds.

This project is in its early stages of development.

## Features (v1.0)

*   **Lua-inspired Syntax:** `local`, `if/then/else/end`.
*   **Statically Typed:** With type inference for simplicity.
*   **AOT Compiled:** Transpiles to C for maximum performance and portability.
*   **Cross-Platform:** The compiler driver intelligently uses `cl.exe` on Windows and `gcc`/`clang` on Linux/macOS.
*   **Tiny Binaries:** No garbage collector, no heavy runtime.

## How to Build the Compiler

1.  **Prerequisites:**
    *   The Rust toolchain ([rust-lang.org](https://rust-lang.org/))
    *   A C compiler (Visual Studio Build Tools on Windows, GCC on Linux, Clang on macOS).

2.  **Clone the repository:**
    ```bash
    git clone https://github.com/KamranchikUnix/Kita.git
    cd Kita
    ```

3.  **Build the compiler:**
    ```bash
    cargo build --release
    ```
    The final executable will be at `target/release/kita.exe` (or `kita` on Linux/macOS).
