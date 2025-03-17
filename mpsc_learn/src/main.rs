use tokio::sync::mpsc;
#[tokio::main]
async fn main() {
    //Create a channel with a buffer size of 10

    let (tx ,mut rx)   = mpsc::channel::<i32>(10);

    let tx2 = tx.clone();
    
    // Task 1: Send numbers 1 to 5
    tokio::spawn(async move {
        for i in 1..=5{
            tx.send(i).await.unwrap();
            println!("Task 1 sent: {}", i);
        }
    });
    
    // Task 2: Send numbers 6 to 10
    tokio::spawn(async move{
        
        for i in 6..=10{
            tx2.send(i).await.unwrap();
            println!("Task 2 sent: {}", i);
        }
    });

    while let Some(num) = rx.recv().await{
        println!("Received: {}", num);
    }

    let (tx, mut rx) = mpsc::channel::<String>(5);
    let tx2 = tx.clone();

    tokio::spawn(async move{
        tx.send("Hello from Task1".to_string()).await.unwrap();
    });

    tokio::spawn(async move{
        tx2.send("Konichiwa from Task2".to_string()).await.unwrap();
    });

    while let Some(msg) = rx.recv().await{
        println!("Got message : {}",msg);
    }
}
