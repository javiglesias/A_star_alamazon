mod tests {
    #[test]
    fn it_works() {
    }
}

#[derive(Debug)]
pub struct node {
    pub g:i32,
    pub h:i32,
}
impl node {
    pub fn f(self)->i32 {
        return self.g+self.h;
    }
}