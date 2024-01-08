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
struct Token<'a> {
    id: TokenId,
    itself: Option<&'a str>,
}

#[derive(Debug)]
struct VirtualMachine {
    stack: Vec<Value>,
    ip: usize,
    sp: usize,
}

impl VirtualMachine {
    fn new() -> Self {
        Self { stack: vec![], ip: 0, sp: 0 }
    }

    fn execute(&mut self, instructions: &Vec<Instruction>) {
        self.ip = 0;
        // println!("instructions.len(): {}", instructions.len());
        loop {
            if self.ip < instructions.len() {
                let ins = instructions[self.ip].clone();
                // println!("ins: {:#?}", ins);
                match ins {
                    Instruction::OpCode(OpCode::Add) => {
                        let b = self.stack[self.sp-1].clone();
                        let a = self.stack[self.sp-2].clone();
                        self.sp -= 1;
                        self.stack[self.sp] = a+b;
                        self.ip += 1;
                    },
                    Instruction::OpCode(OpCode::Sub) => {
                        let b = self.stack[self.sp-1].clone();
                        let a = self.stack[self.sp-2].clone();
                        self.sp -= 1;
                        self.stack[self.sp] = a-b;
                        self.ip += 1;
                    },
                    Instruction::OpCode(OpCode::Mul) => {
                        let b = self.stack[self.sp-1].clone();
                        let a = self.stack[self.sp-2].clone();
                        self.sp -= 1;
                        self.stack[self.sp] = a*b;
                        self.ip += 1;
                    },
                    Instruction::OpCode(OpCode::Div) => {
                        let b = self.stack[self.sp-1].clone();
                        let a = self.stack[self.sp-2].clone();
                        self.sp -= 1;
                        self.stack[self.sp] = a/b;
                        self.ip += 1;
                    },
                    Instruction::OpCode(OpCode::Push) => {
                        let a = instructions[self.ip+1].clone();
                        self.stack.push(a.get_value().expect("Expected a Value in the stack"));
                        self.sp += 1;
                        self.ip += 2;
                    },
                    Instruction::OpCode(OpCode::Print) => {
                        println!("[ {:?} ]", self.stack[self.sp-1]);
                        let _ = io::stdout().flush();
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
    }
}

fn codegen(tokens: &[Token]) -> Vec<Instruction> {
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
                    opcodes.push(Instruction::Value(Value::Int(tok.itself.unwrap().parse::<i32>().unwrap())));
                } else {
                    opcodes.push(Instruction::OpCode(OpCode::Push));
                    opcodes.push(Instruction::Value(Value::Int(tok.itself.unwrap().parse::<i32>().unwrap())));
                }
            }
            TokenId::Text => {
                match tok.itself {
                    Some("DUP") =>  { opcodes.push(Instruction::OpCode(OpCode::Dup)) },
                    Some("DROP") => { opcodes.push(Instruction::OpCode(OpCode::Drop)) },
                    Some("SWAP") => { opcodes.push(Instruction::OpCode(OpCode::Swap)) },
                    Some("OVER") => { opcodes.push(Instruction::OpCode(OpCode::Over)) },
                    Some("PRINT") => { opcodes.push(Instruction::OpCode(OpCode::Print)) },
                    Some(itself) => {
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
                    None => {},
                } 
            }
        }
    }

    opcodes
}

fn parse(command: &str) -> Vec<Token> {
    let mut tokens = vec![];
    for word in command.split_ascii_whitespace() {
        match word.parse::<i32>() {
            Ok(_) => tokens.push(Token { id : TokenId::Digit, itself: Some(word) }),
            Err(_) => {
                match word {
                    "+" => tokens.push(Token { id: TokenId::Plus, itself: None } ),
                    "-" => tokens.push(Token { id: TokenId::Minus, itself: None } ),
                    "*" => tokens.push(Token { id: TokenId::Star, itself: None } ),
                    "/" => tokens.push(Token { id: TokenId::Slash, itself: None } ),
                    ":" => tokens.push(Token { id: TokenId::Colon, itself: None } ),
                    ";" => tokens.push(Token { id: TokenId::Semicolon, itself: None } ),
                    "DUP"|"DROP"|"SWAP"|"OVER"|"PRINT" => tokens.push(Token { id: TokenId::Semicolon, itself: Some(word) } ),
                    _ => tokens.push(Token { id: TokenId::Text, itself: Some(&word[1..word.len()-1]) } ),
                }
            }
        }
    }
    tokens
}

fn main() -> io::Result<()> {

    let mut buffer = String::new();
    let mut vm = VirtualMachine::new();
    loop {
        print!("> ");
        let _ = io::stdout().flush();
        io::stdin().read_line(&mut buffer)?;
        let tokens = parse(&buffer);
        let code = codegen(&tokens);
        println!("tokens: {:?}", &tokens);
        println!("code: {:?}", &code);
        vm.execute(&code);
        println!("vm: {:?}", &vm);
    }
}
