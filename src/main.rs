use std::io::stdin;

fn main() {
    println!("start calculator");
    println!("q is end calculator");

    loop {
        let mut args: Vec<&str> = Vec::new();
        let mut result = 0;

        println!("number >");
        let input_left_number = input();
        match input_left_number.as_str() {
            "q" => {
                println!("end calculator");
                break;
            }
            _ => {}
        }
        args.push(&input_left_number);

        println!("+, -, *, / >");
        let input_arithmetic = input();
        match input_arithmetic.as_str() {
            "+" => args.push(&input_arithmetic),
            "-" => args.push(&input_arithmetic),
            "*" => args.push(&input_arithmetic),
            "/" => args.push(&input_arithmetic),
            _ => {
                break;
            }
        }

        println!("number >");
        let input_right_number = input();
        match input_left_number.as_str() {
            "q" => {
                println!("end calculator");
                break;
            }
            _ => {}
        }
        args.push(&input_right_number);

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
                        break;
                    }
                }
            }
        }
        println!(
            "calculate > {} {} {} = {}",
            args[0], args[1], args[2], result
        )
    }
}

fn input() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read line");
    return input.trim().to_string();
}
