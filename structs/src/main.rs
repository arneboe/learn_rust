#[derive(Debug)]
struct Test {
    a: String,
    b: i32,
    c: i32,
}

impl Test {
    fn calc(&self) -> i32 {
        return self.b * self.c;
    }
}

fn main() {
    let t: Test = Test {
        a: String::from("bla"),
        b: 42,
        c: 42,
    };

    println!("{}", t.calc());
}

enum EN {
    ENA,
    ENB { x: i32, y: i32 },
    ENC(i64),
}

impl EN {}
