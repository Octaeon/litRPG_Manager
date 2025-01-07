use std::fs;

fn main() {
    let filename = "test.txt";

    println!("The file {filename} has these contents:");
    let contents = fs::read_to_string(filename);

    match contents {
        Ok(a) => {
            println!("{a}");
        }
        Err(e) => {
            println!("Error! {e}")
        }
    }
}
