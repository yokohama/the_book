enum AnimalKind {
    Dog,
    Cat
}

struct Animal {
    kind: AnimalKind,
    sound: String
}

fn main() {
    let dog = Animal {
        kind: AnimalKind::Dog,
        sound: String::from("ワンワン！")
    };

    let cat = Animal {
        kind: AnimalKind::Cat,
        sound: String::from("ニャ～ン")
    };

    sound(&dog);
    sound(&cat);
}

fn sound(animal: &Animal) {
    println!("{}", animal.sound);
}
