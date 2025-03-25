fn first_or<'a>(strings: &'a Vec<String>, default: &'a String) -> &'a String {
    if !strings.is_empty() {
        &strings[0] // Return a reference to the first string
    } else {
        default // Return the default reference
    }
}
/*
fn first_or(strings: &Vec<String>, default: &String) -> &String {
    //|1 | fn first_or<'a>(strings: &'a Vec<String>, default: &'a String) -> &'a String     
    let s_ref = &strings[0]; // Reference to the first string
    s_ref // Return the reference
}
*/

fn main(){
    let strings = vec![];
    let default = String::from("default");

    let s = first_or(&strings,&default);
    println!("{:?}", s); // Use `s`
    drop(default);
}

/*
fn main() {
    let s = String::from("Hello World!!!");
    let s_ref = &s;
    
    drop(s);
    println!("{:?}",s_ref);
}

*/