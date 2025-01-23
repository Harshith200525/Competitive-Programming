// Question 4: Valid Number
// How to determine whether a given string represents a valid number?

fn question_4(s: String) -> bool {
    let trimmed = s.trim();
    let valid_number = |s: &str| -> bool {
        let chars: Vec<char> = s.chars().collect();
        let mut has_digit = false;
        let mut has_point = false;

        for (i, &ch) in chars.iter().enumerate() {
            match ch {
                '+' | '-' if i == 0 || chars[i - 1] == 'e' || chars[i - 1] == 'E' => {}
                '0'..='9' => has_digit = true,
                '.' if !has_point => has_point = true,
                'e' | 'E' if has_digit => {
                    return valid_number(&s[i + 1..]);
                }
                _ => return false,
            }
        }

        has_digit
    };

    valid_number(trimmed)
}
