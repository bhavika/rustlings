// variables2.rs
// Execute `rustlings hint variables2` or use the `hint` watch subcommand for a hint.

fn main() {
    let mut x = 6;
    if x == 10 {
        println!("x is ten!");
    } else {
        println!("x is not ten!");
        x = 12;
    }
    println!("x is {}", x);
}
