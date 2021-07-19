use std::thread;
use std::sync::mpsc;

fn main() {
    // mpscはmultiple producer, single customerを表す
    // tx:送信側、rx:受信側・・多くの分野で伝統的に転送機と受信機にそれぞれ使用されている変数名
    let (tx, rx) = mpsc::channel::<u32>();

    // moveをつけてtxの所有権を奪わせることで、txがクロージャより先にdropされることを防ぐ
    // thread::spawnは新しいスレッドを生成する
    let handle = thread::spawn(move || {
        let val: u32 = 2022;
        // channelの転送機(Sender)を通じてデータを送信
        tx.send(val).unwrap();

        let received = rx.recv().unwrap();
        println!("{}", received);
    });

    handle.join().unwrap();
}