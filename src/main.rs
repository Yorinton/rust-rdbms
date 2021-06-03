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
    let config: GrepConfg = GrepConfg::new(args);

    let file = File::open(config.filename).expect("file not found");
    let mut buf: String = String::new();
    let mut reader = BufReader::new(file);

    // stdout.lock()・・stdoutのロックをloopの前に１度だけ取ることで速度向上
    // BufWriter::new()・・標準出力への書き込みをメモリ内にバッファリングしてI/Oの頻度を抑える
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());
    loop {
        let num: usize = reader.read_line(&mut buf)?;
        let res = buf.find(&config.query);
        match res {
            Some(_) => {
                let replaced_buf: String = buf.replace(&config.query, &format!("\x1b[31m{}\x1b[37m", config.query)).replace("\n", "");
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

struct GrepConfg {
    query: String,
    filename: String
}

impl GrepConfg {
    pub fn new(args: Vec<String>) -> Self {
        if args.len() < 3 {
            panic!("引数が足りません");
        }
        GrepConfg {
            // .cloneは新しいメモリ領域にコピーを生成するため、
            // 参照を保持するよりもメモリと時間を食う
            // ただ、参照を保持する場合ライフタイムの設定が必要なので、
            // それが無い分コードの見通しは良くなる
            query: args[1].clone(),
            filename: args[2].clone()
        }
    }
}
