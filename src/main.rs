use std::fs;

fn main() {
    let filename = "test.txt";

    println!("The file {filename} has these contents:");
    let loaded_file = fs::read_to_string(filename);

    match loaded_file {
        Ok(contents) => {
            println!("{contents}");
            fs::write("output.txt", contents + "\ntest")
                .expect("For some reason, couldn't write the output to file");
        }
        Err(e) => {
            println!("Error! {e}");
        }
    }
}
