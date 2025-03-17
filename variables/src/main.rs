// const THREE_HOURS_IN_SECONDS: i32 = 3*60*60;
fn main() {
    // let mut x =5;
    // println!("The value of x is : {}",x);
    // x=6;
    // println!("The value of x is : {}",x);
    // println!("The value of THREE_HOURS_IN_SECONDS is : {}",THREE_HOURS_IN_SECONDS);
    
    let x = 5;
    
    let x = x+1;
    {
        let x = x+2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is : {}",x);

    // let mut spaces = "   ";
    // spaces = spaces.len();

}
