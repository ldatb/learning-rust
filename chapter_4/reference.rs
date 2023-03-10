fn main() {
    let s1 = String::from("hello"); // s comes into scope

    take_ownership(&s1);
    println!("{s1}");

    let mut s2 = s1.clone();
    mutable_reference(&mut s2);
    println!("{s2}");

    // let r1 = &s2; // no problem
    // let r2 = &s2; // no problem
    // let r3 = &mut s2; // BIG PROBLEM
}

fn take_ownership(s: &String) {
    println!("{s}");
} // S will still be valid because it was passed on as a reference

// A reference points to the pointer of the original variable, instead of pointing directly to the value
// https://doc.rust-lang.org/book/img/trpl04-05.svg

fn mutable_reference(s: &mut String) {
    s.push_str(". I'm mutable");
    println!("{s}");
}
