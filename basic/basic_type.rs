fn main() {
    let _x: bool = true;

    // 在 Rust 中，char 类型不是单个字节而是由四个字节表示。
    let s = 'x';
    let s2 = "x";
    let s3 = "xy";
    println!("{} {} {}", s, s2, s3);

    let x = 42; // x has type i32
    let y = 1.0; // y has type f64

    /*
     * 可变大小类型
Rust 还提供了大小取决于底层机器指针的大小的类型。这些类型根据大小分为不同的类别，同样有有符号和无符号的类型。比如这两种类型：isize 和 usize。
     */

    // 定长数组
    let a = [1, 2, 3]; // a: [i32; 3]
    let mut m = [1, 2, 3]; // m: [i32; 3]

    // 长度为20的定长数组，将每个元素初始化成0
    let a = [0; 20]; // a: [i32; 20]

    println!("a has {} elements", a.len());

    // 切片
    // 切片有 &[T] 类型
    let a = [0, 1, 2, 3, 4];
    let middle = &a[1..4]; // A slice of a: just the elements 1, 2, and 3
    let complete = &a[..]; // A slice containing all of the elements in a

    // Rust 的 str 类型是最基本的字符串类型。作为一种无固定大小类型
    let s: &str = "HELLO";
    // 元组, 有固定大小
    let x = (1, "hello");
    let x: (i32, &str) = (1, "hello");

    (0,); // single-element tuple
    (0); // zero in parentheses

    // 元组索引
    let tuple = (1, 2, 3);

    let x = tuple.0;
    let y = tuple.1;
    let z = tuple.2;

    println!("x is {}", x);

    // 函数与函数指针
    fn foo(x: i32) -> i32 { x }
    let x: fn(i32) -> i32 = foo;
}
