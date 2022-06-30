use std::io;

fn main() {

    // tuple
    let tup = (28, 2.987, 334);

    let (x, y, z) = tup;

    let p = tup.0;
    let q = tup.1;
    let r = tup.2;

    println!("The value of (x, y, z) = ({}, {}, {})",x, y, z);
    println!("The value of (p, q, r) = ({}, {}, {})", p, q, r);

    // Array

    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line.");
    
    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number.");

    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );
}
