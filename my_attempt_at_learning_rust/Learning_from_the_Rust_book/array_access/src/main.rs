use std::io;

fn main() {
    // implicit array type
    let a = [1, 2, 3, 4, 5]; 

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
 // ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ is the same as:
 // io::stdin().read_line(&mut index).expect("Failed to read line");

    
    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
