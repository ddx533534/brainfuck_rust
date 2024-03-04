// 操作符类型
#[derive(Debug)]
pub enum OpKind {
    // + 加
    OpInc,
    // - 减
    OpDec,
    // < 左移
    OpLeft,
    // > 右移
    OpRight,
    // . 输出
    OpOutPut,
    // , 输入
    OpInput,
    // [ 若为零则跳转
    OpJumpIfZero,
    // ] 不为零则跳转
    OpJumpIfNonZero,
}

#[derive(Debug)]
pub struct Op {
    op_kind: OpKind,
    operand: i32,
}

impl Op {
    pub fn new(op_kind: OpKind, operand: i32) -> Op {
        Op {
            op_kind,
            operand,
        }
    }
}

#[derive(Debug)]
pub struct Ops {
    items: Vec<Op>,
    count: i32,
    capacity: i32,
}

impl Ops {
    pub fn default() -> Ops {
        return Ops {
            items: Vec::new(),
            count: 0,
            capacity: 0,
        };
    }
}

pub struct Lexer {
    content: Vec<char>,
    pos: usize,
}

impl Lexer {
    pub fn new(string: String, pos: usize) -> Lexer {
        let mut content = Vec::new();
        for ch in string.chars() {
            content.push(ch);
        }
        return Lexer {
            content,
            pos,
        };
    }

    pub fn is_legal_cmd(&self, ch: char) -> bool {
        const CH: &str = "+-<>,.[]";
        return CH.contains(ch);
    }

    pub fn lexer_next(&mut self) -> char {
        // 检查是否是合法指令
        while self.pos < self.content.len() && !self.is_legal_cmd(self.content[self.pos]) {
            self.pos += 1;
        }
        if self.pos >= self.content.len() {
            return '0';
        }
        let ch = self.content[self.pos];
        self.pos += 1;
        return ch;
    }


    // pub fn lexer_illegal_next() -> char {
    //     // assert!(0 & "todo implement");
    //     return '1';
    // }
    //
    // pub fn lexer_has_next(&self) -> bool {
    //     return self.pos < self.content.len()
    // }
}