use std::io;
#[macro_use] extern crate scan_fmt;

fn main() {
    loop{
        
        println!("What temperature would you like to convert?/n Use format 32F or 100C");
    
        let mut temp = String::new();
    
        io::stdin()
            .read_line(&mut temp)
            .expect("Failed to read line");
    
        let Ok((temp, scale)) = scan_fmt!(&temp, "{/[-.0-9]+/}{[A-Za-z]}", f32, char) 
            else { println!("Invalid input"); 
            return; };

        println!("Your temp was: {}{}", temp,scale);
    
        if scale == 'F' || scale == 'f' {
            
            let temp = (temp - 32.0) * 5.0 / 9.0;
            println!("Your converted temp is: {}C", temp);
            break;
        } else if scale == 'C' || scale == 'c' {
            let temp = temp * 9.0 / 5.0 + 32.0;
            println!("Your converted temp is: {}F", temp);
            break;
        } else {
            println!("Invalid scale, use F or C, bro");
            continue;
        }    
    }
    
}