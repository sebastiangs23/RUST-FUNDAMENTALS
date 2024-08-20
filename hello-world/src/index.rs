fn main() {
    let name : &str = "Luciana";

    print!("{name}");
    aca();
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