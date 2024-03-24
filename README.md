# brainfuck_rust

本项目使用Rust语言为BF实现了一种JIT编译器(参考Tsoding版本)。
# 0.运行
进入当前项目目录，直接在控制台下运行:`cargo run`即可。
1. 本项目的指令路径为 `./src/instructions.txt`，可以自由更改文件中的指令内容。
2. 如果想通过控制台直接输入指令，需要加上`-- [指令内容]` 后缀，即`cargo run -- [指令内容]`，运行即可。

例如对于指令 `++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.`
我们将其拆解为`++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]` 和 `>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.`

对于前者：
```
++++++++                Set Cell #0 to 8
[
>++++               Add 4 to Cell #1; this will always set Cell #1 to 4
[                   as the cell will be cleared by the loop
>++             Add 2 to Cell #2
>+++            Add 3 to Cell #3
>+++            Add 3 to Cell #4
>+              Add 1 to Cell #5
<<<<-           Decrement the loop counter in Cell #1
]                   Loop until Cell #1 is zero; number of iterations is 4
>+                  Add 1 to Cell #2
>+                  Add 1 to Cell #3
>-                  Subtract 1 from Cell #4
>>+                 Add 1 to Cell #6
[<]                 Move back to the first zero cell you find; this will
be Cell #1 which was cleared by the previous loop
<-                  Decrement the loop Counter in Cell #0
]                       Loop until Cell #0 is zero; number of iterations is 8

The result of this is:
Cell no :   0   1   2   3   4   5   6
Contents:   0   0  72 104  88  32   8
Pointer :   ^
```
对于后者：
```
>>.                     Cell #2 has value 72 which is 'H'
>---.                   Subtract 3 from Cell #3 to get 101 which is 'e'
+++++++..+++.           Likewise for 'llo' from Cell #3
>>.                     Cell #5 is 32 for the space
<-.                     Subtract 1 from Cell #4 for 87 to give a 'W'
<.                      Cell #3 was set to 'o' from the end of 'Hello'
+++.------.--------.    Cell #3 for 'rl' and 'd'
>>+.                    Add 1 to Cell #5 gives us an exclamation point
>++.                    And finally a newline from Cell #6
```

最终控制台输出 `hello world!`

# 1.BF？
brainfuck，简称BF，皆在创建一种简单的、可以用最小的编译器来实现的、符合图灵完全思想的编程语言，这个语言由八种运算符构成：

<img width="612" alt="image" src="https://github.com/ddx533534/brainfuck_rust/assets/51685343/1bbb2f7f-c3c2-4fda-9a49-c44d20b435fb">

# 2.JIT?
即时编译（Just-in-time compilation），是一种执行计算机代码的方法，这种方法设计在程序执行过程中（在执行期）而不是在执行之前进行编译。
JIT结合了编译代码的速度与解释的灵活性。


