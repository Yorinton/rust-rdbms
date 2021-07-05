use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        // 1 ~ 9
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // JoinHandleは、joinメソッドを呼び出した時に、スレッドでの処理の終了を待つ
    handle.join().unwrap();

    // 1 ~ 4
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    let v = vec![1, 2, 3];
    // moveキーワードをつけることで、クロージャにvの所有権を奪わせる
    // これにより、クロージャの生存期間より先にvがdropすることを防げる
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });
    // クロージャで借用したvを、
    // スレッドの処理実行が終わる前(.join()の前)にdropしている
    // drop(v);

    handle.join().unwrap();
}