use std::io;
mod input_f64;
mod input_i32;

fn main(){
    
    println!("Enter your name: ");
    let mut ip: String = String::new();
    io::stdin()
        .read_line(&mut ip)
        .expect("Failed to read line");
    let owner = ip.trim().to_string();

    println!("Enter your Balance: ");
    let balance: f64 = input_f64::input();

    let mut user: BankAcount = BankAcount{
        owner,
        balance,
    };

    println!("======== Welcome to Bank of Crabs ========");
    println!("What do you want to do?\n1. See Balance\n2. Add\n3. Withdraw\n4. Exit");
    let mut choice: i32;
    
    loop {
        choice = input_i32::input();
        if choice == 1 || choice == 2 || choice == 3 || choice == 4 {
            break;
        }
        else {
            println!("Please enter a number from 1 to 4.");
            continue;
        }
    }

    match choice {
        1 => show_details(&mut user),
        2 => {
            let amount: f64 = input_f64::input();
            add(&mut user, amount);
        }
        3 => {
            let amount: f64 = input_f64::input();
            withdraw(&mut user, amount);
        }
        4 => {
            println!("Thank you for using our software :)");
        }

        _ => println!("Please enter a number between 1 to 4"),

    }
 }

struct BankAcount{
    owner: String,
    balance: f64,
    
}

fn show_details(user: &BankAcount){
    println!("\nOwner: {}\n Balance: {:.2}", user.owner, user.balance);
}

fn withdraw(user: &mut BankAcount, amount: f64) {
    user.balance -= amount;
    println!("{} Withdraw successful", amount);
    show_details(user);
}

fn add(user: &mut BankAcount, amount: f64){
    user.balance += amount;
    println!("{} Add successful", amount);
    show_details(user);
}