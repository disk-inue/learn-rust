pub mod calculate {
    use std::io::{stdin, stdout, Write};

    pub fn exec() {
        println!("start calculator");
        println!("q is end calculator");

        loop {
            let mut args: Vec<&str> = Vec::new();

            let input_left_number = input("number > ");
            if input_left_number == "q" {
                println!("end calculator");
                break;
            }
            let input_left_number: f64 = match input_left_number.parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please input number! end calculator");
                    break;
                }
            };

            let input_arithmetic = input("+, -, *, / > ");
            match input_arithmetic.as_str() {
                "+" => args.push(&input_arithmetic),
                "-" => args.push(&input_arithmetic),
                "*" => args.push(&input_arithmetic),
                "/" => args.push(&input_arithmetic),
                _ => {
                    break;
                }
            }

            let input_right_number = input("number > ");
            if input_right_number == "q" {
                println!("end calculator");
                break;
            }
            let input_right_number: f64 = match input_right_number.parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please input number! end calculator");
                    break;
                }
            };

            let result = calculate(&input_left_number, &input_arithmetic, &input_right_number);
            println!(
                "calculate > {} {} {} = {}",
                input_left_number, input_arithmetic, input_right_number, result
            )
        }
    }

    fn input(message: &str) -> String {
        print!("{}", &message);
        stdout().flush().unwrap();
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed to read line");
        input.trim().to_string()
    }

    fn calculate(left_number: &f64, arithmetic: &str, right_number: &f64) -> f64 {
        match arithmetic {
            "+" => left_number + right_number,
            "-" => left_number - right_number,
            "*" => left_number * right_number,
            "/" => left_number / right_number,
            _ => 0.0,
        }
    }
}
