fn what_is_ownership() {

    // RULEs
    // Each value in Rust has a variable that's called it's owner.
    // There can only be one owner at a time.
    // When the owner goes out of scope, the value will be dropped.

    let mut s = String::from("hello");

    s.push_str(", world!");

    println!("{}", s); 

    // bind the value 5 to x
    let x = 5;
    // make a copy of the value in x and bind it to y.
    #[allow(unused_variables)]
    let y = x;
    // these↑ two values pushed on to the stack

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("{}, {}", s1, s2);
}

fn gives_ownership() -> String {

    let some_string = String::from("hello");

    some_string

}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}

fn main() {
    what_is_ownership();

    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);

    println!("{} {}", s1, s3);

    let (s4, len) = calculate_length(s1);
    println!("The length of {} is {}", s4, len);
}
