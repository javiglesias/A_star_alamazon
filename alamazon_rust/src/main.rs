use std::io;
mod lib;
fn main() {
    let mut option = String::new();
    println!("MENU");
    println!("1-Backtrack.");
    println!("2-primeroProfundidad");
    println!("3-primeroAmplitud");
    println!("4-A*");
    println!("5-MinMax");
    println!("6-Alfabeta");
    io::stdin().read_line(&mut option);
    match 4 {
        1 => {;}
        4 => {a_star(0);}
        _ => {;}
    }
    fn a_star(mut n:i32) {
        println!("calling a_star: {}", n);
        let node_1 = lib::node{g:0, h:10};
        let node_2 = lib::node{g:1, h:10};
       // loop {
            if n==2 { //condicion de salida de recursion
                println!("f1: {}",node_1.f());
                return;
            }
            else {
                    n = n+1;
                    a_star(n); //recursividad tocha
            }
        //}
    }
     io::stdin().read_line(&mut option);
}
//backtracking
//primeroProfundidad
//primeroAmplitud
//A*
//Minmax
//Alfabeta
//Refutación por resolucion