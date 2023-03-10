fn main() {
    let mut counter = 1;

    let result = loop {
        counter += 1;
        if counter == 2023 {
            break counter;
        }
    };

    println!("{result}");
}
