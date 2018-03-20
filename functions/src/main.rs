fn main() {
    test(9);
    for i in 0..4 {
        println!("{}", i);
    }

    let mut s: String = String::new();
    s = return_string(s);
    s.push_str("sad");
    take_string(s);
}

fn return_string(s: String) -> String {
    return s;
}

fn take_string(s: String) {
    println!("{}", s);
}

// fn bla(s: &str) -> &str {
//     // for (i, &item) in s.iter().enumerate() {}
// }

fn test(x: i64) {
    let result = if x > 10 { x + 42 } else { x + 13 };

    println!("Hello, {}", result);
}
