fn main() {
    let name : &str = "Luciana";

    print!("{name}");
    aca();
    arrays();
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

    // print!("{:?}", array);
}   