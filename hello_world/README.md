# Hello, Cargo!

通常代码放在`src`文件夹

`Cargo.toml`是配置文件，映射到哈希表

# 新建项目
`cargo new hello_world --bin`

我们传递了参数 --bin，因为我们正在创建一个二进制程序：如果我们创建的是一个库文件，就不要这个参数了。

# 编译
`cargo build`

# 运行
`cargo run`

可执行文件在`target/debug/`目录下, 如`target/debug/hello_world`

# 发布
`cargo build --release`
