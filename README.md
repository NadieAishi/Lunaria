# 🌙 Lunaria Compiler

**Lunaria** is a minimalist interpreted programming language that aims to bring elegance, creativity, and simplicity together. It features intuitive syntax, playful expressiveness, and the ability to write and compile programs in a poetic style. Inspired by VTuber Himemori Luna and built with love.

## ✨ Features

- Custom keyword-based syntax with poetic flavor
- Expression evaluation engine
- String concatenation via `~`
- Basic arithmetic support
- Console output via `console.out(...)`
- File output via `fs.out(...)`
- Shell command execution via `shell.run(...)`
- Platform-aware compilation support (Windows/Linux/Android/macOS)
- Basic type system (e.g., `Text`, `Int`, etc.)

## 💡 Example

```lunaria
define name::Text := "Denji";
define greeting::Text := "Hello, ";
define message::Text := greeting ~ name ~ "!";

console.out(message);
