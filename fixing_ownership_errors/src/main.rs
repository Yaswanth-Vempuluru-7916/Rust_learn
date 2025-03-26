use std::rc::Rc;

/*
fn return_a_string() -> &String{
    let s = String::from("Hello World");
    &s
}
*/

fn return_a_string() -> String{
    let s = String::from("Hello World");
    s
}

// * 2nd way Another possibility is to return a string literal, which lives forever (indicated by 'static).
//* This solution applies if we never intend to change the string, and then a heap allocation is unnecessary:

fn return_a_static_string()->&'static str {
    "Hello World"
}

//* 3rd  Use a Reference-Counted Pointer (Rc<String>) */
fn return_a_rc_refernce_pointer()-> Rc<String>{
    let s = Rc::new(String::from("Hello World"));
    Rc::clone(&s)
}

fn main(){
    let my_string = return_a_rc_refernce_pointer();
    println!("{}",my_string);
}
