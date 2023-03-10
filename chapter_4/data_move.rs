fn main() {
    let s1 = String::from("hello");
    let s2 = s1;
    let s3 = s2.clone();

    println!("{s2}");
    println!("{s3}");
    // println!("{s1}"); // <- This is invalid. https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html

    let i = 1;
    let j = i;
    println!("{i}");
    println!("{j}");
}
