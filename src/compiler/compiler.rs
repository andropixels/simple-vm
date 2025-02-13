pub struct Compiler {
    bytecode: Vec<u8>,
    variables: HashMap<String, usize>,
    next_var_addr: usize,
}

impl Compiler {
    pub fn new() -> Self {
        Compiler {
            bytecode: Vec::new(),
            variables: HashMap::new(),
            next_var_addr: 0,
        }
    }

    fn emit(&mut self, opcode: u8) {
        self.bytecode.push(opcode);
    }

    fn emit_i64(&mut self, value: i64) {
        self.bytecode.extend_from_slice(&value.to_le_bytes());
    }

    fn get_var_address(&mut self, name: &str) -> usize {
        if let Some(&addr) = self.variables.get(name) {
            addr
        } else {
            let addr = self.next_var_addr;
            self.variables.insert(name.to_string(), addr);
            self.next_var_addr += 1;
            addr
        }
    }

    fn compile_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Number(n) => {
                self.emit(Opcode::Push as u8);
                self.emit_i64(*n);
            }
            Expr::Variable(name) => {
                let addr = self.get_var_address(name);
                self.emit(Opcode::Push as u8);
                self.emit_i64(addr as i64);
                self.emit(Opcode::Load as u8);
            }
            Expr::BinaryOp(left, op, right) => {
                self.compile_expr(left);
                self.compile_expr(right);
                match op {
                    BinaryOpKind::Add => self.emit(Opcode::Add as u8),
                    BinaryOpKind::Sub => self.emit(Opcode::Sub as u8),
                    BinaryOpKind::Mul => self.emit(Opcode::Mul as u8),
                    BinaryOpKind::Div => self.emit(Opcode::Div as u8),
                    BinaryOpKind::Equals => self.emit(Opcode::Equal as u8),
                    BinaryOpKind::LessThan => self.emit(Opcode::Less as u8),
                    BinaryOpKind::GreaterThan => {
                        // a > b is equivalent to b < a
                        let temp = self.bytecode.len() - 16; // Assuming each push takes 8 bytes
                        self.bytecode.swap(temp, temp + 8); // Swap the order of operands
                        self.emit(Opcode::Less as u8);
                    }
                }
            }
        }
    }

    pub fn compile(&mut self, statements: Vec<Statement>) -> Vec<u8> {
        for statement in statements {
            match statement {
                Statement::Let(name, expr) | Statement::Assign(name, expr) => {
                    let addr = self.get_var_address(&name);
                    self.compile_expr(&expr);
                    self.emit(Opcode::Push as u8);
                    self.emit_i64(addr as i64);
                    self.emit(Opcode::Store as u8);
                }
                Statement::If(condition, then_block, else_block) => {
                    self.compile_expr(&condition);
                    
                    // Placeholder for jump addresses
                    let jump_if_pos = self.bytecode.len();
                    self.emit(Opcode::JumpIf as u8);
                    self.emit_i64(0); // Placeholder for else block
                    
                    self.compile(then_block);
                    
                    let jump_end_pos = self.bytecode.len();
                    self.emit(Opcode::Jump as u8);
                    self.emit_i64(0); // Placeholder for end
                    
                    let else_pos = self.bytecode.len();
                    self.compile(else_block);
                    let end_pos = self.bytecode.len();
                    
                    // Fix up the jump addresses
                    let else_addr = else_pos as i64;
                    let end_addr = end_pos as i64;
                    self.bytecode[jump_if_pos+1..jump_if_pos+9].copy_from_slice(&else_addr.to_le_bytes());
                    self.bytecode[jump_end_pos+1..jump_end_pos+9].copy_from_slice(&end_addr.to_le_bytes());
                }
                Statement::While(condition, block) => {
                    let start_pos = self.bytecode.len();
                    
                    self.compile_expr(&condition);
                    
                    let jump_pos = self.bytecode.len();
                    self.emit(Opcode::JumpIf as u8);
                    self.emit_i64(0); // Placeholder for end
                    
                    self.compile(block);
                    
                    // Jump back to start
                    self.emit(Opcode::Push as u8);
                    self.emit_i64(start_pos as i64);
                    self.emit(Opcode::Jump as u8);
                    
                    let end_pos = self.bytecode.len();
                    let end_addr = end_pos as i64;
                    self.bytecode[jump_pos+1..jump_pos+9].copy_from_slice(&end_addr.to_le_bytes());
                }
                Statement::Print(expr) => {
                    self.compile_expr(&expr);
                    self.emit(Opcode::Print as u8);
                }
            }
        }
        
        self.emit(Opcode::Halt as u8);
        self.bytecode.clone()
    }
}