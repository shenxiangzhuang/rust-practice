# Cargo

## 在当前路径下创建项目
```bash
cargo init
```

> Create a new cargo package in an existing directory

<div class="warning">

注意这里不能用`cargo new .`: "error: destination ... already exists".

也就是说，`cargo new`只能**创建一个新的目录**，然后在新目录下初始化项目文件；
`cargo init`就是在当前目录下**初始化项目**。

</div>

