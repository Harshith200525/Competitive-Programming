// Question 2: Brace Expansion II
// How to expand a string with nested braces into all possible combinations using recursive backtracking?

use std::collections::HashSet;

fn question_2(expression: String) -> Vec<String> {
    fn expand(expr: &str) -> HashSet<String> {
        let mut result = HashSet::new();

        if !expr.contains('{') {
            result.insert(expr.to_string());
            return result;
        }

        let mut stack = Vec::new();
        let mut temp = String::new();

        for c in expr.chars() {
            match c {
                '{' => {
                    if !temp.is_empty() {
                        stack.push(temp.clone());
                        temp.clear();
                    }
                    stack.push("{".to_string());
                }
                '}' => {
                    if !temp.is_empty() {
                        stack.push(temp.clone());
                        temp.clear();
                    }
                    let mut group = HashSet::new();
                    while let Some(top) = stack.pop() {
                        if top == "{" {
                            break;
                        }
                        for sub in expand(&top) {
                            group.insert(sub);
                        }
                    }
                    stack.push(group.into_iter().collect::<Vec<_>>().join(","));
                }
                ',' => {
                    if !temp.is_empty() {
                        stack.push(temp.clone());
                        temp.clear();
                    }
                }
                _ => {
                    temp.push(c);
                }
            }
        }

        if !temp.is_empty() {
            stack.push(temp);
        }

        let mut final_result = HashSet::new();
        for s in stack {
            final_result.extend(expand(&s));
        }

        final_result
    }

    let mut result = expand(&expression).into_iter().collect::<Vec<_>>();
    result.sort();
    result
}
