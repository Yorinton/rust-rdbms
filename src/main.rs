use std::env;
use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::io::{BufReader, BufWriter};

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

    // stdout.lock()・・stdoutのロックをloopの前に１度だけ取ることで速度向上
    // BufWriter::new()・・標準出力への書き込みをメモリ内にバッファリングしてI/Oの頻度を抑える
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());
    loop {
        let num: usize = reader.read_line(&mut buf)?;
        let res = buf.find(query);
        match res {
            Some(_) => {
                let replaced_buf: String = buf.replace(query, &format!("\x1b[31m{}\x1b[37m", query)).replace("\n", "");
                writeln!(out, "{}", replaced_buf).unwrap();
                // println!は毎回stdoutのロックを取っているため遅い
                // println!("{}", replaced_buf);
            },
            None => ()
        }
        buf.clear();
        if num == 0 {
            break;
        }
    }
    Ok(())
}