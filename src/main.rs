use std::io::stdin;
use std::io::Read;
use std::panic;
use std::fs;
use std::fs::File;
use std::io::{self, Write};


fn subify(str: &String, slice_len: i32,step: usize) -> Vec<String>{
    let mut str_vec:Vec<String> = vec![];
    let len:i32;

    if str.len() as i32 - slice_len<= slice_len{
        panic!("Size missmatch!");
    }
    else {
        len = str.len() as i32 - slice_len;
    }

    for i in (0..(len as usize+1)).step_by(step){
        let chunk = &str[i..(i+slice_len as usize)];
        str_vec.push(chunk.to_string());
    }
    return str_vec
}

fn main() {

    
    // Use the file_contents string here

    println!("Enter the path to the file (no spaces or weird stuff)");
    let mut file_path = String::new();
    io::stdin().read_line(&mut file_path).expect("Failed to read line");
    let file_path = file_path.trim();

    //get length of sub string
    println!("Enter length of sub string");
    let mut sub_len = String::new();
    io::stdin().read_line(&mut sub_len).expect("Failed to read line");
    let sub_len: i32 = sub_len.trim().parse().expect("Please type a number!");

    //get step size for generating sequences
    println!("Enter step size.");
    let mut step = String::new();
    io::stdin().read_line(&mut step).expect("Failed to reaad line!");
    let step:usize = step.trim().parse().expect("Please type a number!");

    //get output name
    println!("Enter name of output file");
    let mut out_file = String::new();
    stdin().read_line(&mut out_file).expect("oops");
    let file_contents = fs::read_to_string(file_path).expect("Failed to read file");



    let output = subify(&file_contents, sub_len,step);

    let output = output.join("\n");
    let mut file = File::create(out_file+".txt").expect("create failed");
    file.write_all(output.as_bytes()).expect("write failed");
    
}
