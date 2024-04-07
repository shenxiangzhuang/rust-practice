# Question & Answer


## String Concat


## Type casting
如何在Rust中实现类型转换？
首先，Rust中没有隐式的类型转换，如下面的用法就会报错:

```rust
let x_usize: usize = 42;
let x_i64: i64 = x_usize;

println!("{}, {}", x_usize, x_i64);
```

但是我们可以通过`as`关键字来显式地进行类型转换[^1].
>A type cast expression is denoted with the binary operator `as`[^2].
```rust
let x_usize: usize = 42;
let x_i64: i64 = x_usize as i64;

println!("{}, {}", x_usize, x_i64);
```


[^1]: [https://doc.rust-lang.org/rust-by-example/types/cast.html](https://doc.rust-lang.org/rust-by-example/types/cast.html)
[^2]: [https://doc.rust-lang.org/reference/expressions/operator-expr.html#type-cast-expressions](https://doc.rust-lang.org/reference/expressions/operator-expr.html#type-cast-expressions)
