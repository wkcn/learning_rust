fn main() {
    let x = 5;
    let y: i32 = 10;
    let mut z = 9;
    z += x;
    let x = 39;
    let (a, b) = (3, 4);
    let _dummy = 10;
    let out = x + y + z + a + b;
    println!("{}", out);
}
