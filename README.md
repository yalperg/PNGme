# PNGme
PNGme is a command-line tool that allows you to hide secret messages inside PNG image files without changing how they look.

This project is an implementation of the ["PNGme: An Intermediate Rust Project"](https://jrdngr.github.io/pngme_book/introduction.html) tutorial, made for learning purposes.

## What does this tool do?
This program can:
- Hide text messages inside PNG files
- Extract hidden messages from PNG files  
- Remove hidden messages from PNG files
- Show all data chunks in a PNG file

The hidden messages are stored in the PNG file's metadata chunks, so the image looks exactly the same but contains your secret data.

## Installation
1. Make sure you have Rust installed: https://rustup.rs/

2. Clone and build the project:
```bash
git clone https://github.com/yalperg/PNGme.git
cd PNGme
cargo build --release
```

## Commands

### encode - Hide a message

Hide a secret message in a PNG file:

```bash
cargo run -- encode <PNG_FILE> <CHUNK_TYPE> "<MESSAGE>"
```

Example:
```bash
cargo run -- encode photo.png "ruSt" "This is my secret message"
```

You can also save to a different file:
```bash
cargo run -- encode photo.png "ruSt" "Secret text" --output-file new_photo.png
```

### decode - Extract a message

Get a hidden message from a PNG file:

```bash
cargo run -- decode <PNG_FILE> <CHUNK_TYPE>
```

Example:
```bash
cargo run -- decode photo.png "ruSt"
```

### remove - Delete a message

Remove a hidden message from a PNG file:

```bash
cargo run -- remove <PNG_FILE> <CHUNK_TYPE>
```

Example:
```bash
cargo run -- remove photo.png "ruSt"
```

### print - Show file information

Display all chunks in a PNG file:

```bash
cargo run -- print <PNG_FILE>
```

Example:
```bash
cargo run -- print photo.png
```
