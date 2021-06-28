use std::env;
use std::process;
use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::result::Result;
use std::error::Error;

pub fn grep_fast() {
    // env::args()で、コマンドラインから入力された引数を取得
    // args()で取得できるArgsはIteratorトレイトを実装している
    // .collectでiteratorをcollectionに変換する
    // .collectの戻り値はFromIteratorトレイトを実装している必要がある(Vec<T>はFromIteratorを実装している=iteratorから変換可能)
    // args()は不正なUnicodeを含んでいた場合panicを起こす
    // 不正なUnicodeを受け入れる必要がある場合は,args_os()を使う
    let args: Vec<String> = env::args().collect();
    // Resultのunwrap_or_else()でpanic以外の独自エラーが発生した際の処理をクロージャで定義できる
    let config: GrepConfg = GrepConfg::new(args).unwrap_or_else(|err| {
        eprintln!("引数解析時に問題発生： {}", err);
        // non zero codeを指定することでエラーであることを通知
        process::exit(1);
    });

    // env::var()はResultを返すが、.is_err()でbool値に変換している
    // 環境変数が設定されていなければ.is_err()はtrueを、設定されていればfalseを返す
    let is_sensitive: bool = env::var("IS_INSENSITIVE").is_err();

    if is_sensitive == true {
        // unwrapしたい値を返さないのでunwrap_or_elseではなく、if let構文を使う
        if let Err(e) = run_read_buf(config) {
            eprintln!("検索時エラー: {}", e);
            process::exit(1);
        }
    } else {
        // unwrapしたい値を返さないのでunwrap_or_elseではなく、if let構文を使う
        if let Err(e) = run_read_all(config) {
            eprintln!("検索時エラー: {}", e);
            process::exit(1);
        }
    }
}

// Box<dyn Error>はトレイトオブジェクト
// Errorトレイトを実装しているオブジェクトであればなんでも返せるため、
// エラー時の戻り値を柔軟に出来る
#[allow(dead_code)]
fn run_read_buf(config: GrepConfg)-> Result<(), Box<dyn Error>> {
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

fn run_read_all(config: GrepConfg)-> Result<(), Box<dyn Error>> {
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    for line in search(&config.query, &contents) {
        println!("{}", line);
    }

    Ok(())
}

fn search(query: &str, contents: &str) -> Vec<String> {
    let mut results = Vec::new();
    // 検索文字列と検索対象を共に小文字に統一してから検索
    let query = query.to_lowercase();
    for line in contents.lines() {
        // to_lowercaseは新しいStringを生成し返す
        // lineはto_lowercaseに借用される
        if line.to_lowercase().contains(&query) {
            results.push(line.replace(&query, &format!("\x1b[31m{}\x1b[37m", query)).replace("\n", ""));
        }
    }

    results
}

#[allow(dead_code)]
fn create_file_reader(filename: &str) -> BufReader<File> {
    let file = File::open(filename).expect("file not found");
    BufReader::new(file)
}

struct GrepConfg {
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
