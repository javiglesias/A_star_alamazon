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
#[derive(Clone, Debug)]
pub  struct node {
    /*this is the general struct for the nodes of the expansion tree of the A_star algorithm.
    In my case i have to have 4 possible options of childrend, 
    ONE: go back in the hall
    TWO: go forward
    THREE: go left
    FOUR: go right.
    every1 of the options hjave to be a pointer of the type node.
    And also the child have to have a pointer to it parent.*/
    /*we have 2 types of nodes
        1-> shelving_node: has 2 actions: get item, return to cross
        2-> cross_node: has 4 actions, description above.*/
    g:i32,
    h:i32,
    f:i32,
    pnext:Option<Box<node>>,
    plast:Option<Box<node>>,
    pright:Option<Box<node>>,
    pleft:Option<Box<node>>,
}
impl node {
    pub fn setg() {

    }
     pub fn seth() {

    }
    pub fn setf() {

    }
    pub fn calculatef(mut self)->i32 {
        self.f = self.g + self.h;
        return self.f;
    }
    pub  fn updateH(mut self) {
        self.h = self.h+1;
    }
}
#[derive(Default)]
pub struct object {
    pub name:String,
    pub quantity:u32,
}
pub struct state {
    position:node,
    inventory:[object; 3],
}
pub struct shelving {
    o:[String;4],
    //position:node,
}
struct cross {
    sh:[shelving;2],
   // c:[cross;2],
}
