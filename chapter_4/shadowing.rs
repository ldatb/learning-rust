fn main() {
    // Original value
    let x = 5;

    let z = &x;
    
    {
        // Shadowed value
        let x = x + 5;
        let y = &x;
        println!("Shadowed X: {x}");
        println!("Referenced outer X: {z}");
        println!("Referenced inner X: {y}");
    }
}
