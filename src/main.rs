use std::io::stdin;

enum FourArithmeticOperators {
    Add,
    Sub,
    Mul,
    Div,
}

fn main() {
    let mut arg = String::new();
    let mut args: Vec<String> = Vec::new();
    let mut result = 0;

    println!("start calculator");

    println!("please number >");
    stdin().read_line(&mut arg).expect("Failed to read line");
    args.push(arg.parse().expect("Failed to parse"));

    println!("please four arithmetic operators(+, -, *, /) >");
    stdin().read_line(&mut arg).expect("Failed to read line");

    let parsed_arg: String = arg.parse().expect("Failed to parse");

    /* match parsed_arg.to_string() {
        "+" => args.push(parsed_arg),
        "-" => args.push(parsed_arg),
        "*" => args.push(parsed_arg),
        "/" => args.push(parsed_arg),
    } */
    args.push(arg.parse().expect("Failed to parse"));

    println!("please number >");
    stdin().read_line(&mut arg).expect("Failed to read line");
    args.push(arg.parse().expect("Failed to parse"));

    println!("please four arithmetic operators or enter >");
    stdin().read_line(&mut arg).expect("Failed to read line");
    args.push(arg.parse().expect("Failed to parse"));

    println!("{}", arg);
}
