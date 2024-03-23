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


impl OpKind {
    pub fn get_raw(str: &char) -> OpKind {
        match str {
            '+' => { OpKind::OpInc }
            '-' => { OpKind::OpDec }
            '>' => { OpKind::OpRight }
            '<' => { OpKind::OpLeft }
            '.' => { OpKind::OpOutPut }
            ',' => { OpKind::OpInput }
            '[' => { OpKind::OpJumpIfZero }
            ']' => { OpKind::OpJumpIfNonZero }
            _ => { OpKind::OpInc }
        }
    }
}

#[derive(Debug)]
pub struct Op {
    pub op_kind: OpKind,
    pub operand: usize,
}

impl Op {
    pub fn new(op_kind: OpKind, operand: usize) -> Op {
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

// back patch
pub struct AddrStack {
    pub items: Vec<usize>,
    pub count: usize,
    pub capacity: usize,
}

impl AddrStack {
    pub fn default() -> AddrStack {
        AddrStack {
            items: Vec::new(),
            count: 0,
            capacity: 1,
        }
    }

    pub fn push_item(&mut self, item: usize) {
        self.items.push(item);
        self.count += 1;
        if self.count > self.capacity {
            self.capacity = self.capacity * 4;
        }
    }
    pub fn pop_item(&mut self) -> usize {
        if !self.is_empty() {
            self.count -= 1;
            return self.items.pop().unwrap();
        }
        return 0;
    }

    pub fn is_empty(&self) -> bool {
        self.count == 0
    }
}

pub struct Memory {
    pub items: Vec<usize>,
    pub count: usize,
    pub capacity: usize,
}

impl Memory {
    pub fn default() -> Memory {
        Memory {
            items: Vec::new(),
            count: 0,
            capacity: 1,
        }
    }
}