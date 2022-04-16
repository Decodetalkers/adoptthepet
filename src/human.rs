use crate::{
    animal::{Animal, Attact},
    adored::Adored,
};
pub mod boy;
pub mod girl;
pub trait Human {
    fn adorepet<T>(&mut self, pet: &mut T)
    where
        T: Adored + Animal;
    fn wry<T>(&mut self, pet: &mut T)
    where
        T: Attact + Animal;
    fn born(name: String) -> Self;
}
