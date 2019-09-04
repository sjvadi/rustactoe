mod display;
mod input_handling;
mod assets;
fn main(){
    let mut count: u8 = 0;
    let mut active_piece = assets::Type::Cross;
    let mut boa = assets::init_board(); //Creates an empty playing field / board
    //Game loop, refreshes every turn
    loop{  
        // display::draw(); Should draw the board
        count = count + 1;
        println!("{}", count); //prints the count even = cross odd = circle
        if is_even(&count){ //checks which type is
            active_piece = assets::Type::Cross;
        }
        boa.change(choice_decider(&get_input()), &active_piece);
        print_occ(&boa);
    }
}


fn is_even(n: &u8) -> bool{
    if n % 2 == 0 {
        return true
    }
    false
}

use std::io::{stdin, stdout, Write};

pub fn get_input() -> String{
    let mut s = String::new();
    println!("Enter something: ");
    let _ = stdout().flush();
    stdin().read_line(&mut s).expect("Incorrect");
    if let Some('\n')=s.chars().next_back() {
        s.pop();
    }
    if let Some('\r')=s.chars().next_back() {
        s.pop();
    }
    println!("You typed: {}",s);
    s
}

pub fn choice_decider(c: &String) -> assets::Place {
    match c.as_ref() {
        "a1" => assets::Place::A1,
        "a2" => assets::Place::A2,
        "a3" => assets::Place::A3,
        "b1" => assets::Place::B1,
        "b2" => assets::Place::B2,
        "b3" => assets::Place::B3,
        "c1" => assets::Place::C1,
        "c2" => assets::Place::C2,
        "c3" => assets::Place::C3,
        _ => assets::Place::NaN
    }
}

fn print_occ(b: &assets::Board){
    if b.A1 != assets::Type::Empty {
        println!("A1 is occ");
    }
    if b.A2 != assets::Type::Empty {
        println!("A2 is occ");
    }
    if b.A3 != assets::Type::Empty {
        println!("A3 is occ");
    }
    if b.B1 != assets::Type::Empty {
        println!("B1 is occ");
    }
    if b.B2 != assets::Type::Empty {
        println!("B2 is occ");
    }
    if b.B3 != assets::Type::Empty {
        println!("B3 is occ");
    }
    if b.C1 != assets::Type::Empty {
        println!("C1 is occ");
    }
    if b.C2 != assets::Type::Empty {
        println!("C2 is occ");
    }
    if b.C3 != assets::Type::Empty {
        println!("C3 is occ");
    }
}
