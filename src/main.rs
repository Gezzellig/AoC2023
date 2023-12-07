use std::fs::read_to_string;

fn extract_first_last_digit(line: str) {
    
}

fn e1a() {
    let filename = "1a.in";
    println!("In file {}", filename);


    for line in read_to_string(filename).unwrap().lines() {
        line.to_string()
    }

    // let lines = read_to_string(filename)
    //     .unwrap()  // panic on possible file-reading errors
    //     .lines()  // split the string into an iterator of string slices
    //     .map(String::from)  // make each slice into a string
    //     .collect();  // gather them together into a vector

    let contents = fs::read_to_string(in_file)
        .expect("Should have been able to read the file");

    for contents.lines()
    println!("With text:\n{contents}");
}


fn main() {
    e1a();
    println!("Hello, world!");
}
