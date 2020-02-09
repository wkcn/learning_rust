extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Hello, world!");

    println!("Guess the number!");
    // [1, 101)
    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("Please input your guess.");

    loop {
        // guess是可变的
        // 这个guess必须在循环体内，不然下一次循环的guess不是字符串
        let mut guess = String::new();

        // 不使用use std::io，可以直接写std::io::stdin()
        // 引用默认是不可变的，&guess是不可变的引用
        io::stdin().read_line(&mut guess)
            .ok()
            .expect("Failed to read line");

        println!("You guessed: {}", guess);

        // 注意，这里有一个match
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // match结束不用加分号
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
