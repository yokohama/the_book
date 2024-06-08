pub trait PetShopBehavior {
    fn is_pet(&self) -> bool;
    fn price(&self) -> String;
}

pub struct PetSold;
impl PetShopBehavior for PetSold {
    fn is_pet(&self) -> bool {
        true
    }
    fn price(&self) -> String {
        "500円です".to_string()
    }
}

pub struct NotForSale;
impl PetShopBehavior for NotForSale {
    fn is_pet(&self) -> bool {
        false
    }
    fn price(&self) -> String {
        "売り物ではありません。野生です。".to_string()
    }
}

pub trait Action {
    fn run(&self) -> String;
    fn sound(&self) -> &str;
}

pub struct Dog {
    potential: u32,
}
impl Dog {
    pub fn new(potential: u32) -> Self {
        Self { potential }
    }
}
impl Action for Dog {
    fn run(&self) -> String {
        format!("Dog Run! {}", self.potential)
    }
    fn sound(&self) -> &str {
        "Wan! Wan!"
    }
}

pub struct Cat {
    potential: u32,
}
impl Cat {
    pub fn new(potential: u32) -> Self {
        Self { potential }
    }
}
impl Action for Cat {
    fn run(&self) -> String {
        format!("Cat Run! {}", self.potential)
    }
    fn sound(&self) -> &str {
        "Nyan! Nyan!"
    }
}

pub struct Animal {
    pub action: Box<dyn Action>,
    pub behavior: Box<dyn PetShopBehavior>,
}

impl Animal {
    pub fn new(
        action: Box<dyn Action>,
        behavior: Box<dyn PetShopBehavior>
    ) -> Self {
        Self { action, behavior }
    }

    pub fn run(&self) -> String {
        self.action.run()
    }

    pub fn sound(&self) -> &str {
        self.action.sound()
    }

    pub fn is_pect(&self) -> bool {
        self.behavior.is_pet()
    }

    pub fn price(&self) -> String {
        self.behavior.price()
    }
}
