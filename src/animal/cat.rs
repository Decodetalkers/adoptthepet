use super::Animal;
pub trait Cat: Animal {
    fn strach(&mut self) {
        self.grow();
        println!("hit -1");
    }
}
