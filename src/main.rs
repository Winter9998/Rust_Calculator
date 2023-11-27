use std::fmt::format;
use std::io::{stdin};
use std::num::ParseIntError;

pub fn calculator(x: String, y: String, z: String) -> i64 {
    let mut res: i64 = 0;
    let num1 = x.trim().parse::<i64>()
        .map_err(|e| format("Error parsing x: {}", e))?;
    let num2 = y.trim().parse::<i64>();

    let res = match z.trim() {
        "+" => num1 + num2,
        "-" => num1 - num2,
        "x" => num1 * num2,
        ":" => num1 / num2,
        _ => Err("Invalid operation"),
    };


}

fn main() -> i64 {
    let mut x = String::new();
    println!("Choose your first number ! : ");
    stdin().read_line(&mut x)?;
    x.trim().parse::<i64>().expect("Couldn't retrieve data");
    println!(" your first number is : {}", x);


    let mut y = String::new();
    println!("Choose your second number ! : ");
    stdin().read_line(&mut y)?;
    y.trim().parse::<i64>().expect("Couldn't retrieve data");
    println!(" your second number is : {}", y);

    let mut sign = String::new();
    println!("Choose what to do : + - * /");
    match stdin().read_line(&mut sign) {
        Ok(n) => {
            println!("Your sign is {}", sign);
        },
        Err(error) => {
            println!("{}", error);

        }
    };

    calculator(x,y, sign)

}
