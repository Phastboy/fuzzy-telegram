# Learn Rust
Packages are referred to as `crates`.
## Generating a new project
```
cargo new <project-name>
```

## Adding dependency
```
cargo add <dependency-name>
```

## Run a project
```
cargo run
```

## Generating executable
```
rustc <fileName> -o <executableName>
```

## Installing dependencies
```
cargo build
```

# Focus
## Macros
### Formatted print
- `format` write formatted text to string.
- `print!`: same as `format` but text is printed to the console(`io::stdout`).
- `println!`: same as `print!` but a new line is added.

## Variables
The let keyword is used to declare a variable.
Variables are immutable by default in rust.
For example, you can't change the value of `pi` in:
```
let pi=3.141592;
```

 To make it mutable, you have to specify `mut` as in:
```
let mut pi=3.141592;
```

## Data Types
``` mermaid
graph TD;
    DT --> s["`scalar
        ____________________________
        integers
        floating-point numbers
        booleans
        characters`"]
    DT --> c["`compound
        ____________________________
        arrays
        tuples`"]
```
