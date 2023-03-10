fn main() {
    let s1 = "I'm a string literal!";

    // let mut s1 = "I'm a string literal!";
    // s1 += "this doesn't work";

    let mut s2 = String::from("I'm a");
    s2.push_str(" String!");

    println!("{s1}");
    println!("{s2}");
}
