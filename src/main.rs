fn main(){


    // Memory Heavy Clone Method
    clone_method();
}

fn clone_method(){
    let s1 = String::from("Hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
}