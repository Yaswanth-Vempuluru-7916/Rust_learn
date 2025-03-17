use std::io;
fn main() {
   /*
    * Data Types --> // ! 1) Scalar  2)Compound
   1) Scalar  
        |->integers  i8 :  -(2 power (n-1)) to ((2 power (n-1)) - 1)  u : 0 to (2 power n) -1
        |->float
        |->Booleans
        |->Characters :  Rust’s char type is //!{four} bytes in size . => Unicode Scalar Value,
        it can represent a lot more than just ASCII. Accented letters; Chinese, Japanese, and Korean characters; emoji;
         and zero-width spaces are all valid char values in Rust. 
   
   1) Compound  
        |->Tuples --> fixed size . cant grow in size once declared
        |->Arrays
   */

   let a = ["Sung Jinwoo" , "Cha Hae-In", "Igris BloodRed","Bellion" , "Beru"];
   println!("Please enter the index : ");

   let mut index = String::new();
    io::stdin()
   .read_line(&mut index)
   .expect("Failed to read line");

    let index: usize = match index.trim().parse() {
        Ok(num) if num < a.len() =>num,
        _=>{ //❌ Handles both parsing errors & out-of-bounds cases
            //but Err doesnt handle out of bound errors
            println!("Invalid index! Please enter a number between 0 and {}", a.len() - 1);
            return;
        },
    };
    let element = a[index];
    println!("The value of the element at index {index} is: {element}");


// let tup : (u32 ,f64 , String) = (500 , 6.4 ,"Arise!!!".to_string());

// println!("The first value is : {:?}",tup.0);
// println!("The second value is : {:?}",tup.1);
// println!("The third value is : {:?}",tup.2);

// let a :[i32; 5] = [1, 2, 3, 4, 5];

// let a = [3; 5] ; // [3,3,3,3,3]

//   let x = 2.01;
//   println!("x is  : {x}");

//   let sum = 5 + 10;
//   let difference = 95.5 - 4.3;
//   let product = 4 * 30;

//   // division
//   let quotient = 56.7 / 32.2;
//   let truncated = -5 / 3; // Results in -1

//   // remainder
//   let remainder = 43 % 5;

//   println!("sum : {sum}\n difference : {difference}\n product : {product}\n quotient : {quotient}\n truncated : {truncated}\n remainder {remainder}");
//   let x: fsize = 2.0;
//   println!("{x}");
}
