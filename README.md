# simple-bytecode-vm

A bytecode virtual machine and compiler written in Rust.

## Install





```
cargo add simple-vm
```





## Example 
```
let x = 5;
while x > 0 {
    print x;
    x = x - 1;
}
```

## Features

- Basic arithmetic (add, subtract, multiply, divide)
- Variables
- While loops
- If/else statements
- Print statements

## How it works
The VM takes your code, breaks it into simple instructions using a compiler, and runs them one by one. Like when you write x = x + 1, it becomes:

Load x
Add 1
Save back to x


## Usage

```
use simple_vm::{VM, compiler::{Parser, Compiler}};

let code = "let x = 5; print x;";
let bytecode = Compiler::new().compile(Parser::new(code).parse()?);
VM::new(bytecode, 1024).run()?;
```

### License
MIT
