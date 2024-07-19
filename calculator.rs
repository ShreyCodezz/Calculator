use std::io;

fn main(){
    calculator();
}

fn calculator() {
    println!("Fast Calculator!");
    println!("Enter the first value: ");
    let mut x: String = String::new();
    io::stdin().read_line(&mut x).expect("INVALID INPUT");
    let new_x = x.trim().parse::<i32>().expect("ENTER A VALID DIGIT");

    println!("Enter the second value: ");
    let mut y = String::new();
    io::stdin().read_line(&mut y).expect("INVALID INPUT");
    let new_y = y.trim().parse::<i32>().expect("ENTER A VALID DIGIT");

    println!("Enter the operator use +,-,*,/ OR % : ");
    let mut op = String::new();
    io::stdin().read_line(&mut op).expect("INVALID OPERATOR");
    let op = op.trim();

    match op {
        "+" => println!("Your sum is {}", new_x + new_y),
        "-" => println!("Your difference is {}", new_x - new_y),
        "*" => println!("Your product is {}", new_x * new_y),
        "/" => {
            if new_y != 0 {
                println!("Your quotient is {}", new_x / new_y)
            } else {
                println!("Cant be divided by zero")
            }
        }
        "%" => {
            if new_y != 0 {
                println!("Your remainder is {}", new_x % new_y)
            } else {
                println!("Cant compute remainder with zero")
            }
        }
        _ => println!("INVALID OPERATOR! PLEASE USE +,-,*,/ OR %"),
    }
}
