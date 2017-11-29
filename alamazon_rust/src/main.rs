use std::io;
mod lib;
fn main() {
    let mut objects: [String; 3] = Default::default();
    let mut quatity: [String;3] = Default::default();
    let mut c=0;
    loop {
        if(c == 3) {
            break;
        }else {
            println!("Introduce object {} name: ",c);
            io::stdin().read_line(&mut objects[c]);
             println!("Introduce object {} quatity: ",c);
            io::stdin().read_line(&mut quatity[c]);
            c = c+1;
        }
    }
    fn a_star(mut o:[String; 3], mut n:[String;3]) {
        let mut n =0;
        println!("calling a_star: {}", n);
        let node_1 = lib::node{g:0, h:10};
        let node_2 = lib::node{g:1, h:10};
            if n==2 { //condicion de salida de recursion
                println!("f1: {}",node_1.f());
                return;
            }
            else {
                    n = n+1;
                    //a_star(n); //recursividad tocha
            }
    }
}