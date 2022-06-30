fn main() {
    println!("Hello, world!");

    let x = {
        let p = 160;
        p*2
    };

    one_more_hello();

    println!("The value of x is: {}", x)
}

fn one_more_hello() {
    println!("Here's another hello world");
}

#[allow(dead_code)]
#[allow(unused_variables)]
fn summation(a1 : u32, a2: u32) -> u32 {
    #[allow(unused_mut)]
    let mut x: u32 = 0;
    for i in [a1..a2] {
        // 範囲指定に使った変数は整数型他の型とは足したりできない．
        // x = x + i[i.itr()];
    }

    x
}