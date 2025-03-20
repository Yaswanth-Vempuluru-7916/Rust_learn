// //!Pointer Safety Principle: data should never be aliased and mutated at the same time.

fn main(){

    // let mut v = vec![1,2,3];
    // let num = &v[1];
    // v.push(4);
    // println!("Third element is {}", *num);
    
}


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
