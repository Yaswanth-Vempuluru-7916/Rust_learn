// fn main(){
//     // let mut v = vec![1,2,3];
//     // let num = &v[2];
//     // println!("Third element is {}", *num);
//     // v.push(4);

//     // println!("Third element is {}", *num);

//     // let x = 0;
//     // let mut x_ref = &x;
//     // *x_ref+=1;
//     // //!!x_ref has the W permission, while *x_ref does not

//     //** immutable reference removes WRITE permission from the parent but READ intact */

//     let mut v = vec![1,2,3,4]; // R W O all gone
//     let num = &mut v[2];
//     // num --> R O
//     // *num -> R W 

//     let num2 = &*num;
//     // num2 --> R    *num -->R
//     // 
//     println!("{} {}", *num, *num2);


// }

fn main(){
    let mut v = vec!['a','b','c'];
    ascii_capitalize(&mut v);
}
fn ascii_capitalize(v: &mut Vec<char>) {
    let c = &v[0];
    if c.is_ascii_lowercase() {
        let up = c.to_ascii_uppercase();
        v[0] = up;

        // variable c has a different lifetime in each branch of the if-statement.
        // In the then-block, c is used in the expression c.to_ascii_uppercase(). 
        // Therefore *v does not regain the W permission until after that line.
    } else {
        println!("Already capitalized: {:?}", v);
    }
}










// //!Pointer Safety Principle: data should never be aliased and mutated at the same time.

// fn main(){

//     let mut v = vec![1,2,3];
//     let num = &v[1];
//     v.push(4);
//     println!("Third element is {}", *num);

// }


// fn main(){
//     let x = Box::new(-1);
//     let x_abs1 = i32::abs(*x);
//     let x_abs2 = x.abs();
//     assert_eq!(x_abs1,x_abs2);
    
//     let r = &x;
//     let r_abs1 = i32::abs(**r);
//     let r_abs2 = r.abs();
//     assert_eq!(r_abs1,r_abs2);

//     let s = String::from("Hello");
//     let s_len1 = str::len(&s);
//     let s_len2 = s.len(); //*. If you call len on an owned String, then Rust will insert a single borrowing operator.
//     assert_eq!(s_len1, s_len2);

// }


// fn main() {
//     let x : Box<i32> = Box::new(42);

//     let r2 : &i32 = &*x;
//     let r3 : i32 = *x; //* Here i32 implements copy trait its ok  */
//     //*x = i32 taking reference now &(*x) -> &i32 */

//     println!("Value via r2: {}", *r2);
//     println!("Value via r2: {}", r3);
//     println!("Value via x: {}", x);

//     // let x = Box::new(String::from("hello"));
//     // let r2 = *x;
//     // println!("Value via r2: {}", r2);
//     // println!("Value via r2: {}", x);
//     //                               ^ value borrowed here after move
// }
