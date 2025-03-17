use std::io;

fn fibonacci(n: i32)->i32{
    if n==0 || n==1{
        return n;
    }
    return fibonacci(n-1)+fibonacci(n-2);
}
fn main() {
    //Celsius to F
    //F = (9/5)*C + 32
    
    println!("Enter the Celsius temp : ");
    let mut celsius = String::new();

    io::stdin()
        .read_line(&mut celsius)
        .expect("Failed to read line");

    let celsius : f64 = match celsius.trim().parse() {
        Ok(num)=>num,
        Err(_)=> {
            println!("Invalid input. Please enter a valid number.");
            return;
        }
    };

    let fahrenheit = ((9.0/5.0) * celsius) +32.0;
    println!("{:.2} C = {:.2}f\n",celsius,fahrenheit);

    println!("Enter which fibonacci number you want : ");
    let mut n = String::new();
    io::stdin()
    .read_line(&mut n)
    .expect("Unable to read line");

    let n : i32 = match n.trim().parse() {
        Ok(num) if num>=0 => num,
       _=>{
            println!("Enter a valid number");
            return;
        }
    };

    let fibonacci_num = fibonacci(n);

    println!("The fibonacci result is : {fibonacci_num}");


    // let a = [10, 20, 30, 40, 50];
    // let mut index = 0;

    // while index < 5 {
    //     println!("the value is: {}", a[index]);

    //     index += 1;
    // }

    // for element in a {
    //     println!("the value is: {}", element);
    // }

    // for number in (1..4).rev(){
    //     println!("{number}!");
    // }
    // let condition = true;
    /*
        let number = if condition {"six"} else {5};

    3 |     let number = if condition {"six"} else {5};
      |                                -----        ^ expected `&str`, found integer
      |                                |
      |                                expected because of this
           //// !Same type undali
        */
    // println!("The value of number is: {number}");

    // let mut counter = 0;

    // let result = loop {
    //     counter +=1;

    //     if counter ==10 {
    //         break counter*2
    //     }
    // };
    // println!("The result is {result}");

    // let mut cnt = 0;
    // 'counting_loop : loop {
    //     println!("Count : {cnt}");
    //     let mut remaining=10;

    //     loop{
    //         println!("remaining = {remaining}");

    //         if remaining==9{
    //             break;
    //         }

    //         if cnt ==2{
    //             break 'counting_loop;
    //         }

    //         remaining-=1;
    //     }
    //     cnt+=1;
    // }
    // println!("End count = {cnt}");
}
