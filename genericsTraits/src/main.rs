fn main() {
    println!("Hello, world!");


    create_person();


}

struct Person<PetType: Animal + NotDengerous, PetType2:> {
    first_name: String,
    pet: PetType,
    pet2:PetType2
}

trait Animal {
    fn make_sound(&self) -> ();
}

trait NotDengerous {}

struct Dog {}

impl NotDengerous for Dog {}

impl Animal for Dog {
    fn make_sound(&self) -> () {
        println!("Dog Barks!");
    }
}


struct Cat {}
impl NotDengerous for Cat {}
impl Animal for Cat {
    fn make_sound(&self) -> () {
        println!("Cat meows");
    }
 }

struct Bear {}

impl Animal for Bear {

    fn make_sound(&self) -> () {
        
        println!("Bear noo");
    }
}

struct Tiger {}
impl Animal for Tiger {
    
    fn make_sound(&self) -> () {
        println!("Tiger Rooaaar");
    }

}

fn create_person() {
    let pet1 = Dog{};
    let pet2 = Cat{};
    let pet3 = Bear{};
    let pet4 = Tiger{};

    let p2 = Person{
        first_name: "Raj".to_string(),
        pet:pet2,
    };

    p2.pet.make_sound();
    
}


