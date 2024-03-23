use std::{env, fmt};
use std::fmt::{Display, Formatter};
use crate::nob::{AddrStack, Lexer, Memory, Op, OpKind, Ops};

mod nob;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("程序名称: {}, len: {}", args[0], args.len());
    if args.len() > 1 {
        for arg in &args[1..] {
            println!("{}", arg);
        }
    } else {
        println!("no input is provided");
    }
    start(String::from(args.last().unwrap()));
}

//++++++++++[>+++++++>++++++++++>+++>+<<<<-]
// >++.>+.+++++++..+++.>++.<<+++++++++++++++.
// >.+++.------.--------.>+.>.
fn start(args: String) {
    let mut ops = Ops::default();
    process_input(args);
}

// 处理输入指令
fn process_input(string: String) {
    let mut instructions: Vec<Op> = Vec::new();
    let mut lexer = Lexer::new(string, 0);
    let mut ch = lexer.lexer_next();
    let mut addr_stack = AddrStack::default();
    while ch != '0' {
        println!("{}", ch);
        match ch {
            '+' | '-' | '<' | '>' | '.' | ',' => {
                let mut count = 1;
                let mut s = lexer.lexer_next();
                while s == ch {
                    count += 1;
                    s = lexer.lexer_next();
                }
                instructions.push(Op::new(OpKind::get_raw(&ch), count));
                ch = s;
            }
            // back patch
            '[' => {
                // loop start position
                let addr = instructions.len();
                println!("encounter [ ! insructions len: {} ", addr);
                instructions.push(Op::new(OpKind::OpJumpIfZero, 1));
                addr_stack.push_item(addr);
                ch = lexer.lexer_next();
            }
            ']' => {
                if addr_stack.is_empty() {
                    println!("????");
                    return;
                }
                // loop start position
                let addr = addr_stack.pop_item();
                instructions.push(Op::new(OpKind::OpJumpIfNonZero, addr + 1));
                instructions[addr].operand = instructions.len();
                ch = lexer.lexer_next();
            }
            _ => {
                // ignore it
            }
        }
    }

    let mut memory = Memory::default();
    memory.items.push(0);
    let mut head: usize = 0;
    let mut ip: usize = 0;
    while ip < instructions.len() {
        let op = &instructions[ip];
        match op.op_kind {
            OpKind::OpInc => {
                memory.items[head] += op.operand;
                ip += 1;
            }
            OpKind::OpDec => {
                memory.items[head] -= op.operand;
                ip += 1;
            }
            OpKind::OpLeft => {
                if head < op.operand {
                    println!("runtime error!");
                    return;
                }
                head -= op.operand;
                ip += 1;
            }
            OpKind::OpRight => {
                head += op.operand;
                while head >= memory.items.len() {
                    memory.items.push(0);
                }
                ip += 1;
            }
            OpKind::OpOutPut => {
                ip += 1;
                println!("output:{}", memory.items[head] as u8 as char);
            }
            OpKind::OpInput => {
                // todo not implemented!
                ip += 1;
            }
            OpKind::OpJumpIfZero => {
                println!("OpJumpIfZero: {},{}",memory.items[head],op.operand);
                if memory.items[head] == 0 {
                    ip = op.operand;
                } else {
                    ip += 1;
                }
            }
            OpKind::OpJumpIfNonZero => {
                // back！！！
                println!("OpJumpIfNonZero: {},{}",memory.items[head],op.operand);
                if memory.items[head] != 0 {
                    ip = op.operand;
                } else {
                    ip += 1;
                }
            }
        }
    }

    let custom = CustomStruct {
        ops: instructions
    };
    println!("instructions: \n{:}", custom);
    println!("memory: \n{:?}", memory.items);

}

fn process_op(instructions: &mut Vec<Op>, string: &String, ch: &char, op: Op) {
    let mut count = 0;
}

struct CustomStruct {
    ops: Vec<Op>,
}

impl Display for CustomStruct {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // 遍历 Vec 中的元素并打印
        for (i, item) in self.ops.iter().enumerate() {
            write!(f, "{}:{:?}({:?}),\n", i, item.op_kind, item.operand)?;
        }
        write!(f, "\n")
    }
}