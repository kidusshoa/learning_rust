use std::io::stdin;

fn main() {
    println!("Calculator Program\n\n");
    println!("--------------------------------");

    let mut num1: String = String::new();
    let mut num2: String = String::new();
    let mut choice: String = String::new();

    println!("Please provide the first number");
    stdin().read_line( &mut num1).expect( "Error in input");

    println!("Please provide the second number");
    stdin().read_line( &mut num2).expect( "Error in input");
    
    println!("Please provide the operation to be performed");
    println!("[+] for Addiction");
    println!("[-] for Substraction");
    println!("[*] for Multiplication");
    println!("[/] for Division");
    stdin().read_line( &mut choice).expect( "Error in input");

    let num1:f64 = num1.trim().parse.expect("Invalid number");
    let num2:f64 = num2.trim().parse.expect("Invalid number"); 

    let mut result: f64 = 0.0;

    let choice: &str = choice.as_str().trim();
    if choice == "+" {
        result = num1 + num2;
    }else if choice == "-" {
        result = num1 - num2;
    }else if choice == "*" {
        result = num1 * num2;
    }else if choice == "/" && num2 != 0.0 {
        result = num1 / num2;
    }else{
        println!("Invalid choice");
    }
}
