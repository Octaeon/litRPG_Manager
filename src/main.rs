use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Input the name of the file as the first argument!");
        return;
    }

    let filename = &args[1];
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
