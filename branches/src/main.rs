use std::{array, ops::RemAssign};

fn main() {
    let condition: bool = true;
    //We can use conditional logic to put an expression after our let statement
    let number = if condition  { 5 } else { 6 };

    if number <= 5 {
        println!("condition was true");
    }
    else {
        println!("condtion was false");
    }

    let arr = [1, 2, 3, 4, 5];
    counter(5);
    arrayposter(&arr, 2);
    forarrayposter(&arr);
    blastoff();
}


fn counter(iterations: i32) {
    let mut count = 0; 
    while count <= iterations {
        println!("count = {count}");
        count += 1;
    }
}

fn arrayposter(arr: &[i32], target: usize){
    if target >= arr.len() { println!("Select an index in range of array") }
    else {
        let mut index: usize = 0;

        while index <= target {
            println!("the value at {index} is: {}", arr[index]);
            index += 1;
        }
    }   
}

fn forarrayposter(arr: &[i32]){
    for element in arr {
        println!("the value is: {element}")
    }
}

fn blastoff() {
    for number in (1..5).rev() {
        println!("T MINUS {number}...");
    }
    println!("LIFTOFF!");
}