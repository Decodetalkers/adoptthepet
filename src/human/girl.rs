use bevy::prelude::Component;

use super::Human;
#[derive(Component)]
pub struct Girl {
    pub hp: i32,
    name: String,
    pet: Option<String>,
}

impl Human for Girl {
    fn adorepet<T>(&mut self, pet: &mut T)
    where
        T: crate::adored::Adored + crate::animal::Animal,
    {
        self.pet = Some(pet.name());
        pet.changeadmin(self.name.clone());
        pet.grow();
    }
    fn wry<T>(&mut self, pet: &mut T)
    where
        T: crate::animal::Attact + crate::animal::Animal,
    {
        pet.happy();
        self.hp += 1;
    }
    fn born(name: String) -> Self {
        Girl {
            hp: 100,
            name,
            pet: None,
        }
    }
}
