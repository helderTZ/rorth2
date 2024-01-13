use std::io;
use std::io::Write;
use std::ops::{Add, Sub, Mul, Div};

#[derive(Debug, Clone, Copy, PartialEq)]
enum OpCode {
    Dup,
    Drop,
    Swap,
    Over,
    Add,
    Sub,
    Mul,
    Div,
    BeginDefine,
    EndDefine,
    Push,
    Pop,
    PrintStack,
    PrintTop,
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

#[derive(Debug, Clone, PartialEq)]
enum Value {
    Int(i32),
    Str(String),
    Ins(Box<Instruction>),
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

impl Value {
    fn get_str(&self) -> String {
        match self {
            Value::Int(i) => String::from(i.to_string()),
            Value::Str(s) => String::from(s),
            Value::Ins(ins) => String::from(format!("{:?}", ins)),
        }
    }
    fn get_int(&self) -> i32 {
        match self {
            Value::Int(a) => *a,
            Value::Str(_) => 0,
            Value::Ins(_) => -1,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Instruction {
    opcode: OpCode,
    values: Vec<Value>,
}

#[derive(Debug, Clone)]
struct Token {
    id: TokenId,
    itself: Option<String>,
}

#[derive(Debug, Clone)]
struct CustomCommand {
    name: String,
    instructions: Vec<Instruction>,
}

#[derive(Debug, Clone)]
struct VirtualMachine {
    stack: Vec<Value>,
    custom_commands: Vec<CustomCommand>,
    ip: usize,
}

impl<'a> VirtualMachine {
    fn new() -> Self {
        Self { stack: vec![], custom_commands: vec![], ip: 0 }
    }

    fn stack(&self) -> &Vec<Value> {
        &self.stack
    }

    fn execute(&'a mut self, command: &'a str) -> bool {
        self.run(&self.codegen(&self.parse(command)))
    }

    fn run(&mut self, instructions: &Vec<Instruction>) -> bool {
        self.ip = 0;
        let mut should_exit = false;
        loop {
            if self.ip < instructions.len() {
                let ins = instructions[self.ip].clone();
                match ins.opcode {
                    OpCode::Add => {
                        let b = self.stack.pop().unwrap();
                        let a = self.stack.pop().unwrap();
                        self.stack.push(a+b);
                        self.ip += 1;
                    },
                    OpCode::Sub => {
                        let b = self.stack.pop().unwrap();
                        let a = self.stack.pop().unwrap();
                        self.stack.push(a-b);
                        self.ip += 1;
                    },
                    OpCode::Mul => {
                        let b = self.stack.pop().unwrap();
                        let a = self.stack.pop().unwrap();
                        self.stack.push(a*b);
                        self.ip += 1;
                    },
                    OpCode::Div => {
                        let b = self.stack.pop().unwrap();
                        let a = self.stack.pop().unwrap();
                        self.stack.push(a/b);
                        self.ip += 1;
                    },
                    OpCode::Dup => {
                        let a = self.stack.pop().unwrap();
                        self.stack.push(a.clone());
                        self.stack.push(a);
                        self.ip += 1;
                    },
                    OpCode::Drop => {
                        let _ = self.stack.pop();
                        self.ip += 1;
                    },
                    OpCode::Swap => {
                        let b = self.stack.pop().unwrap();
                        let a = self.stack.pop().unwrap();
                        self.stack.push(b);
                        self.stack.push(a);
                        self.ip += 1;
                    },
                    OpCode::Over => {
                        let b = self.stack.pop().unwrap();
                        let a = self.stack.pop().unwrap();
                        self.stack.push(a.clone());
                        self.stack.push(b);
                        self.stack.push(a);
                        self.ip += 1;
                    },
                    OpCode::BeginDefine => {
                        let name = ins.values[0].clone();
                        self.ip += 1;
                        let mut cmd = CustomCommand {
                            name: name.get_str(),
                            instructions: vec![],
                        };
                        for i in 1..ins.values.len() {
                            let ii = ins.values[i].clone();
                            match ii {
                                //FIXME: not working, the instructions are being pushed as Str
                                //       need to se if Str is referring to an instruction...
                                Value::Ins(iii) => cmd.instructions.push(*iii),
                                _ => panic!("Expected an Instruction"),
                            }
                        }
                        self.custom_commands.push(cmd); 
                        self.ip += ins.values.len()-1;
                    },
                    //TODO: this needs to check if the to be pushed values
                    //      are not custom words, if so, execute them
                    OpCode::Push => {
                        let val = ins.values[0].clone();
                        let mut is_cmd = false;
                        for cmd in self.custom_commands.iter() {
                            if cmd.name == val.get_str() {
                                is_cmd = true;
                                self.clone().run(&cmd.instructions);
                            }
                        }
                        if !is_cmd {
                            self.stack.push(val);
                        }
                        self.ip += 1;
                    },
                    OpCode::Pop => {
                        let _ = self.stack.pop();
                        self.ip += 1;
                    },
                    OpCode::PrintStack => {
                        if self.stack.is_empty() {
                            println!("{:?}", Value::Str(String::from("nil")));
                        } else {
                            println!("{:?}", self.stack);
                        }
                        let _ = io::stdout().flush();
                        self.ip += 1;
                    },
                    OpCode::PrintTop => {
                        if self.stack.is_empty() {
                            println!("{:?}", Value::Str(String::from("nil")));
                        } else {
                            println!("{:?}", self.stack[self.stack.len()-1]);
                        }
                        let _ = io::stdout().flush();
                        self.ip += 1;
                    },
                    OpCode::Exit => {
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
                TokenId::Plus =>  { opcodes.push(Instruction { opcode: OpCode::Add, values: vec![] }); }
                TokenId::Minus => { opcodes.push(Instruction { opcode: OpCode::Sub, values: vec![] }); }
                TokenId::Star =>  { opcodes.push(Instruction { opcode: OpCode::Mul, values: vec![] }); }
                TokenId::Slash => { opcodes.push(Instruction { opcode: OpCode::Div, values: vec![] }); }
                TokenId::Colon => {
                    declaration_mode = true;
                }
                TokenId::Semicolon => {
                    definition_mode = false;
                    opcodes.push(Instruction { opcode: OpCode::EndDefine, values: vec![] });
                }
                TokenId::Digit => {
                    opcodes.push(Instruction {
                        opcode: OpCode::Push,
                        values: vec![Value::Int(tok.itself.clone().unwrap().parse::<i32>().unwrap())],
                    });
                }
                TokenId::Text => {
                    match &tok.itself {
                        Some(a) => match a.as_str() {
                            "DUP" =>   { opcodes.push(Instruction { opcode: OpCode::Dup, values: vec![] }) },
                            "DROP" =>  { opcodes.push(Instruction { opcode: OpCode::Drop, values: vec![] }) },
                            "SWAP" =>  { opcodes.push(Instruction { opcode: OpCode::Swap, values: vec![] }) },
                            "OVER" =>  { opcodes.push(Instruction { opcode: OpCode::Over, values: vec![] }) },
                            "PRINT" => { opcodes.push(Instruction { opcode: OpCode::PrintStack, values: vec![] }) },
                            "POP" =>   { opcodes.push(Instruction { opcode: OpCode::Pop, values: vec![] }) },
                            "EXIT" =>  { opcodes.push(Instruction { opcode: OpCode::Exit, values: vec![] }) },
                            itself => {
                                if declaration_mode {
                                    opcodes.push(Instruction {
                                        opcode: OpCode::BeginDefine,
                                        values: vec![Value::Str(String::from(itself))],
                                    });
                                    declaration_mode = false;
                                    definition_mode = true;
                                } else if definition_mode {
                                    println!("=== def mode ===");
                                    let idx = opcodes.len()-1;
                                    opcodes[idx].values.push(Value::Str(String::from(itself)));
                                } else {
                                    opcodes.push(Instruction {
                                        opcode: OpCode::Push,
                                        values: vec![Value::Str(String::from(itself))],
                                    });
                                }
                            },
                        },
                        None => {},
                    } 
                }
            }
        }
        if opcodes.len() > 0 && opcodes[opcodes.len()-1].opcode != OpCode::PrintStack {
            opcodes.push(Instruction { opcode: OpCode::PrintTop, values: vec![] });
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
                        _ => {
                            if word.starts_with("\"") && word.ends_with("\"") {
                                tokens.push(Token { id: TokenId::Text, itself: Some(String::from(&word[1..word.len()-1])) } );
                            } else {
                                tokens.push(Token { id: TokenId::Text, itself: Some(String::from(word)) } );
                            }
                        }
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
        if vm.execute(&buffer) {
            break;
        }
        buffer.clear();
    }

    Ok(())
}
