use traits::{Cat,Dog, Sound};
fn main () {
    let puss = Cat {
        name: String::from("Puss"),
        food: String::from("catfood"),
        age: 3,
    };
    let bosco = Dog {
        name: String::from("Bosco"),
        food: String::from("dogfood"),
        age: 4,
    };
    puss.sound();
    puss.eats();
    bosco.sound();
    bosco.eats();

    println!("Bosco is {:?} yrs old", bosco.get_age());

}