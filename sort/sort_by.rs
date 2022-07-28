// To run locally, use: rustc sort_by.rs && ./sort_by

#[derive(Debug)]
struct Example {
    prop: i8,
    name: String,
}

fn main() {
    let mut arr = [
        Example { prop: 3, name: String::from("first") },
        Example { prop: 2, name: String::from("second") },
        Example { prop: 1, name: String::from("third") }
    ];

    arr.sort_by(|a, b| a.prop.cmp(&b.prop));

    println!("{:?}", arr);  // [Example { prop: 1, name: "third" }, Example { prop: 2, name: "second" }, Example { prop: 3, name: "first" }]
}
