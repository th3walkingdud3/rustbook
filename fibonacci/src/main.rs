use std::io;

fn main() {
    loop {
        println!("Enter a number to calculate the Fibonacci number at that index: ");

        let mut sequence_index = String::new();

        io::stdin()
                .read_line(&mut sequence_index)
                .expect("Failed to read line");

        let Ok(sequence_index) = sequence_index.trim().parse()
            else { println!("Invalid input, please enter a number"); 
            continue; };

        let sub_constant = 5.0_f64.sqrt();

        let big_p_phi = (1.0 + sub_constant) / 2.0;
        let phi = (1.0 - sub_constant) / 2.0;

        let x = (big_p_phi.powf(sequence_index) - phi.powf(sequence_index)) / sub_constant;
        
        println!("The Fibonacci number at index {} is {}", sequence_index, x.round());

        break;
    }
}