fn main() {
    println!("Variable in rust");
    let number1 = 10;
    println!("value: {}", number1);

    // mutable number
    let mut number2 = 20;
    println!("before change value: {}", number2);
    number2=30; 
    println!("after change value: {}", number2);
    
    let mut name = "String: saikiran";
    println!("before change name: {}", name);
    name="vinay";
    println!("after change name: {}", name);
}
