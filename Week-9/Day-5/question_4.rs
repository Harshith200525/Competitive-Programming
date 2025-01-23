// Question 4: Encode and Decode Strings
// How to design a system for encoding and decoding a list of strings?

fn encode(strs: Vec<String>) -> String {
    strs.into_iter()
        .map(|s| format!("{}#{}", s.len(), s))
        .collect()
}

fn decode(encoded: String) -> Vec<String> {
    let mut result = Vec::new();
    let chars: Vec<char> = encoded.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        let mut j = i;
        while chars[j] != '#' {
            j += 1;
        }

        let length: usize = chars[i..j].iter().collect::<String>().parse().unwrap();
        let start = j + 1;
        result.push(chars[start..start + length].iter().collect());
        i = start + length;
    }

    result
}
