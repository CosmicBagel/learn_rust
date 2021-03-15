fn main () {
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert("Blue".to_string(), 10);
    scores.insert("Yellow".to_string(), 50);

    let teams = vec!["Blue".to_string(), "Yellow".to_string()];
    let inital_scores = vec![10,50];

    let mut scores: HashMap<_,_> = teams.into_iter().zip(inital_scores.into_iter()).collect();

    let team_name = "Blue".to_string();
    let score = scores.get(&team_name);

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}

fn string_ex () {
    let data = "inital contents";

    let s = data.to_string();

    let s = "inital contents".to_string();
}

fn vector_ex() {
    let v = vec![1, 2, 3, 4, 5];

    let third = v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(&third) => println!("The third element is {}", third),
        None => println!("There is not third element"),
    }

    // let does_not_exist = &v[100]; // causes panic
    let does_not_exist = v.get(100);

    // println!("The 100th element is: {}", does_not_exist)

    // all new vec for all new borrows and references
    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];
    println!("The first element is {}", first);
    v.push(6);

    let v = vec![100, 32, 57];

    let mut v = v;
    for i in &mut v {
        *i += 50
    }

    for i in &v {
        println!("{}", i);
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
