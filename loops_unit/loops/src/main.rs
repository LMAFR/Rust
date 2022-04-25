fn main() {
    let mut counter = 1;

    let stop_loop = loop {
        counter *= 2;
        if counter > 100 {
            // Stop loop, return counter value
            break counter;
        }
    };

    // Loop should break when counter = 128
    println!("Brak the loop at counter = {}.", stop_loop)
}
