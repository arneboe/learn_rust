fn main() {
    let test = (42, 43, "test");

    println!("{}, {}, {}", test.0, test.1, test.2);

    let (x, y, t): (i32, i16, &str) = test;

    println!("{}, {}, {}", t, x, y);

    let mut arr = [42, 43, 44];

    println!("{:?}", arr);

    let mut bla: &i32 = &arr[0];
    bla = &1337;
    println!("{}", bla);
}
