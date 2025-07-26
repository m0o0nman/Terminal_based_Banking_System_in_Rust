use std::io;
pub fn input() -> f64{
    let mut ip: String = String::new();
    let op: f64;
    loop{
        ip.clear();
        println!("Please enter a number:");

        io::stdin()
            .read_line(&mut ip)
            .expect("Failed to read line");
        
        match ip.trim().parse() {
            Ok(num) => {
                op = num;
                break;
            }
            Err(_) => {
                println!("Please enter a number.");
            }
        }
    }
    return op; 
}