fn foo() -> i32 {
    10
    // 10或者return 10;
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn print_num(n: i32) {
    println!("{}", n);
}

fn main() {
    println!("{}", foo());
    print_num(39);
    print_num(add(3, 9));

    // Rust 是一门主要基于表达式的语言。它只有两种类型的语句，其他的都是表达式。
    // Rust 中有两种类型的语句：“声明语句”和“表达式语句”
    
    let mut y = 5;
    let x = (y = 6); //x has the value '()', not '6'
}
