// Question 5: Evaluate Reverse Polish Notation
// How to evaluate an expression in Reverse Polish Notation (postfix notation) using Rust?

fn question_5(tokens: Vec<String>) -> i32 {
    let mut stack = Vec::new();

    for token in tokens {
        match token.as_str() {
            "+" => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(a + b);
            }
            "-" => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(a - b);
            }
            "*" => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(a * b);
            }
            "/" => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(a / b);
            }
            _ => stack.push(token.parse().unwrap()),
        }
    }

    stack.pop().unwrap()
}
