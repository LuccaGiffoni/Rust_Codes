use std::io;

fn main () 
{
	// Strings for name and age
	let mut name = String::new();
	let mut age = String::new();
  let mut n1 = String::new();
  let mut n2 = String::new();
  let mut operation = String::new();

	// Reading the inputs
	println!("Hello, use! What's your name? ");
	io::stdin().read_line(&mut name).expect("Failed to read! Try again.");
	let name : String = name.trim().parse().expect("Failed to parse!");
  println!();

	println!("What a beautiful name, { }! Now, please, tell me: how old are you? ", name);
	io::stdin().read_line(&mut age).expect("Failed");
	let age : i32 = age.trim().parse().expect("Failed to parse!");
  println!();

	if age >= 18
  {
		println!("\nYou are able to use the calculator!");
    println!("What you wanna do?\n");

    // Asking the first number
    println!("Type your first number: ");
    io::stdin().read_line(&mut n1).expect("Failed");
    let n1 : i32 = n1.trim().parse().expect("Failed to parse!");

    // Asking the second number
    println!("\nType your second number: ");
    io::stdin().read_line(&mut n2).expect("Failed");
    let n2 : i32 = n2.trim().parse().expect("Failed to parse!");

    // Asking for the operation
    println!("\nWhat you wanna do?");
    println!("1. Add");
    println!("2. Subtract");
    println!("3. Divide");
    println!("4. Multiply");

    io::stdin().read_line(&mut operation).expect("Failed");
    let operation : i32 = operation.trim().parse().expect("Failed to parse!");
    println!();

    // Switch case for the operations
    if operation == 1
    {
      println!();
      println!("{ } + { } = { }", n1, n2, n1 + n2);
    }

    else if operation == 2
    {
      println!();
      println!("{ } - { } = { }", n1, n2, n1 - n2);
      println!("\nAND\n{ } - { } = { }", n2, n1, n2 - n1);
    }

    else if operation == 3
    {
      println!("{ } / { } = { }", n1, n2, n1 as f64 / n2 as f64);
      println!("AND\n{ } / { } = { }", n2, n1, n2 as f64 / n1 as f64);
    }

    else if operation == 4
    {
      println!("{ } * { } = { }", n1, n2, n1 * n2);
    }
	
	else{
		let years_until = 18 - age;
		println!("You're too young try again in { } years!", years_until);
	}
  }
}
