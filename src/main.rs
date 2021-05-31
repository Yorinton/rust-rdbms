use std::env;
use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;

fn main() -> io::Result<()> {
    // env::args()で、コマンドラインから入力された引数を取得
    // args()で取得できるArgsはIteratorトレイトを実装している
    // .collectでiteratorをcollectionに変換する
    // .collectの戻り値はFromIteratorトレイトを実装している必要がある(Vec<T>はFromIteratorを実装している=iteratorから変換可能)
    // args()は不正なUnicodeを含んでいた場合panicを起こす
    // 不正なUnicodeを受け入れる必要がある場合は,args_os()を使う
    let args: Vec<String> = env::args().collect();

    let query: &str = &args[1];
    let filename: &str = &args[2];

    let file = File::open(filename)?;
    let mut buf: String = String::new();
    let mut reader = BufReader::new(file);

    loop {
        let num: usize = reader.read_line(&mut buf)?;
        let res = buf.find(query);
        match res {
            Some(_) => {
                let replaced_buf: String = buf.replace(query, &format!("\x1b[31m{}\x1b[37m", query)).replace("\n", "");
                println!("{}", replaced_buf);
            },
            None => ()
        }
        buf.clear();
        if num == 0 {
            break;
        }
    }
    
    // file.read_to_string(&mut buf)?;

    //let res = buf.find(query);
    // let res2: Vec<&str> = buf.matches(query).collect();
    // println!("検索結果：{:?}", res2);
    Ok(())
}