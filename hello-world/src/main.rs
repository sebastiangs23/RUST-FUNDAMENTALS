fn main() {

    //Declare String
   let string_innmutable: &str = "sebastian";

   //Declare Numbers
   let mut  numerow : i64 = 123;
   numerow = numerow - 100;

   print!("My name is : {string_innmutable} and I have {numerow}");


    //Declare Floats
    let  decimalss : f64 = 105.556;

    print!("{decimalss}");

    //Declare Booleans
    let  maybe : bool = true;
    print!("{maybe}");

    //Declare const
    const LASTNAME: &str = "Gomez";

    print!("My lastname is: {LASTNAME}");
}
