fn main() {
    let s = String::from("jazz bazz");
    println!("{}", first_word("hello world".to_string()));
    println!("{}", first_word_v2("hello world"));
    println!("{}", first_word_v2(&s[..]));
}

fn first_word(s: String) -> String {
    let mut out_str = String::new();

    for (_, c) in s.chars().enumerate() {
        out_str.push(c);
        if c == ' ' {
            break;
        }

    }

    out_str
}

fn first_word_v2(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}