mod lib;
use std::env;
use std::process;

use lib::GrepConfg;

fn main() {
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
        if let Err(e) = lib::run_read_buf(config) {
            eprintln!("検索時エラー: {}", e);
            process::exit(1);
        }
    } else {
        // unwrapしたい値を返さないのでunwrap_or_elseではなく、if let構文を使う
        if let Err(e) = lib::run_read_all(config) {
            eprintln!("検索時エラー: {}", e);
            process::exit(1);
        }
    }
}