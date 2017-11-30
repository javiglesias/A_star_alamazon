use std::io;
use std::cell::Cell;
mod lib;
use std::default;
fn a_star() {
    println!("calling a_star...");
}
fn start_db(mut o:[String; 3], mut n:[i32;3]) {
    let mut db:[lib::object;14] = Default::default();
    db[0].name = o[0].clone();
    db[0].quantity = n[0];
    db[1].name = o[1].clone();
    db[1].quantity = n[1];
    db[2].name = o[2].clone();
    db[2].quantity = n[2];
    println!("initialized: {} with {}",db[0].name, db[0].quantity);
    println!("initialized: {} with {}",db[1].name, db[1].quantity);
    println!("initialized: {} with {}",db[2].name, db[2].quantity);
}
fn main() {
    let mut objects: [String; 3] = Default::default();
    let mut quantity: [i32;3] = Default::default();
    let mut tmp:String = String::new();
    let mut c=0;
    loop {
        if(c == 3) {
            break;
        }else {
            println!("Introduce object {} name: ",c);
            io::stdin().read_line(&mut objects[c]);
             println!("Introduce object {} quatity: ",c);
            io::stdin().read_line(&mut tmp).expect("Error i32 input");
            quantity[c] = tmp.parse::<i32>().unwrap_or(200); //FAIL!
            c = c+1;
        }
    }
    a_star();
    start_db(objects, quantity);
}