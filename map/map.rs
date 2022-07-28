// To run locally, use: rustc map.rs && ./map

#[derive(Debug)]
struct Person {
    display_name: String,
    family_name: String,
}

#[derive(Debug)]
struct SimplePerson {
    name: String,
    family: String,
}

fn main() {

    // this is a silly example, but it's a good example of how to use the map function
    let arr = [
        Person { display_name: String::from("John"), family_name: String::from("Doe") },
        Person { display_name: String::from("Jane"), family_name: String::from("Doe") },
        Person { display_name: String::from("Jaken"), family_name: String::from("Herman") },
    ];


    let res = arr.map(|r| SimplePerson {
      name: r.display_name.clone(),
      family: r.family_name.clone(),
    });
    println!("{:?}", res);
    // ^^^
    // [
        // SimplePerson { name: "John", family: "Doe" },
        // SimplePerson { name: "Jane", family: "Doe" },
        // SimplePerson { name: "Jaken", family: "Herman" }
    // ]
