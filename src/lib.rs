pub struct Cat {
    pub name: String,
    pub food: String,
    pub age: u8,
}

pub struct Dog {
    pub name: String,
    pub food: String,
    pub age: u8,
}

pub trait Sound {
    fn get_name(&self) -> String;
    fn get_age(&self) -> u8;
    fn sound(&self);
    fn age(&self) {
        println!("{}'s age is {}",self.get_name(), self.get_age())
    }
    
}

// pub trait Aged {
    
// }

// impl<T: Aged> Sound for T {
//     fn age(&self) {
        
//     }

//     fn get_name(&self) -> String {
//         todo!()
//     }

//     fn get_age(&self) -> u8 {
//         todo!()
//     }

//     fn sound(&self) {
//         todo!()
//     }
// }

impl Dog {
    pub fn eats(&self) {
        println!("{} eats dogfood",self.name)
    }
}

impl Cat {
    pub fn eats(&self) {
        println!("{} eats catfood",self.name)
    }
}

impl Sound for Cat {
    fn sound(&self) {
        println!("{} Meows!",self.name);
    
    }

    fn get_name(&self) -> String {
        format!("{}",self.name)
    }

    fn get_age(&self) -> u8 {
        self.age
    }
    
}

impl Sound for Dog {
    fn sound(&self) {
        println!("{} oofs!",self.name);
    
    }

    fn get_name(&self) -> String {
        format!("{}",self.name)
    }

    fn get_age(&self) -> u8 {
        self.age
    }
        
}