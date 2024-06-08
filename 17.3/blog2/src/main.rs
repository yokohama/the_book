extern crate blog2;
use blog2::*;

fn main() {
    let dog = Animal::new(
        Box::new(Dog::new(10)),
        Box::new(PetSold));
    let cat = Animal::new(
        Box::new(Cat::new(5)),
        Box::new(NotForSale));

    let animals = vec![ dog, cat ];

    for animal in animals.iter() {
        println!("{}", animal.action.sound());
        println!("{}", animal.action.run());
        println!("{}", animal.behavior.is_pet());
        println!("{}", animal.behavior.price());
    }
}
