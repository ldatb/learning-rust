fn main() {
    let s1 = String::from("hello"); // s1 comes into scope
    let s2 = s1.clone();

    take_ownership(s1); // S ownership changes
    //println!("{s1}"); // This is not valid due to Rust's ownership

    let s3 = take_ownership_and_return(s2);
    println!("{s3}");

    let str_lit: &str = "hello string literal";
    string_take_ownership(str_lit);
    println!("{str_lit}");

    let x: u8 = 5;
    number_take_ownership(x);
    println!("{x}");
}

fn take_ownership(s: String) {
    println!("{s}");
} // S is no longer valid here because String is a heap

fn take_ownership_and_return(s: String) -> String {
    println!("{s}");
    s
} // S will be valid because it's value is returned

fn string_take_ownership(str_lit: &str) {
    println!("{str_lit}");
} // STR_LIT will still be valid

fn number_take_ownership(x: u8) {
    println!("{x}");
} // X will still be valid since it's not a heap
