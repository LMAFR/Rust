fn process(input: String) {}

fn caller() {
    let s = String::from("Hello, world!");
    process(s); // Ownership of the string in `s` moved into `process`
    println!("{}", s); // Error! ownership already moved.
}

fn main() {
    caller();
}
