use shuntyard::ast::AstNode;

fn shuntyard(input: &str) -> String {
    let mut stack: Vec<char> = Vec::new();
    let mut output = String::new();
    let numbers = "0123456789";
    let ops = "-+";

    for c in input.chars() {
        if numbers.contains(c) {
            output.push(c)
        }
        if ops.contains(c) {
            stack.push(c)
        }
    }
    while let Some(c) = stack.pop() {
        output.push(c)
    }
    output
}

fn main() {
    println!("{}", shuntyard("1+2-3"));
    println!("Hello, world!");
    println!("{:?}", AstNode::Empty);
}
