extern crate lib_2048;

use lib_2048::data;
use std::io;
use std::io::BufRead;

fn main() {
    let mut field = data::Field::new(4);
    let mut new = true;
    loop {
        if ! field.insertable() {
            println!("You lost");
            break;
        } else {
            println!("================");
            if new {
                field.insert_random();
            } else {
                new = true;
            }
            field.print();
            let mut charc = String::new();
            let stdin = io::stdin();
            stdin.lock().read_line(&mut charc);
            match charc.trim() {
                "r" => field.swipe_right(),
                "l" => field.swipe_left(),
                "u" => field.swipe_up(),
                "d" => field.swipe_down(),
                _   => new = false,
            }
                
        }
    }   
}
