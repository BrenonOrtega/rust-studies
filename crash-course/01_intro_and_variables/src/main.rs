use std::string;

fn main() {
    //Defining variables
    let bunnies = 5;
    let (bunnies, carrots) = (5, 10);
    println!("{} {}", bunnies, carrots);
    
    //mutable variables
    let mut a = 25;
    a = 333;

    const MY_STRING: &str = "hello";

    //Shadowing with scope
    let x = 99;
    {
        let x = 5;
        println!("{}", x);
    }

    println!("{}", x);
    

    //Exercise A 
    const STARTING_MISSILES: i32 = 8;
       let (mut missiles, ready) : (i32, i32) = (STARTING_MISSILES, 2);
    
    println!("Firing {} of my {} missiles", ready, missiles);
    println!("remaining missiles {}", missiles - ready);
}