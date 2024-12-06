fn main() {
    loop {
        let mut n = String::new();

        println!("Enter nth place to find:");

        std::io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");

        let target: i32 = n
        .trim()
        .parse()
        .expect("Index entered was not a number");


        let mut curr: u64 = 1;
        let mut prev: u64 = 0;

        for _ in 1..target {
            let next = prev.checked_add(curr);
            match next {
                Some(next) => {
                    prev = curr;
                    curr = next;
                }
                None => {
                    println!("Calculation overflow!");
                    break;
                }
            }
            
        }

        println!("fib n = {}", curr)
        }
}



