use tokio::sync::mpsc;

#[tokio::main]
async fn main(){
    // 多生产者，单消费者
    let (tx, mut rx) = mpsc::channel(32);
    let tx2 = tx.clone();

    tokio::spawn(async move{
        tx.send("Send the first msg").await;
    });

    tokio::spawn(async move{
        tx2.send("Send the second msg").await;
    });

    while let Some(message) = rx.recv().await {
        println!("{}", message);
    }
}