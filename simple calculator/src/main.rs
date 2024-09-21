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

}
