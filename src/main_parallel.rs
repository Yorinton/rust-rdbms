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
}