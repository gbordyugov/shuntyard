fn shuntyard(input: &str) -> String {
    let mut stack: Vec<char> = Vec::new();
    let mut output = String::new();
    let numbers = "0123456789";
    let ops = "-+*/";

    for c in input.chars() {
        if numbers.contains(c) {
            output.push(c);
        }
        if ops.contains(c) {
            stack.push(c);
        }
    }
    output
}

fn main() {
    println!("{}", shuntyard("014+145-/145"));
    println!("Hello, world!");
}
