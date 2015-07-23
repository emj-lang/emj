# Emj Standard Library

The Emj standard library is based on the Lua standard library.

## Base

- 🔀📄 = Convert the arguments to strings.

## I/O

- 💬 = Print a line to stdout. Arguments are separated with tabs, and converted to string with the global 🔀📄 function.
- 📁 = IO module
    - 📂 = Open a file
    - 📛 = Rename a file
    - 📥 = Input. A function that sets a new stdin, or returns the current stdin.
    - 📤 = Output. A function that sets a new stdout, or returns the current stdout.
    - ⚠️ = stderr

## String

- 📄 = String module
    - 🔍 = Find (see [emj_patterns.md](emj_patterns.md))
    - 🔎 = Match (I'm sorry, everyone with dyslexia - I'll change this if I can find better emoji) (see [emj_patterns.md](emj_patterns.md))
