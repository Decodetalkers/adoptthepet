use bevy::prelude::Component;

use crate::adored::Adored;
use crate::animal::{cat::Cat, Animal, Attact};
use std::default::Default;
#[derive(Component)]
pub struct Kitty {
    pub admin: String,
    age: i32,
    pub hp: i32,
    pub strangth: i32,
    name: String,
}
impl Animal for Kitty {
    fn grow(&mut self) {
        self.age += 1;
    }
    fn eat(&mut self) {
        println!("Miao");
    }
}
impl Cat for Kitty {}
impl Adored for Kitty {
    fn changeadmin(&mut self, admin: String) {
        self.admin = admin;
    }
    fn name(&self) -> String {
        self.name.clone()
    }
}
impl Attact for Kitty {
    fn attacthuman(&mut self) {
        self.hp -= 10;
    }
    fn angry(&mut self) {
        self.strangth += 1;
    }
    fn happy(&mut self) {
        self.strangth -= 1;
    }
}
impl Default for Kitty {
    fn default() -> Self {
        Kitty {
            admin: "Chen".to_string(),
            age: 0,
            hp: 100,
            strangth: 2,
            name: "Kitty".to_string(),
        }
    }
}
