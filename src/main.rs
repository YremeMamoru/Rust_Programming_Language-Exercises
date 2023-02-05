use std::io::{self, Write, Read};

// Utils
fn clear_screen(){
    // ????
    print!("\x1B[2J\x1B[1;1H");
}


fn pause(){
    print!("Press any key to continue...");
    io::stdout()
        .flush()
        .unwrap();
    
    // Read a single byte and discard
    let _ = io::stdin().read(&mut [0u8]).unwrap();
}

// Convert temperatures between Fahrenheit and Celsius
fn main() {

    loop {
        clear_screen();
        let option:u32 = match start_prompt().trim().parse() {
            Ok(option) => option,
            Err(_) => continue
        };

        if option == 1 {
            convert(true);
        } else if option == 2 {
            convert(false);
        } else if option == 0 {
            break;
        } else {
            continue;
        }
    }
}


fn convert(is_celsius_to_fahrenheit: bool) {
    let mut input_value = String::new();
    let input_unit = if is_celsius_to_fahrenheit { "°C" } else { "°F" };
    let output_unit = if !is_celsius_to_fahrenheit { "°C" } else { "°F" };
    
    loop{
        clear_screen();
        println!("Write the value in {input_unit} you want to convert.");
        print!("> ");
        io::stdout()
            .flush()
            .unwrap();

        io::stdin()
            .read_line(&mut input_value)
            .expect("failed to read line");
    
        let input_value: f64 = match input_value.trim().parse() {
            Ok(input_value) => input_value,
            Err(_) => {
                input_value.clear();
                continue
            }
        };

        let output_value: f64 = if is_celsius_to_fahrenheit 
            { (9.0 / 5.0) * input_value + 32.0 }
            else { (5.0 / 9.0) * (input_value - 32.0) };
        
        
        println!("> {input_value:.1}{input_unit} is equal to {output_value:.1}{output_unit}");
        pause();
        break;
    };
}


fn start_prompt() -> String{
    let mut option = String::new();

    println!("=============================");
    println!("=    celsius_fahrenheit     =");
    println!("=                           =");
    println!("=  please choose an option  =");
    println!("=                           =");
    println!("=  1-celsius to fahrenheit  =");
    println!("=  2-fahrenheit to celsius  =");
    println!("=  0-exit                   =");
    println!("=============================");
    print!("> ");


    io::stdout()
        .flush()
        .unwrap();
    
    io::stdin()
        .read_line(&mut option)
        .expect("failed to read line");
    
    option
}