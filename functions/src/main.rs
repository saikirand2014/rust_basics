fn main() {
    println!("Hello, world!");
    first_fun();
    second_fun(34);
    third_fun(22, String::from("Saikiran"));
}


fn first_fun(){
    println!("First function!");
}

fn second_fun(num:i32){
    println!("second function value passed: {}", num);
}

fn third_fun(age:i32, name: String){
    println!("My name is {} and my age {}", name, age);
}