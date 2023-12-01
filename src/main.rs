fn main(){

    println!("\n");
    // Memory Heavy Clone Method
    clone_method();
    println!("\n");
    
    // Copy Trait available only for types using stack, or having known size at compile time
    stack_copy_trait();
    println!("\n");

    // Function Scoping

    let s = String::from("Saksham");
    takes_ownership(s);

    let x = 5;
    makes_copy(x);
    println!("Original x(in main):{}",x);
    println!("\n");

    // Return Values and Scope

    let _s1 = gives_ownership();
    
    let _s2 = String::from("Saksham");

    let _s3 = takes_and_gives_back(_s2);

    println!("String 1:{} \nString 3:{}",_s1,_s3);


}

fn clone_method(){
    let s1 = String::from("Hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
}

fn stack_copy_trait(){
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);
}

fn takes_ownership(some_string:String){
    println!("{}", some_string);
}

fn makes_copy(some_int:i32){
    println!("{}", some_int);
}

// Return Values and Scope
fn gives_ownership() -> String{
    let some_str =String::from("Aloo");

    some_str
}

fn takes_and_gives_back(a_string:String) -> String{
    a_string
}