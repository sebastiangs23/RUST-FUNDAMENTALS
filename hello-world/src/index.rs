fn main() {
    let name : &str = "Luciana";

    print!("{name}");
    aca();
    arrays();
    learning_structures();
}


fn aca(){
    let lastname : &str = "Gomez";
    print!("{lastname}");

    let numero : i64 = 100;

    if numero < 90 {
        print!("entro en la condicional:  {lastname}")
    }else {
        print!("No entro en la condicional")
    }
}

fn arrays(){
    let array : Vec<&str> = vec!["the", "sintax", "is", "very", "weird"];
    let people : Vec<&str> = vec!["pepito","juanita", "rosita"];

    for element in array {
        print!("the value is: {element}")
    }

    let mut ages : i64 = 25;
    for element in people {
        print!("the names are {element}, and his ages are {ages} ");
        ages = ages + 1;
    }

}   

fn learning_structures() {
    let new_structure:Persona = Persona::new("sebas", 23, 1.71); 
    print!("{} {} {}", new_structure.name, new_structure.age, new_structure.height);
}

// En rust no hay class , son 'struct'
struct Persona{
    name : String,
    age : i32,
    height: f32,
}

// En rust no hay constructor, son 'impl'
impl Persona{
    fn new(name: &str, age: i32, height: f32) -> Persona {
        Persona {
            name: String::from(name),
            age,
            height
        }
    }
}