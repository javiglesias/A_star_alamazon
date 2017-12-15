mod tests {
    #[test]
    fn it_works() {
    }
}
use std::cell::Cell;
use std::boxed::Box;
use std::mem;
use std::ptr;
use std::option;
 /*In my case i have to have 4 possible options of childrend, 
    ONE: go back in the hall
    TWO: go forward
    THREE: go left
    FOUR: go right.
    every1 of the options hjave to be a pointer of the type node.
    And also the child have to have a pointer to it parent.*/
    /*we have 2 types of nodes
        1-> shelving_node: has 2 actions: get item, return to cross
        2-> cross_node: has 4 actions, description above.*/
#[derive(Debug,Default,Clone)]
pub struct object {
    pub name:String,
    pub quantity:u32,
}
#[derive(Clone, Debug, Copy)]
pub struct _shelving {
    /*this are the shelving nodes*/
    g:i32,
    h:i32,
    f:i32,
    pback:Option<Box<_shelving>>,
}
impl _shelving {
    /*here goes the funcion get, that catch the item from the shelving and put it in the 
    inventory.*/
    fn print(&self) { println!("shelving: {}",self.g); }
}
#[derive(Clone, Debug)]
pub  struct node {
    /*this is the general struct for the nodes of the expansion tree of the A_star algorithm.
   */
    g:i32,
    h:i32,
    f:i32,
    /*this is for the cross nodes*/
    pnext:Option<Box<node>>,
    plast:Option<Box<node>>,
    pright:Option<Box<node>>,
    pleft:Option<Box<node>>,
    sh_right:Option<Box<_shelving>>,
    sh_left:Option<Box<_shelving>>,
}
impl node {
    /*Here goes the functions that calculate the h, f and g of each node*/
    fn print(&self) { println!("node: {}",self.g); }
}
#[derive(Debug,Default,Clone)]
pub struct universe {
    pub sh:[_shelving;8], //number of shelvings in the map
    pub cr:[node; 6],//number of crosses in the map
} 