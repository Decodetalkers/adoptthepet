pub mod cat;
pub mod dog;
pub trait Animal {
    fn grow(&mut self);
    fn eat(&mut self);
}

pub trait Attact {
    fn attacthuman(&mut self);
    fn angry(&mut self);
    fn happy(&mut self);
}
