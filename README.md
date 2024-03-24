# brainfuck_rust

本项目使用Rust语言为BF实现了一种JIT编译器(参考Tsoding版本)。
# 0.运行
进入当前项目目录，直接在控制台下运行:`cargo run`即可。
1. 本项目的指令路径为 `./src/instructions.txt`，可以自由更改文件中的指令内容。
2. 如果想通过控制台直接输入指令，需要加上`-- [指令内容]` 后缀，即`cargo run -- [指令内容]`，运行即可。
# 1.BF？
brainfuck，简称BF，皆在创建一种简单的、可以用最小的编译器来实现的、符合图灵完全思想的编程语言，这个语言由八种运算符构成：

<img width="612" alt="image" src="https://github.com/ddx533534/brainfuck_rust/assets/51685343/1bbb2f7f-c3c2-4fda-9a49-c44d20b435fb">

# 2.JIT?
即时编译（Just-in-time compilation），是一种执行计算机代码的方法，这种方法设计在程序执行过程中（在执行期）而不是在执行之前进行编译。
JIT结合了编译代码的速度与解释的灵活性。


