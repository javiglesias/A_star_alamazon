use std::io;
use std::cell::Cell;
mod lib;
fn main() {
    let mut objects: [String; 3] = Default::default();
    let mut quantity: [String;3] = Default::default();
    let mut c=0;
    loop {
        if(c == 3) {
            break;
        }else {
            println!("Introduce object {} name: ",c);
            io::stdin().read_line(&mut objects[c]);
             println!("Introduce object {} quatity: ",c);
            io::stdin().read_line(&mut quantity[c]);
            c = c+1;
        }
    }
    a_star(objects, quantity);
    fn start_db() {

    }
    fn a_star(mut o:[String; 3], mut n:[String;3]) {
        println!("calling a_star...");
        let mut node_1 = lib::node{g:0, h:10, f:0};
        //let node_2 = lib::node{g:1, h:10};
        println!("f1: {}",node_1.calculatef());
        node_1.updateH();
       // println!("f1: {}",node_1.calculatef());
    }
}