use std::io::{stdin, stdout, Write};

fn main() {
    println!("start calculator");
    println!("q is end calculator");

    loop {
        let mut args: Vec<&str> = Vec::new();

        let input_left_number = input("number > ".to_string());
        match input_left_number.as_str() {
            "q" => {
                println!("end calculator");
                break;
            }
            _ => {}
        }
        args.push(&input_left_number);

        let input_arithmetic = input("+, -, *, / > ".to_string());
        match input_arithmetic.as_str() {
            "+" => args.push(&input_arithmetic),
            "-" => args.push(&input_arithmetic),
            "*" => args.push(&input_arithmetic),
            "/" => args.push(&input_arithmetic),
            _ => {
                break;
            }
        }

        let input_right_number = input("number > ".to_string());
        match input_left_number.as_str() {
            "q" => {
                println!("end calculator");
                break;
            }
            _ => {}
        }
        args.push(&input_right_number);

        let result: f32 = calculate(&args);
        println!(
            "calculate > {} {} {} = {}",
            args[0], args[1], args[2], result
        )
    }
}

fn input(message: String) -> String {
    print!("{}", &message);
    stdout().flush().unwrap();
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read line");
    return input.trim().to_string();
}

fn calculate(args: &Vec<&str>) -> f32 {
    let mut result: f32 = 0.0;
    for n in 0..args.len() {
        if n == 0 {
            result += args[n].parse::<f32>().unwrap();
            continue;
        }
        if n % 2 == 0 {
            let num = args[n].parse::<f32>().unwrap();
            match args[n - 1] {
                "+" => result += num,
                "-" => result -= num,
                "*" => result *= num,
                "/" => result /= num,
                _ => {
                    return 0.0;
                }
            }
        }
    }
    return result;
}
