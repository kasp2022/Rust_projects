fn main() {
    let string = no_dangle();

    println!("{}", string);
} 

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
