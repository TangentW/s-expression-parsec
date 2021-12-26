# Parser Combinator ❤️ S-expression 

![repl](https://github.com/TangentW/s-expression-parsec/blob/7df81a2085a77973cfbe5837cfdaf438e353d355/img/repl.gif)

基于 `Parser Combinator` 的 `S-expression` 解释器。

## 简介

本项目实现了一套实用的 `Parser Combinator`（解析器组合子）以及一个遵循对应语法的 `S-expression`（S-表达式）解释器，解释器的解析部分基于前面的 `Parser Combinator`，表达式的语法规则将在下面详述。

这是作为我练手 `Rust` 和`函数式编程`的项目，旨在探索 `Rust` 在函数式编程领域的强大能力。

## 结构

项目分为以下三个主要构成：

### REPL 

位于 `repl` 目录，`S-expression` 的 `REPL`（交互式解释器）程序，运行过程可以在其中输入表达式进行解释求值，并打印出求值结果。上方的 GIF 图即为 `REPL` 的录屏。

### Parser Combinator

位于 `parsec` 目录，用 Rust 实现的具有实用性的解析器组合子，参考自 `Haskell` 的 `Parsec` 库。使用 `Parser Combinator`，我们可以快乐地用简洁易读的代码编写解析逻辑~

### S-expression

位于 `s-expression` 目录，`S-表达式`的解释器，解析部分基于 `Parser Combinator`。它的语法类似 `Lisp`。

语法规则来源于网上一个相传的 “微软面试题“ 😀，以下包含语法描述的引用：

> CSDN: https://blog.csdn.net/wdlsjdl2/article/details/53980438

> 牛客网: https://www.nowcoder.com/questionTerminal/7e6d2dd2e8774db3877f1fce2dd73834

语法内容如下：

> S-expression is a prefix notation invented for and popularized by the programming language Lisp. Your task is to evaluate some simple S-expressions.
> In this problem, S-expression is defined as:
> * An atom.
>   * a. A nonnegative integer. Its value is the value of the integer.
>   * b. A boolean value, true or false. Its value is the boolean value of itself.
>   * c. A variable, consisting of no more than 10 lower case letters, excluding reserved words "if", "let", "true" and "false". Its value is the value bound to the variable. If the varible is not bound yet, produce an error "Unbound Identifier". (See below for details)
> * ( f x ... )
>   * a. one of the following 4 forms: ( + x y ) ( - x y ) ( * x y ) ( / x y ) in which x and y are both S-expressions.
> To evaluate the S-expression, you need to first evaluate x then evaluate y.
> If the value of x or y is not an integer, produce an error "Type Mismatch".
> If both x and y are integers, its value is the their sum(x+y)/difference(x-y)/product(x*y)/quotient(x/y). The division is the same as the integer division ("/" operator) in C/C++/Java/C#, truncated division. If the value of y is zero, produce an error: "Division By Zero".
>   * b. ( if a b c ) in which a, b and c are S-expressions.
> To evaluate the S-expression, you need to evaluate a at first.
> If the value of a is not a boolean value, produce an error: "Type Mismatch".
> If the value of a is true, evaluate b and the S-expression's value is the value of b.
> If the value of a is false, evaluate c and the S-expression's value is the value of c.
> Note that b or c may not be evaluated during the calculation.
>   * c. ( let ( x a ) b ) in which x is a variable consisting of no more than 10 lower case letters, excluding reserved words "if", "let", "true" and "false", a and b are S-expressions.
> To evaluate the S-expression, you need to first evaluate a.
> Then bind the value of a to the variable x and evaluate b. Binding means if variable x appears in b, its value is the value of a. The S-expression's value is the value of b.
> Note that if x is bound to another value in b, the outer binding is ineffective. For example the value of ( let ( x 1 ) ( let ( x 2 ) x ) ) is 2.
>   * d. one of the following 3 forms: ( < x y ) ( > x y ) ( = x y ) in which x and y are S-expressions.
> To evaluate the S-expression, you need to first evaluate x then evaluate y.
> If the value of x or y is not an integer, produce an error "Type Mismatch".
> If both x and y are integers, its value is a boolean value indicating whether x < y, x > y or x = y is true.
> Given an S-expression, output its value. If an error occurs stop the evaluation and output the error.

引入此 crate，我们可以调用解释函数对相应的表达式进行解释：

```Rust
use s_expression;

let res = s_expression::run("(let (x 2) (y (+ 2 3)) (* x y))");
println!("{:?}", res);
```

输出：

``
Ok(Int(10))
``

