use std::io::stdin;

fn main() {
    let mut args: Vec<&str> = Vec::new();
    let mut result = 0;

    println!("start calculator");

    println!("please number >");
    let mut arg = String::new();
    stdin().read_line(&mut arg).expect("Failed to read line");
    args.push(arg.trim());

    let mut arg = String::new();
    println!("please four arithmetic operators(+, -, *, /) >");
    stdin().read_line(&mut arg).expect("Failed to read line");
    let parsed_arg = arg.trim();
    match parsed_arg {
        "+" => args.push(parsed_arg),
        "-" => args.push(parsed_arg),
        "*" => args.push(parsed_arg),
        "/" => args.push(parsed_arg),
        _ => {
            println!("please four arithmetic operators(+, -, *, /)");
            return;
        }
    }

    let mut arg = String::new();
    println!("please number >");
    stdin().read_line(&mut arg).expect("Failed to read line");
    args.push(arg.trim());

    let mut arg = String::new();
    println!("please four arithmetic operators or enter >");
    stdin().read_line(&mut arg).expect("Failed to read line");
    args.push(arg.trim());

    for n in 0..args.len() {
        if n == 0 {
            result += args[n].parse::<i32>().unwrap();
            continue;
        }
        if n % 2 == 0 {
            let num = args[n].parse::<i32>().unwrap();
            match args[n - 1] {
                "+" => result += num,
                "-" => result -= num,
                "*" => result *= num,
                "/" => result /= num,
                _ => {
                    println!("please four arithmetic operators(+, -, *, /)");
                    return;
                }
            }
        }
    }
    println!("{}", result)
}
