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
```
---
## 🧠 Philosophy
Lunaria was created not to compete with existing languages, but to play, to express, and to experiment.
If you're here, it's probably because you enjoy building things that make you feel something — and that’s enough reason to explore Lunaria.
---
🤝 Contributing
We warmly welcome contributors!

### Contribution Guidelines
All improvements to Lunaria, whether bugfixes, new features, or ideas, must be open-sourced.

If you use Lunaria in a commercial product, you're required to share your modifications publicly under the same AGPL-3.0 license.

Patents based on Lunaria or forks are not allowed unless they are shared back under AGPL.

Pull requests, issues, and discussions are welcome.

Please be respectful, constructive, and collaborative.

## Want to help?
You can:
-Improve the interpreter or parser
-Port Lunaria to other platforms (Android? WebAssembly?)
- Add new language features (conditions, loops, etc.)
- Create tutorials or guides

Start by checking out the src/ directory and understanding how parser.rs and interpreter.rs work!

## 🏗️ Build Instructions
```
git clone https://github.com/NadieAishi/Lunaria
cd Lunaria
cargo run -- your_script.lna
```

## License
Lunaria is licensed under the **GNU Affero General Public License v3.0.**

You are free to:
- Use
- Modify
- Share

As long as:
- You share your modifications under the same license
- You do not attempt to privatize or patent its core

## 🧑‍🎤 Author
Created by PrettyJoke ✨
A playful rebellion in code, built under moonlight.

“So far from China, yet so close to the moon.”
