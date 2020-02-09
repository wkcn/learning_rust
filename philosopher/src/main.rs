use std::thread;
use std::sync::{Mutex, Arc};

struct Table {
    forks: Vec<Mutex<()>>,
}

// 结构体定义结尾不用加分号;
// 将名字的类型选择成 String 型，而不是 &str。一般来说，使用一种自身拥有数据的类型比使用一个利用引用的类型容易的多。
struct Philosopher {
    name: String,
    left: usize,
    right: usize,
}

impl Philosopher {
    /*
     * Rust 是一种基于表达式的语言，这意味着 Rust 中几乎所有的内容都是返回一个值的表达式。这个说法对函数亦是如此，因为函数的最后一个表达式将自动返回。由于这个函数的最后一个表达式是我们创建了一个新的 Philosopher，我们最终返回这个 Philosopher。
     */
    fn new(name: &str, left: usize, right: usize) -> Philosopher {
        Philosopher {
            name: name.to_string(),
            left: left,
            right: right,
        }
    }

    fn eat(&self, table: &Table) {
        /*
         * 这些下划线有什么用？嗯，因为我们不打算使用锁内的值。我们只是想持有锁。因此，Rust 会警告我们，我们从来没有使用过锁内值。通过使用下划线，我们就会告诉 Rust，这就是我们想要的，这样的话 Rust 就不会抛出警告。
         */
        let _left = table.forks[self.left].lock().unwrap();
        let _right = table.forks[self.right].lock().unwrap();

        println!("{} is eating.", self.name);
        thread::sleep_ms(1000);
        println!("{} is done eating.", self.name);
    }
}

fn main() {
    /*
     * “arc” 是 “atomic reference count” 的缩写，我们要通过多线程来共享我们的 Table。当我们要共享它时，引用数就会上升，当每个线程结束时，引用数就会减少。
     */
    let table = Arc::new(Table { forks: vec![
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
    ]});

    let philosophers = vec![
        Philosopher::new("Baruch Spinoza", 0, 1),
        Philosopher::new("Gilles Deleuze", 1, 2),
        Philosopher::new("Karl Marx", 2, 3),
        Philosopher::new("Friedrich Nietzsche", 3, 4),
        Philosopher::new("Michel Foucault", 0, 4), // 这里的0, 4用于防止死锁
    ];

    let handles: Vec<_> = philosophers.into_iter().map(|p| {
        // Arc<T> 调用的 clone() 方法会增加引用的计数
        let table = table.clone();
        thread::spawn(move || {
            p.eat(&table);
        })
    }).collect();

    for h in handles {
        h.join().unwrap();
    }
}
