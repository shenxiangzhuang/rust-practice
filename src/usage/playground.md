# Playground


## Default Code Block
A default playground:
```rust
println!("Hello, world!");
```

## Editable Code Block

An editable playground:
```rust, editable
fn main() {
    println!("Hello, world!");
}
```

## Visualize by aquascope
```aquascope,interpreter
#fn main() {
let mut s = String::from("hello ");`[]`
s.push_str("world");`[]`
#}
```

```aquascope,permissions,stepper
#fn main() {
let mut v: Vec<i32> = vec![1, 2, 3];
let num: &i32 = &v[2];
println!("Third element is {}", *num);
println!("Again, the third element is {}", *num);
v.push(4);
}
```