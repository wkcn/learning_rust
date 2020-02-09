fn main() {
    println!("Hello, world!");

    // [1, 3)
    for i in 1..3 {
        println!("{}", i);
    }

    // 如果调用next, 必须是mut变量
    let mut range = 0..3;
    loop {
        match range.next() {
            Some(x) => {
                println!("{}", x);
            },
            None => {break}
        }
    }

    let nums = vec![1, 2, 3];

    for num in &nums {
        println!("{}", num);
    }

    // 消费者
    let one_to_three = (1..4).collect::<Vec<_>>();
    for num in &one_to_three {
        println!("{}", num);
    }

    println!("Find Consumer");
    let greater_than_two = (1..4).find(|x| *x > 2);
    for num in &greater_than_two {
        println!("{}", num);
    }

    match greater_than_two {
        Some(_) => println!("We got some numbers!"),
        None => println!("No numbers found :("),
    }

    println!("Fold consumer");
    // fold(base, |accumulator, element| ...)
    let sum = (1..4).fold(0, |sum, x| sum + x);
    println!("sum: {}", sum);

    // Lazy
    let it = (1..10).map(|x| println!("lazy {}", x));
    for _ in it {
    }

    for i in (1..).step_by(5).take(5) {
        println!("{}", i);
    }

    for i in (1..10).filter(|&x| x % 2 == 0) {
        println!("{}", i);
    }
}
