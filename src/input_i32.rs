use std::io;

pub fn input() -> i32{
    let mut ip: String = String::new();
    let op: i32;

    loop{
        ip.clear();

        io::stdin()
            .read_line(&mut ip)
            .expect("Failed to read line");

        match ip.trim().parse::<i32>() {
            Ok(num) => {
                op = num as i32;
                break;
            }

            Err(_) => {
                println!("Please type a number!");
            }
        }
    }
    return op;
}