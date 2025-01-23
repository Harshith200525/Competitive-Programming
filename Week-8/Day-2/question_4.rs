// Question 4: Basic Calculator II
// How to evaluate a string expression containing +, -, *, and / operators using Rust?

fn question_4(s: String) -> i32 {
    let mut stack = Vec::new();
    let mut num = 0;
    let mut sign = '+';
    let chars: Vec<char> = s.chars().collect();

    for i in 0..chars.len() {
        if chars[i].is_digit(10) {
            num = num * 10 + chars[i].to_digit(10).unwrap() as i32;
        }
        if (chars[i] == '+' || chars[i] == '-' || chars[i] == '*' || chars[i] == '/' || i == chars.len() - 1) && chars[i] != ' ' {
            match sign {
                '+' => stack.push(num),
                '-' => stack.push(-num),
                '*' => {
                    let last = stack.pop().unwrap();
                    stack.push(last * num);
                }
                '/' => {
                    let last = stack.pop().unwrap();
                    stack.push(last / num);
                }
                _ => {}
            }
            sign = chars[i];
            num = 0;
        }
    }

    stack.iter().sum()
}
