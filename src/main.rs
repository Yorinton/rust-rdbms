// Listは再帰的な列挙子を含んでいる = Listは自身の別の値を保持している
// この場合、コンパイラはこの型の値を格納するのに必要な領域を計算できない
// List型の値の格納に必要なメモリ領域を計算する際に、まずはConsを見てi32とListを1つずつ格納出来るメモリ領域が必要だと判断する
// 次にそのListの格納に必要なメモリ領域を計算するために、またConsを見にいく、というように無限に続いてしまう
// List型を参照にすることで必要なサイズを確定できる。なぜならポインタの格納に必要なサイズは決まっているから
// enumなので実際の値はCons(i32, Box<List>)かNilのどちらかになる
#[derive(Debug)] // std::fmt::Debugを実装？
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use List::{Cons, Nil};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::rc::Rc;
use std::cell::RefCell;
use std::any::{type_name};
use std::ops::Add;
use std::cmp::{PartialOrd};

// 異なるモジュールから同名の要素(structなど)をimportすることは出来ない
// RustがどちらのResultを使っているか分からないから
/*
use std::io::Result;
use std::fmt::Result;
*/
// 以下のようにimportして、
// io::Result、fmt::Resultのように使う
#[allow(unused_imports)]
use std::io::{self, Seek}; // std::ioとstd::io::Seekをこのスコープに取り込みたい場合の書き方(selfを使う)

// asを使ってエイリアスを設定することも可能
#[allow(unused_imports)]
use std::fmt as FmtResult;

// globパターン
// パッケージの全ての公開要素を取り込む
// 該当の要素がlocalで定義されたものか、useで外部から取り込んだものか分かりづらくなる
// テストで使うことが多い
#[allow(unused_imports)]
use std::fs::*;

// src直下のファイル名(拡張子抜き)をモジュール名としてimportできる
mod summary;
mod back_of_house;
// mod xxxxでモジュールをimportして、useでモジュール内の要素(structやenumなど)をimportできる
use summary::{Tweet, Summary};
use back_of_house::{BreakFast, Language};

// 相対パス：呼び出し側と定義側を一緒に移動する可能性が高いならこっち
// use lib::traits::Summary;

// 構造体やenumをimportする時は関数とは違いフルパスで書くのが慣習
// use lib::back_of_house::{BreakFast, Language};

enum MyEnum {
    Variant1,
    Variant2(u32, u32),
    Variant3 { x: u8, y: u8 }
}

fn en(val: MyEnum) -> () {
    match val {
        MyEnum::Variant1 => println!("{}", "this is variant1"),
        MyEnum::Variant2(x, y) => println!("this is variant2 {},{}", x, y),
        MyEnum::Variant3{ x, y } => println!("this is variant3 {},{}", x, y),
    }
}

// 外部パッケージをuse
// cargo add randでCargo.tomlに追加した上でuse
// cargo build か cargo runでrandも一緒にコンパイルされる
use rand::{thread_rng, Rng};
fn foo<R: Rng + ?Sized>(rng: &mut R) -> f32 {
    rng.gen()
}

#[allow(unused_variables)]
fn main() {
    let mut rng = thread_rng();
    let rng_value = foo(&mut rng);
    println!("{}", &rng_value);

    // 関数をimportする際はuseでモジュールだけimportする
    // 使う時はモジュール::関数()のようにして使う
    // こうすることで、localで定義した関数なのかモジュールからimportした関数なのか明確になる
    // このやり方がRustの慣習
    let val = back_of_house::sample_function();

    let lang = "ja";
    use std::str::FromStr;
    let langage_code = Language::from_str(lang);
    // @TODO unwrapを使ったらErrの場合にpanicになるので良くない
    match langage_code.unwrap() {
        Language::Japanese => println!("{}", "日本語！"),
        Language::English => println!("{}", "英語！"),
    }

    let toast = "france";
    let mut bf = BreakFast::summer(toast);
    // bf自体がmutableであり、toastプロパティはpublicなので変更可能
    bf.toast = String::from("chocolate");
    // bf自体がmutableだけど、seasonal_fruitはprivateなので参照も修正も出来ずコンパイルエラーになる
    // bf.seasonal_fruit = String::from("peach");

    let localhost_v4 = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let localhost_v6 = IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1));
    println!("{}", localhost_v4);
    println!("{}", localhost_v6);

    let list;

    {
        // Box<T>はデータをヒープ領域に置きつつ、参照として機能する
        // Box<T>の値がスコープを抜けるとボックスが参照しているヒープ領域のデータもdropされる
        // Rc<T>は複数の所有者が許容されるため全て不変参照になる
        list = Rc::new(Cons(1, Rc::new(Cons(2, Rc::new(Cons(3, Rc::new(Nil)))))));
        // Rc::cloneはディープコピーではなく参照カウントをインクリメントするだけなのでパフォーマンス上の問題にはならない
        // ディープコピーはデータ量によってはパフォーマンス上のボトルネックになりうる
        // println!("2, {}", Rc::strong_count(&list));
        // let b = Cons(4, Rc::clone(&list));
        // let c = Cons(5, Rc::clone(&list));
        // println!("3, {}", Rc::strong_count(&list));
        // println!("4, {}", Rc::strong_count(&list));
        // let d = Cons(5, Rc::clone(&list));
        // let e = Cons(5, Rc::clone(&list));
        // println!("5, {}", Rc::strong_count(&list));
    }
    println!("6, {}", Rc::strong_count(&list));

    println!("{:p}", &list);
    println!("{:?}", list);

    let s = String::from("Please don't forget it");
    let s1 = &s; // sの参照をs1に借用
    // println!("ここに、{}を出力", s1);
    // println!("ここに、{}を出力", s); // 所有権をmoveしている訳では無いのでここでもsを扱える

    let my_enum_value_3 = MyEnum::Variant3 {
        x: 100, y: 200
    };
    let my_enum_value_1 = MyEnum::Variant1;
    let my_enum_value_2 = MyEnum::Variant2(11, 22);
    en(my_enum_value_1);
    en(my_enum_value_2);
    en(my_enum_value_3);

    // 「文字列スライス」：文字列の一部への参照
    // 0番目から5番目の文字列への参照
    // コンピューターで扱えるデータの最小単位は1byte = 8bit
    // 1byte = 8bitは0 ~ 255
    // ASCII文字は1byteの範囲(0~127)で扱える
    // ASCII以外の文字は128 ~ 255を組み合わせて作られる(マルチバイト文字)
    // UTF-8の場合、マルチバイト文字は2 ~ 4byteの可変長
    // UTF-8の文字境界以外のところで区切った場合はエラーになる
    // let string_slice = &s1[..6];
    // println!("{}", string_slice);

    let word = first_word(&s1[..]);
    // s1.clear(); // s1の一部への参照が存在する(word)のでs1をclearしようとするエラーになる
    println!("{}", word);


    // &[u8; 5]：u8のスライスへの参照
    // let x: &[u8; 5] = b"hello"; // バイトリテラル型. b"hello"は参照
    //println!("b\"hello\" {:#?}", x);

    // xはスコープを抜ける際にdropされる
    // そのため、スコープの外でxの参照を使うことは出来ない
    // let r;
    // {
    //     let x = 5;
    //     r = &x;
    // }
    // println!("{}", r);

    // a, b共に文字列リテラルなのでスタック領域に保持される
    // スコープを抜けるタイミングでdropされることがない
    // そのためライフタイムを意識しなくていい？ = ライフタイムで決定したスコープ外で参照してもコンパイルエラーにならない
    let a: &str = "good night";
    let ret: &str;
    {
        let b: &str = "good afternoon";
        ret = longest(a, b);
    }
    println!("longest sentence is {}", ret);


    // String::fromで生成した文字列はヒープ領域に割り当てられる
    // スコープを抜けるタイミングでdropされるのでライフタイムを意識する必要がある
    // ライフタイムで決定したスコープ外で参照するとコンパイルエラーになる
    let x: String = String::from("hello world");
    let result: &str;
    {
        let y: String = String::from("good morning");
        result = longest(x.as_str(), y.as_str());
        println!("longest sentence is {}", result);
        let mut add: String = result.to_string();
        add.push_str(" and evening and ホゲホゲ");
        println!("add str: {}", add);
    }
    //println!("longest sentence is {}", result);
    println!("{}", hoge());

    // "Katsu"はヒープに格納される
    // bは値"Katsu"を所有しているため、スコープを抜ける際にメモリが解放される
    // 解放されるのはスタックに格納されているBoxとヒープに格納されている"Katsu"のデータ
    // コンパイル時にコンパイラが知っておかねばならないのは、ある型が占有する領域の大きさ
    // 再帰的な型はコンパイル時にサイズが分からない
    let b: Box<&str> = Box::new("Katsu");
    println!("{}", b);

    // RefCell<T>で内部可変化する
    // imutableで定義しても可変参照を持てるようになる
    // 特定の関数内でのみ可変にしたい場合などに使える
    let msgs = Rc::new(RefCell::new(vec![]));
    add_message(&msgs, String::from("hello"));

    msgs.borrow_mut().push(String::from("bbbb"));

    let num = Rc::new(RefCell::new(11));
    // core::cell::Ref<i32>
    println!("{:#}", type_of(num.borrow()));
    // i32
    println!("{:#}", type_of(*num.borrow()));
    // *で参照外しを行うことでi32として演算が可能になる
    *num.borrow_mut() += 10;
    
    // RefMut<T>に対する複数の可変参照を持っているため実行時エラーになる
    // let mut msg2 = msgs.borrow_mut();
    // let mut msg3 = msgs.borrow_mut();
    // msg2.push(String::from("aaaa"));
    // msg3.push(String::from("bbbb"));

    // .borrow()で不変参照を得る
    println!("{:#?}", msgs.borrow());


    struct Sample<T> {
        x: T,
        y: T
    }

    // TがAdd<Output = T>トレイトとCopyトレイトを実装している場合のみ
    // addメソッドを実装する
    // それ以外の型の場合はaddメソッドはない
    impl<T: Add<Output = T> + Copy> Sample<T> {
        fn add(&self) -> T {
            self.x + self.y
        }
    }
    let sample = Sample {
        x: 22.5,
        y: 33.5,
    };
    println!("{}", sample.add());
    println!("{}", sample.add());

    // sample_aのプロパティの型はAddやCopyトレイトを実装していないため
    // addメソッドが存在せず、addメソッドを使おうとしたらコンパイルエラーになる
    // let sample_a = Sample {
    //     x: String::from("hello"),
    //     y: String::from("bye"),
    // };
    // println!("{}", sample_a.add());

    let tweet = Tweet {
        author: String::from("Katsu"),
        text: String::from("I'm very happy! Yeah!"),
    };
    
    println!("{:#}", type_of(&tweet.summarize()));
    
    println!("{}", &tweet.default());

    notify(&tweet);

    // sampleはSummaryトレイトを実装してないのでコンパイルエラー
    // notify(&sample);

    let vec: Vec<u32> = vec![1, 199, 22, 18];
    let largest = largest(&vec);
    println!("{}", largest);
    println!("{:?}", vec);


    // let mut b = &a;とlet b = &mut a;は違う
    // 前者はaに入ったポインタが可変という意味なので、ポインタの値が別のポインタに変わる = 別のアドレスのデータを参照する可能性がある
    // 後者はaを通してbの値自体変わる可能性がある。(その場合はb自体も可変である必要がある)
    let a = String::from("hello");
    let mut b = &a;
    println!("first:{}", b);
    let c = String::from("bye");
    b = &c;
    println!("second:{}", b);
    println!("third:{}", a);

    // Vec<T>
    // Vec<T>は同じ型の値のコレクション
    // コンパイル時にサイズは決まっておらず、ヒープ領域に保持される
    // 生成方法new()かvec![](マクロ)
    // 生成時に値が空の場合は型注釈必須
    let v_new: Vec<String> = Vec::new();
    let mut v_macro = vec![1,12,15]; // 初期値のあるVec<T>を生成する方が一般的
    v_macro.push(24); // 末尾に追加
    v_macro.pop(); // 末尾から取り除く
    v_macro.remove(0); // 指定したインデックスを削除
    v_macro.reverse(); // 順番を逆にする
    println!("{:?}", &v_macro); // [15, 12]

    // Vec<T>の要素を読む
    let first = &v_macro[0]; // 要素が存在しないインデックスにアクセスしようとした場合panic
    let first_get: Option<&i32> = v_macro.get(21); // 要素が存在しないインデックスにアクセスしようとした場合Noneが返る
    match first_get {
        Some(&ele) => println!("get {:?}", &ele),
        None => println!("{}", "No element"),
    }

    // Vec<T>の要素の型がスタックに保持されるような型の場合
    let mut v_borrow = vec![9, 22, 42, 500];
    // v_borrow[1]の値がsecondにコピーされる(Vec<T>の要素がi32で、スタックに保持されるため)
    // secondとv_borrow[1]はそれぞれスタックの別々のアドレスに保持されている
    let second = v_borrow[1];
    // v_borrowの値を書き換えてもsecondの値は書き換わらない
    v_borrow[1] = 100;
    // 以下の2つは異なるアドレスを指す
    println!("{:p}", &v_borrow[1]);
    println!("{:p}", &second);

    let mut v_string = vec![String::from("h"), String::from("a"), String::from("c"), String::from("k")];
    let first_st = &mut v_string[2];
    // v_string[2]のデータはfirst_stに借用中なので変更できない
    // 通常、mutableで借用すれば可変なはずだが、
    // Vec<T>の場合、変更の際に連続したメモリ領域を確保できなかった場合に別の領域を確保し直し、
    // 全く別のアドレスになってしまう可能性があるから
    // その場合、first_stはダングリングポインタになってしまう
    // ↓ そのため、これらはコンパイルエラーになる
    // v_string.push(String::from("ccc"));
    // v_string[2] = String::from("bb");
    println!("{}", first_st);

    // for in 文で内部の要素を走査出来る
    let v_for = vec![40, 22, 499, 211];
    // itemは不変な参照
    for item in &v_for {
        println!("{}", item);
    }

    let mut v_for_mut = vec![40, 22, 499, 211];
    // itemは可変な参照
    for item in &mut v_for_mut {
        *item = 19;
        println!("{}", item);
    }

    // 異なる型のコレクションを保持したい場合はenumを使う
    enum SpreadssheetCell {
        Int(i32),
        Float(f32),
        Text(String),
    }
    let row = vec![
        SpreadssheetCell::Int(21),
        SpreadssheetCell::Float(4.4),
        SpreadssheetCell::Text(String::from("hello")),
    ];

    // 配列 = 要素数が固定 = 各要素がメモリに格納され、開始点への「ポインタ」「長さ」「キャパシティ」をarrに持つ
    let arr: [u32; 3] = [22,33,44];
    // 配列の一部or全部への参照 = 開始点のポインタと長さを持つ
    let slice: &[u32] = &arr[1..2];
    // String = ヒープ領域に文字列が格納され、そのメモリへの「ポインタ」、「長さ」、「キャパシティ」をstring_valが持つ
    // String型は伸長可能、可変、所有権を持つ
    // Stringも&strもUTFエンコードされたデータが入る
    let string_val: String = String::from("aaaa");
    // &str = Stringの一部or全部の参照(&Stringの場合も&strの型注釈を付けれるっぽい) = 開始点のポインタと長さを持つ
    let str_val: &str = &string_val;
    let str_val_part: &str = &string_val[1..2];
    let str_literal: &str = "aaaa";

    let hira = String::from("あ");
    // 文字列をバイトスライスに変換
    // "あ" => UTF-8エンコード => E3,81,82(16進数)
    // E3,81,82 => 16進数から10進数に変換 => 227,129,130
    // [227,129,130]がバイトスライス
    let bytes_hira = hira.as_bytes();
    println!("{:?}", bytes_hira);

    // ASCII文字の場合は直接10進数表現と対応している
    let alpha = String::from("a");
    let bytes_alpha = alpha.as_bytes();
    println!("{:?}", bytes_alpha); // [97]

    // 文字列連結
    let mut str_base = String::from("base");
    let str_pushed_literal = " and pushed";
    let str_pushed_real = String::from(" and pushed");
    // push_strは引数にStringではなく&strを渡すため、引数に指定した変数の所有権を奪わない(借用するだけ)
    str_base.push_str(&str_pushed_real);
    // push_strに渡した後も参照できる
    println!("{}", &str_pushed_real);
    println!("{}", &str_base);
}

// Summaryトレイトを実装したインスタンス(の参照)のみ受け付ける
fn notify(item: &impl Summary) {
    println!("Summary is {:?} for me", item.summarize());
}

// トレイト境界を使った書き方
#[allow(dead_code)] // 勉強用に描いただけなのでdead_codeを許容
fn notify_boundary<T: Summary>(item: &T) {
    println!("Summary is {:?} for me", item.summarize());
}

// whereを使った書き方
#[allow(dead_code)] // 勉強用に描いただけなのでdead_codeを許容
fn notify_where<T>(item: &T) 
    where T: Summary
{
    println!("Summary is {:?} for me", item.summarize());
}

// 戻り値に特定のトレイトを実装した型を指定
#[allow(dead_code)]
fn return_summarize() -> impl Summary {
    Tweet {
        author: String::from("Katsu"),
        text: String::from("I am implemented Summary"),
    }
}


// <_: T>引数は不要だけど引数の型だけ使いたい場合のジェネリクス
fn type_of<T>(_: T) -> String{
    let a = type_name::<T>();
    return a.to_string();
}

fn add_message(msgs: &Rc<RefCell<Vec<String>>>, msg: String) {
    // borrow_mut()でRefCell<T>の内部の値(Vec<String>)への可変参照を得る
    // borrow_mut(), borrow()はそれぞれRefMut<T>, Ref<T>型の値を返す
    // どちらもDerefトレイトを実装しているため参照のように扱える
    msgs.borrow_mut().push(msg);
}

// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();
//     // bytes.iter()でイテレータを返すことでitemを参照できる
//     // bytes.iter().enumerate()によりイテレータの各要素をラップして添字iと&itemのタプルを取り出せるようになる
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i
//         }
//     }
//     s.len()
// }

// 文字列スライス(&str)を返す
// &strは不変な参照
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    // bytes.iter()でイテレータを返すことでitemを参照できる
    // bytes.iter().enumerate()によりイテレータの各要素をラップして添字iと&itemのタプルを取り出せるようになる
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i]
        }
    }
    &s[..]
}

// 'a：ジェネリックなライフタイム引数
// <>で与えられたライフタイムが各引数と戻り値のライフタイムに設定されている
// 引数と戻り値のライフタイムが同じことを表している
// <'a>に渡されるライフタイムは、xかyのライフタイムの短い方になる
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// largest関数はT型のスライスを受け取る
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    // スライスの一部の値をコピーする = largestのデータがスタックにコピーされる(コピートレイトを実装してるため)
    let mut largest = list[0];

    // itemにはlistの各値が入る(参照ではなく実態がスタックにコピーされる)
    // スタックに積まれている実体同士を比較するためにitemではなく&itemにしている
    // &itemにすることによりitemには実体が入る
    for &item in list {
        // スタックに存在する値同士を比較
        if item > largest {
            largest = item;
        }
    }
    largest
}

#[allow(dead_code)]
fn largest_clone<T: PartialOrd + Clone>(list: &[T]) -> T {
    // list[0]の参照をlargestに代入 = 所有権はムーブしない
    // largestはlist[0]へのポインタを保持しており、largestはmutableになっている
    // = largestのポインタは書き換え可能なので、別のポインタが入ることができる
    // = largestが可変だからといって、list[0]が可変な訳ではない
    let mut largest = &list[0];

    // listの各要素の参照がitemに入る
    // itemはimmutable
    for item in list {
        // 参照している値同士を比較
        if item > largest {
            // largestに入っているポインタの値を
            // itemに入っているポインタの値で上書きする
            // largestはmutableなので可能
            largest = item;
        }
    }
    // largestが指している値をcloneして返却
    // cloneしているのでlargestが指しているデータの所有権はムーブしない
    largest.clone()
}

#[allow(dead_code)]
fn largest_reference<T: PartialOrd>(list: &[T]) -> &T {
    // largestには参照が入る
    let mut largest = &list[0];

    // itemにも参照が入る
    for item in list {
        // 参照先の値同士を比較
        if item > largest {
            // 参照に入っているポインタの値を上書き
            // 実態に影響はない
            largest = item;
        }
    }
    // 参照をそのまま返す
    largest
}



// ローカル変数の参照を返すことは出来ない
// なぜなら、参照の方がローカル変数の実体より長く生きてしまうから
// ローカル変数を生成してそれをreturnするなら参照ではなく実体をreturnして所有権ごと渡してしまった方がいい
// この関数自体、return後にローカル変数を使うことが無いため、所有権を渡しても問題ない
fn hoge() -> String {
    let hoge = "hello".to_string();
    hoge
}



// pub struct ImportantExcerpt<'a> {
//     part: &'a str,
// }

// // implキーワードの横の<'a>でライフタイムを定義
// impl<'a> ImportantExcerpt<'a> {
//     // 戻り値のライフタイムは&selfのライフタイムになる
//     fn level(&self, announcement: &str) -> &str {

//         // 'staticライフタイムはプログラム全体の期間を表す
//         // 文字列リテラルはそのままバイナリに埋め込まれる(スタック領域)ため'staticになる
//         let literal: &'static str = "literal has static lifetime";
//         println!("{}", literal);

//         println!("{}", announcement);
//         self.part
//     }
// }