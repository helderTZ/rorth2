use std::io;
use std::io::Write;
use std::ops::{Add, Sub, Mul, Div};

#[derive(Debug, Clone, Copy)]
enum OpCode {
    Dup,
    Drop,
    Swap,
    Over,
    Add,
    Sub,
    Mul,
    Div,
    Define,
    Assign,
    Push,
    Pop,
    Print,
    Exit,
}

#[derive(Debug, Clone, Copy)]
enum TokenId {
    Plus,
    Minus,
    Star,
    Slash,
    Text,
    Digit,
    Colon,
    Semicolon,
}

#[derive(Debug, Clone)]
enum Value {
    Int(i32),
    Str(String),
}

impl Add for Value {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        match (self, other) {
            (Value::Int(this), Value::Int(other)) => Value::Int(this + other),
            _ => { unimplemented!() },
        }
    }
}

impl Sub for Value {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        match (self, other) {
            (Value::Int(this), Value::Int(other)) => Value::Int(this - other),
            _ => { unimplemented!() },
        }
    }
}

impl Mul for Value {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        match (self, other) {
            (Value::Int(this), Value::Int(other)) => Value::Int(this * other),
            _ => { unimplemented!() },
        }
    }
}

impl Div for Value {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        match (self, other) {
            (Value::Int(this), Value::Int(other)) => Value::Int(this / other),
            _ => { unimplemented!() },
        }
    }
}

#[derive(Debug, Clone)]
enum Instruction {
    OpCode(OpCode),
    Value(Value),
}

impl Instruction {
    fn get_value(&self) -> Option<Value> {
        match self {
            Instruction::Value(a) => Some(a.clone()),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
struct Token {
    id: TokenId,
    itself: Option<String>,
}

#[derive(Debug)]
struct VirtualMachine {
    stack: Vec<Value>,
    ip: usize,
}

impl<'a> VirtualMachine {
    fn new() -> Self {
        Self { stack: vec![], ip: 0 }
    }

    fn stack(&self) -> &Vec<Value> {
        &self.stack
    }

    fn exec(&'a mut self, command: &'a str) -> bool {
        self.execute(&self.codegen(&self.parse(command)))
    }

    fn execute(&mut self, instructions: &Vec<Instruction>) -> bool {
        self.ip = 0;
        let mut should_exit = false;
        // println!("instructions.len(): {}", instructions.len());
        loop {
            if self.ip < instructions.len() {
                let ins = instructions[self.ip].clone();
                // println!("ins: {:#?}", ins);
                match ins {
                    Instruction::OpCode(OpCode::Add) => {
                        let b = self.stack.pop().unwrap();
                        let a = self.stack.pop().unwrap();
                        self.stack.push(a+b);
                        self.ip += 1;
                    },
                    Instruction::OpCode(OpCode::Sub) => {
                        let b = self.stack.pop().unwrap();
                        let a = self.stack.pop().unwrap();
                        self.stack.push(a-b);
                        self.ip += 1;
                    },
                    Instruction::OpCode(OpCode::Mul) => {
                        let b = self.stack.pop().unwrap();
                        let a = self.stack.pop().unwrap();
                        self.stack.push(a*b);
                        self.ip += 1;
                    },
                    Instruction::OpCode(OpCode::Div) => {
                        let b = self.stack.pop().unwrap();
                        let a = self.stack.pop().unwrap();
                        self.stack.push(a/b);
                        self.ip += 1;
                    },
                    Instruction::OpCode(OpCode::Dup) => {
                        let a = self.stack.pop().unwrap();
                        self.stack.push(a.clone());
                        self.stack.push(a);
                        self.ip += 1;
                    },
                    Instruction::OpCode(OpCode::Drop) => {
                        let _ = self.stack.pop();
                        self.ip += 1;
                    },
                    Instruction::OpCode(OpCode::Swap) => {
                        let b = self.stack.pop().unwrap();
                        let a = self.stack.pop().unwrap();
                        self.stack.push(b);
                        self.stack.push(a);
                        self.ip += 1;
                    },
                    Instruction::OpCode(OpCode::Over) => {
                        let b = self.stack.pop().unwrap();
                        let a = self.stack.pop().unwrap();
                        self.stack.push(a.clone());
                        self.stack.push(b);
                        self.stack.push(a);
                        self.ip += 1;
                    },
                    Instruction::OpCode(OpCode::Push) => {
                        let a = instructions[self.ip+1].clone();
                        self.stack.push(a.get_value().expect("Expected a Value in the stack"));
                        self.ip += 2;
                    },
                    Instruction::OpCode(OpCode::Pop) => {
                        let _ = self.stack.pop();
                        self.ip += 1;
                    },
                    Instruction::OpCode(OpCode::Print) => {
                        println!("{:?}", self.stack);
                        let _ = io::stdout().flush();
                        self.ip += 1;
                    },
                    Instruction::OpCode(OpCode::Exit) => {
                        should_exit = true;
                        self.ip += 1;
                    },
                    _ => {
                        unimplemented!();
                    }
                }
            } else {
                break;
            }
        }
        should_exit
    }

    fn codegen(&self, tokens: &[Token]) -> Vec<Instruction> {
        let mut opcodes: Vec<Instruction> = vec![];
        let mut declaration_mode = false;
        let mut definition_mode = false;
        for tok in tokens.iter() {
            match tok.id {
                TokenId::Plus =>  { opcodes.push(Instruction::OpCode(OpCode::Add)); }
                TokenId::Minus => { opcodes.push(Instruction::OpCode(OpCode::Sub)); }
                TokenId::Star =>  { opcodes.push(Instruction::OpCode(OpCode::Mul)); }
                TokenId::Slash => { opcodes.push(Instruction::OpCode(OpCode::Div)); }
                TokenId::Colon => { declaration_mode = true; }
                TokenId::Semicolon => {}
                TokenId::Digit => {
                    if definition_mode {
                        opcodes.push(Instruction::OpCode(OpCode::Assign));
                        opcodes.push(Instruction::Value(Value::Int(tok.itself.as_ref().unwrap().parse::<i32>().unwrap())));
                    } else {
                        opcodes.push(Instruction::OpCode(OpCode::Push));
                        opcodes.push(Instruction::Value(Value::Int(tok.itself.as_ref().unwrap().parse::<i32>().unwrap())));
                    }
                }
                TokenId::Text => {
                    match &tok.itself {
                        Some(a) => match a.as_str() {
                            "DUP" =>  { opcodes.push(Instruction::OpCode(OpCode::Dup)) },
                            "DROP" => { opcodes.push(Instruction::OpCode(OpCode::Drop)) },
                            "SWAP" => { opcodes.push(Instruction::OpCode(OpCode::Swap)) },
                            "OVER" => { opcodes.push(Instruction::OpCode(OpCode::Over)) },
                            "PRINT" => { opcodes.push(Instruction::OpCode(OpCode::Print)) },
                            "POP" => { opcodes.push(Instruction::OpCode(OpCode::Pop)) },
                            "EXIT" => { opcodes.push(Instruction::OpCode(OpCode::Exit)) },
                            itself => {
                                if declaration_mode {
                                    opcodes.push(Instruction::OpCode(OpCode::Define));
                                    opcodes.push(Instruction::Value(Value::Str(String::from(itself))));
                                    declaration_mode = false;
                                    definition_mode = true;
                                } else if definition_mode {
                                    opcodes.push(Instruction::OpCode(OpCode::Assign));
                                    opcodes.push(Instruction::Value(Value::Str(String::from(itself))));
                                    definition_mode = false;
                                } else {
                                    opcodes.push(Instruction::OpCode(OpCode::Push));
                                    opcodes.push(Instruction::Value(Value::Str(String::from(itself))));
                                }
                            },
                        },
                        None => {},
                    } 
                }
            }
        }

        opcodes
    }

    fn parse(&'a self, command: &'a str) -> Vec<Token> {
        let mut tokens = vec![];
        let words = command.split_ascii_whitespace().collect::<Vec<&str>>();

        for i in 0..words.len() {
            let word = words[i];

            if word.starts_with("\"") {
                if word.ends_with("\"") {
                    tokens.push(Token { id: TokenId::Text, itself: Some(String::from(word)) } );
                } else {
                    let mut user_string = String::from(word);
                    let mut j = 1;
                    let mut next_word = words[i+j];
                    while !next_word.ends_with("\"") {
                        user_string.push_str(" ");
                        user_string.push_str(next_word);
                        j += 1;
                        next_word = words[i+j];
                    }
                    user_string.push_str(" ");
                    user_string.push_str(next_word);
                    tokens.push(Token { id: TokenId::Text, itself: Some(user_string) } );
                }
                continue;
            }

            if word.ends_with("\"") {
                continue;
            }

            match word.parse::<i32>() {
                Ok(_) => tokens.push(Token { id : TokenId::Digit, itself: Some(String::from(word)) }),
                Err(_) => {
                    match word {
                        "+" => tokens.push(Token { id: TokenId::Plus, itself: None } ),
                        "-" => tokens.push(Token { id: TokenId::Minus, itself: None } ),
                        "*" => tokens.push(Token { id: TokenId::Star, itself: None } ),
                        "/" => tokens.push(Token { id: TokenId::Slash, itself: None } ),
                        ":" => tokens.push(Token { id: TokenId::Colon, itself: None } ),
                        ";" => tokens.push(Token { id: TokenId::Semicolon, itself: None } ),
                        "DUP"|"DROP"|"SWAP"|"OVER"|"PRINT"|"POP"|"EXIT" => tokens.push(Token { id: TokenId::Text, itself: Some(String::from(word)) } ),
                        _ => tokens.push(Token { id: TokenId::Text, itself: Some(String::from(&word[1..word.len()-1])) } ),
                    }
                }
            }
        }
        tokens
    }
}

fn main() -> io::Result<()> {

    let mut buffer = String::new();
    let mut vm = VirtualMachine::new();
    loop {
        print!("> ");
        let _ = io::stdout().flush();
        io::stdin().read_line(&mut buffer)?;
        buffer = buffer.to_uppercase();
        if vm.exec(&buffer) {
            break;
        }
        buffer.clear();
        println!("{:?}", vm.stack());
    }

    Ok(())
}
