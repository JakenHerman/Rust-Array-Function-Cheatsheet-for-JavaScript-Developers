// To run locally, use: rustc filter.rs && ./filter

#[derive(Debug)]
struct Example {
    prop: u8,
    name: String,
}

fn main() {
    let arr = [
        Example { prop: 1, name: String::from("first") },
        Example { prop: 2, name: String::from("second") },
        Example { prop: 2, name: String::from("third") }
    ];


    // filter `arr` down to include only the elements that have
    // object `prop` properties of the `comparator` variable (2). 
    // save the result into a new array called `res`.
    let comparator = 2;
    let res = arr.iter().filter(|x| x.prop == comparator).collect::<Vec<_>>();

    println!("{:?}", res);  // [Example { prop: 2, name: "second" }, Example { prop: 2, name: "third" }]
}
