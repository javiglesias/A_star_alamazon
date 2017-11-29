mod tests {
    #[test]
    fn it_works() {
    }
}
use std::cell::Cell;
#[derive(Debug)]
pub struct node {
    /*this is the general struct for the nodes of the expansion tree of the A_star algorithm.
    In my case i have to have 4 possible options of childrend, 
    ONE: go back in the hall
    TWO: go forward
    THREE: go left
    FOUR: go right.
    every1 of the options hjave to be a pointer of the type node.
    And also the child have to have a pointer to it parent.*/
    pub g:i32,
    pub h:i32,
    pub f:i32,
}
impl node {
    pub fn calculatef(mut self)->i32 {
        self.f = self.g + self.h;
        return self.f;
    }
    pub fn updateH(mut self) {
        self.h = self.h+1;
    }
}