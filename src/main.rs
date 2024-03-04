use std::env;
use crate::nob::{Lexer, Op, OpKind, Ops};

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
    while ch != '0' {
        println!("{}", ch);
        match ch {
            '+' | '-' | '<' | '>' => {
                let mut count = 1;
                let mut s = lexer.lexer_next();
                while s == ch {
                    count += 1;
                    s = lexer.lexer_next();
                }
                instructions.push(Op::new(OpKind::OpInc, count));
                ch = s;
            }
            '.' => {
                instructions.push(Op::new(OpKind::OpOutPut, 1));
                ch = lexer.lexer_next();
            }
            ',' => {
                instructions.push(Op::new(OpKind::OpInput, 1));
                ch = lexer.lexer_next();
            }
            '[' => {
                instructions.push(Op::new(OpKind::OpJumpIfZero, 1));
                ch = lexer.lexer_next();
            }
            ']' => {
                instructions.push(Op::new(OpKind::OpJumpIfNonZero, 1));
                ch = lexer.lexer_next();
            }
            _ => {
                // ignore it
            }
        }
    }

    println!("instructions：{:?}", instructions);
}

fn process_op(instructions: &mut Vec<Op>, string: &String, ch: &char, op: Op) {
    let mut count = 0;
}
