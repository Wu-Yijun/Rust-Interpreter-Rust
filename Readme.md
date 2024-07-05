# Rust 编写的 Rust 解释器

- [ ] [English Version](./Readme-en.md)

**!!未完成!!**

## *注意: 此解释器不能完全覆盖 Rust 特性*

## 使用方法

可以类似 Python 的使用, 控制台输入 Rusty 进入 Rust Interpreter 环境.

## 待完成的特性

```Rust
/// declearation
let var_d;
let var_d : type;
let mut var_d;
let mut var_d : type;
/// psudo initial
let var_p = Literal;
let mut var_p = Literal;
/// initial
let var_i = Defined Expression;
let mut var_i = Defined Expression;
let var_i : type = Defined Expression;
let mut var_i : type = Defined Expression;
let var_i : type = Undefined Expression;
let mut var_i : type = Undefined Expression;

/// consts and static
const VAR_C : type = Const Expression;
static VAR_S : type = Const Expression;
static mut VAR_M : type = Const Expression;

/// function and closure
/// function can capture VAR_C, VAR_S, VAR_M
/// closure can capture VAR_C, VAR_S, VAR_M,
///     borrow var_i
/// closure can impl FnOnce, FnMut, Fn,
let var_f = let var_f : type = Closure;
let var_f : type = Closure;
fn fun(args: type) Expression
fn fun(args: type) -> type Expression

/// type can be
/// basic
i32, u32, ... 
/// struct
Struct1, ...

/// structs and traits
trait Trait {}
````