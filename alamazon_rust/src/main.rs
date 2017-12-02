use std::io;
use std::cell::Cell;
mod lib;
use std::default;
fn read_number() -> u32{
    let mut tmp:String = String::new();
    io::stdin().read_line(&mut tmp).expect("failed to read input.");
    let tmp2 = tmp.trim();
    match tmp2.parse::<u32>() {
        Ok(i) => return (i),
        Err(..) => return 0,
    };
}
fn a_star() {
    println!("calling a_star...");
}
fn start_db() {
    let mut db:[lib::object;14] = Default::default();
    db[0].name = String::from("Patatas");
    db[0].quantity = 200;
    db[1].name = String::from("Melones");
    db[1].quantity = 100;
    db[2].name = String::from("Boligrafos");
    db[2].quantity = 500;
    db[3].name = String::from("Boligrafos");
    db[3].quantity = 400;
    db[4].name = String::from("Melocotones");
    db[4].quantity = 200;
    db[5].name = String::from("Berzas");
    db[5].quantity = 100;
    db[6].name = String::from("Papeles");
    db[6].quantity = 500;
    db[7].name = String::from("Boligrafos");
    db[7].quantity = 400;
    db[8].name = String::from("Patatas");
    db[8].quantity = 200;
    db[9].name = String::from("Melones");
    db[9].quantity = 100;
    db[10].name = String::from("Plumas");
    db[10].quantity = 500;
    db[11].name = String::from("Plumas");
    db[11].quantity = 400;
    db[12].name = String::from("Colonias");
    db[12].quantity = 150;
    db[13].name = String::from("Ratones");
    db[13].quantity = 210;
    println!("initialized: {} with {}",db[0].name, db[0].quantity);
    println!("initialized: {} with {}",db[10].name, db[10].quantity);
    println!("initialized: {} with {}",db[12].name, db[12].quantity);
}
fn main() {
    let mut objects: [String; 3] = Default::default();
    let mut quantity: [u32;3] = Default::default();
    let mut tmp:String = String::new();
    let mut c=0;
   /* loop {
        if(c == 3) {
            break;
        }else {
            println!("Introduce object {} name: ",c);
            io::stdin().read_line(&mut objects[c]);
             println!("Introduce object {} quatity: ",c);
            //io::stdin().read_line(&mut tmp).expect("Error i32 input");
            quantity[c] = read_number();
            // tmp.parse::<i32>().unwrap_or(200); //FAIL!
            c = c+1;
        }
    }*/
    a_star();
    start_db();
}