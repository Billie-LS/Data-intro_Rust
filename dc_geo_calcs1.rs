use std::collections::HashMap;
use std::io;

fn calculate_rectangle_area(length: f64, width: f64) -> f64 {
    length * width
}

fn calculate_triangle_area(base: f64, height: f64) -> f64 {
    0.5 * base * height
}

fn calculate_circle_area(radius: f64) -> f64 {
    std::f64::consts::PI * radius * radius
}

fn main() {
    let mut shapes: HashMap<u32, (&'static str, &'static str, &'static str)> = HashMap::new();
    shapes.insert(1, ("Rectangle", "length", "width"));
    shapes.insert(2, ("Triangle", "base", "height"));
    shapes.insert(3, ("Circle", "radius", ""));

    loop {
        println!("Shape Area Calculator");
        for (choice, (shape, param1, param2)) in &shapes {
            println!("{}. {}", choice, shape);
        }
        println!("4. Exit");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let choice: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid choice. Please select 1, 2, 3, or 4.");
                continue;
            }
        };

        if choice == 4 {
            println!("Goodbye!");
            break;
        }

        match shapes.get(&choice) {
            Some(&(shape, param1, param2)) => {
                let param1_prompt = format!("Enter the {} of the {}: ", param1, shape);
                let param2_prompt = format!("Enter the {} of the {}: ", param2, shape);
                let get_param = |param_prompt: &'static str| -> f64 {
                    loop {
                        let mut param = String::new();
                        print!("{}", param_prompt);
                        io::stdin()
                            .read_line(&mut param)
                            .expect("Failed to read line");
                        if let Ok(value) = param.trim().parse() {
                            return value;
                        }
                        println!("Invalid input. Please enter a valid number.");
                    }
                };
                let param1_value = get_param(&param1_prompt);
                let param2_value = get_param(&param2_prompt);

                let area = match choice {
                    1 => calculate_rectangle_area(param1_value, param2_value),
                    2 => calculate_triangle_area(param1_value, param2_value),
                    3 => calculate_circle_area(param1_value),
                    _ => 0.0,
                };

                println!("The area of the {} is: {}", shape, area);
            }
            None => println!("Invalid choice. Please select 1, 2, 3, or 4."),
        }
    }
}
