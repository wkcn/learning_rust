# Hello, Cargo!

通常代码放在`src`文件夹

`Cargo.toml`是配置文件，映射到哈希表

# 新建项目
`cargo new hello_world`

默认是`--bin`, 可选`--lib`

# 编译
`cargo build`

# 运行
`cargo run`

可执行文件在`target/debug/`目录下, 如`target/debug/hello_world`

# 发布
`cargo build --release`
