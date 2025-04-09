# 🌙 Lunaria Compiler

**Lunaria** is a minimalist interpreted programming language that brings elegance, creativity, and simplicity together. With its intuitive syntax and expressive structure, Lunaria lets you build poetic, powerful programs that feel alive.

Inspired by VTuber Himemori Luna and built with love.

---

## ✨ Features

- 🌸 Poetic syntax using custom keywords
- 🧮 Arithmetic and expressions
- 🧵 String concatenation with `~`
- 📤 Output with `console.out(...)`
- 📁 File writing with `fs.out(...)`
- 🖥️ Platform-aware shell execution with `shell.run(...)`
- 🧠 Type system (`Text`, `Int`, etc.)
- 🖋️ Custom DSL for creativity and clarity

---

## 💡 Example

```lunaria
define name::Text := "Denji";
define greeting::Text := "Hello, ";
define message::Text := greeting ~ name ~ "!";

console.out(message);
