use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::result::Result;
use std::error::Error;

// Box<dyn Error>はトレイトオブジェクト
// Errorトレイトを実装しているオブジェクトであればなんでも返せるため、
// エラー時の戻り値を柔軟に出来る
pub fn run(config: GrepConfg)-> Result<(), Box<dyn Error>> {
    let mut reader = create_file_reader(&config.filename);
    // stdout.lock()・・stdoutのロックをloopの前に１度だけ取ることで速度向上
    // BufWriter::new()・・標準出力への書き込みをメモリ内にバッファリングしてI/Oの頻度を抑える
    let stdout = io::stdout();
    let mut stdout_writer = BufWriter::new(stdout.lock());
    let mut search_target_text: String = String::new();
    loop {
        let num: usize = reader.read_line(&mut search_target_text)?;
        let res = search_target_text.find(&config.query);
        match res {
            Some(_) => {
                let display_text: String = search_target_text.replace(&config.query, &format!("\x1b[31m{}\x1b[37m", config.query)).replace("\n", "");
                writeln!(stdout_writer, "{}", display_text)?;
                // println!は毎回stdoutのロックを取っているため遅い
                // println!("{}", replaced_buf);
            },
            None => ()
        }
        search_target_text.clear();
        if num == 0 {
            break;
        }
    }
    Ok(())
}

fn create_file_reader(filename: &str) -> BufReader<File> {
    let file = File::open(filename).expect("file not found");
    BufReader::new(file)
}

pub struct GrepConfg {
    query: String,
    filename: String
}

impl GrepConfg {
    // この関数内で生成された文字列の参照である&strをreturnする
    // ライフタイムを指定しないと、スコープを抜けた際にダングリング参照になる可能性がある
    // 'staticが無いと「この関数の戻り値の型には、ライフタイムが省略された借用値が含まれていますが、ライフタイムは引数から導出できません」というエラーが出る
    pub fn new(args: Vec<String>) -> Result<GrepConfg, &'static str> {
        if args.len() < 3 {
            return Err("引数が足りません")
        }
        Ok(GrepConfg {
            // .cloneは新しいメモリ領域にコピーを生成するため、
            // 参照を保持するよりもメモリと時間を食う
            // ただ、参照を保持する場合ライフタイムの設定が必要なので、
            // それが無い分コードの見通しは良くなる
            query: args[1].clone(),
            filename: args[2].clone()
        })
    }
}
