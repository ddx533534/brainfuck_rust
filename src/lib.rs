mod nob;

pub mod brain_fuck {
    use std::{env, fs};
    use std::fmt::{Display, Formatter};
    use crate::nob::{AddrStack, Lexer, Memory, Op, OpKind, Ops};

    // 两种方式启动程序，一种可以从控制台输入读取指令，另一种则从文件读取指令。具体取决于参数长度，详细请看readme.md。
    pub fn start() {
        let args: &Vec<String> = &env::args().collect();
        if args.len() > 1 {
            start_from_input();
        } else {
            let file_path = String::new();
            start_from_file(file_path);
        }
    }

    // 从输入读取指令。格式: cargo run -- [指令字符串]
    // 例如：cargo run -- "++>+++++[<+>-]++++++++[< +++ +++> -]< ."
    fn start_from_input() {
        let args: Vec<String> = env::args().collect();
        println!("程序名称: {}, len: {}", args[0], args.len());
        if args.len() <= 1 {
            panic!("no instruction content is provided!");
        }
        let str = String::from(args.last().unwrap());
        let mut instructions: Vec<Op> = Vec::new();
        process_instructions(str, &mut instructions);
        intercept_instructions(&instructions);
    }

    // 从文件读取指令。格式: cargo run
    // 默认指令文件路径为./src/instructions.txt
    fn start_from_file(file_path: String) {
        let mut instructions: Vec<Op> = Vec::new();
        let str = read_instructions(file_path);
        process_instructions(str, &mut instructions);
        intercept_instructions(&instructions);
    }

    // 1.读取指令。
    fn read_instructions(mut file_path: String) -> String {
        if file_path.is_empty() {
            file_path = String::from("./src/instructions.txt");
        }
        // 读取文件内容并处理可能的错误
        let result = fs::read_to_string(file_path);

        // 使用 match 表达式处理 Result
        match result {
            Ok(content) => {
                // 如果成功读取文件，则打印文件内容
                println!("File content:\n{}", content);
                content
            }
            Err(err) => {
                // 如果出现错误，则打印错误信息
                panic!("Error reading file: {}", err);
            }
        }
    }

    // 2.处理指令。将指令存入动态数组并简化，针对特殊指令 [ 和 ] ，利用栈维护地址信息
    fn process_instructions(ops_str: String, instructions: &mut Vec<Op>) {
        let mut lexer = Lexer::new(ops_str, 0);
        let mut ch = lexer.lexer_next();
        let mut addr_stack = AddrStack::default();
        while ch != '0' {
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
                    instructions.push(Op::new(OpKind::OpJumpIfZero, 1));
                    addr_stack.push_item(addr);
                    ch = lexer.lexer_next();
                }
                ']' => {
                    if addr_stack.is_empty() {
                        panic!("????");
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
        // let ops = DisplayOps {
        //     ops: &instructions
        // };
        // println!("instructions: \n{:}", ops);
    }

    // 3.解释指令。通过创建对应的内存数组，根据不同指令的作用，输出经过解释的值。
    fn intercept_instructions(instructions: &Vec<Op>) {
        let mut memory = Memory::default();
        let mut output = String::new();
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
                        panic!("runtime error!");
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
                    for _i in 0..op.operand {
                        let char = std::char::from_u32(memory.items[head] as u32).unwrap();
                        // println!("output:{}", char);
                        output.push(char);
                    }
                    ip += 1;
                }
                OpKind::OpInput => {
                    ip += 1;
                    // todo not implemented!
                }
                OpKind::OpJumpIfZero => {
                    // println!("ready for jump: {},{}", memory.items[head], op.operand);
                    if memory.items[head] == 0 {
                        ip = op.operand;
                    } else {
                        ip += 1;
                    }
                }
                OpKind::OpJumpIfNonZero => {
                    // back！！！
                    // println!("jump back: {},{}", memory.items[head], op.operand);
                    if memory.items[head] != 0 {
                        ip = op.operand;
                    } else {
                        ip += 1;
                    }
                }
            }
        }

        println!("memory is : {:?}", memory.items);
        println!("head is : {:?}", head);
        println!("output is : {}", output);
    }

    struct DisplayOps<'a> {
        ops: &'a Vec<Op>,
    }

    impl<'a> Display for DisplayOps<'a> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            // 遍历 Vec 中的元素并打印
            for (i, item) in self.ops.iter().enumerate() {
                write!(f, "{}:{:?}({:?}),\n", i, item.op_kind, item.operand)?;
            }
            write!(f, "\n")
        }
    }
}