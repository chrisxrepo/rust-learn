mod fib;

fn main() {
    let s = String::from("abc");
    let s1 = &s;
    let s2 = s1;
    println!("{}, {}", s1, s2);

    println!("hello world");
}
