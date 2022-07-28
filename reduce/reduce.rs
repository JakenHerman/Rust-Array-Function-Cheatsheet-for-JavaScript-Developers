// To run locally, use: rustc reduce.rs && ./reduce

#[derive(Debug)]
struct Example {
    prop: i8,
    name: String,
}

fn main() {
    let arr = [
        Example { prop: 1, name: String::from("first") },
        Example { prop: 2, name: String::from("second") },
        Example { prop: -2, name: String::from("third") }
    ];

    let res = arr.iter().reduce(|acc, obj| {
      if (obj.prop).abs() < (acc.prop).abs() {
          obj
      } else {
          acc
      }
    });

    println!("{:?}", res.unwrap());  // Example { prop: 1, name: "first" }
}
