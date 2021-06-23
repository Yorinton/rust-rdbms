use std::collections::HashMap;

pub fn vec_iter() {
    let v1 = vec![10,20,30];
    let v1_iter = v1.iter();
    // forループがv1_iterの所有権を奪っている
    for val in v1_iter {
        println!("{}", val);
    }
    // 所有権が奪われているため、以下でv1_iterは使えない
    // println!("{:?}", v1_iter);
}

pub fn vec_iter_next() {
    let v1 = vec![10,20,30];

    // nextメソッドを呼び出す際は、イテレータは可変である必要がある
    // nextによって今シーケンスのどこにいるのかを追いかけるため、内部の状態を変更する必要があるため
    // forループを使う場合は、forがイテレータの所有権を奪って裏で内部状態を変更する
    let mut v1_iter = v1.iter();
    println!("{}", v1_iter.next().unwrap());
    println!("{}", v1_iter.next().unwrap());
    println!("{}", v1_iter.next().unwrap());
}

pub fn hash_map_iter() {
    let mut h = HashMap::new();
    h.insert(String::from("福岡"), String::from("ラーメン"));
    h.insert(String::from("長崎"), String::from("ちゃんぽん"));

    let h_iter = h.iter();
    for (key, val) in h_iter {
        println!("{}の名産は{}", key, val);
    }
}

pub fn hash_map_iter_mut() {
    let mut h = HashMap::new();
    h.insert(String::from("福岡"), String::from("ラーメン"));
    h.insert(String::from("長崎"), String::from("ちゃんぽん"));

    let h_iter = h.iter_mut();
    for (key, val) in h_iter {
        if key == "福岡" {
            // dereferenceしてから値(実態)を書き換える
            *val = String::from("明太子");
        }
        println!("{}の名産は{}", key, val);
    }
}

pub fn create_hash_map_by_map() {
    // mapを使ってRangeからHashMapを生成
    let hmap: HashMap<i32, i32> = (0..3).map(|k| (k, k*2)).collect();
    // HashMapを直接ループ処理した場合と、HashMapのイテレータをループ処理した場合、
    // 処理順序が異なる
    // HashMapを直接ループ処理した場合は、ハッシュ値の生成にランダム性があるので順番が毎回異なるのは分かる
    // @TODO：イテレータをループ処理した場合は要調査
    for (k,v) in hmap.iter() {
        println!("key：{}, val：{}", k, v);
    }

    // Rangeに対するループ処理
    // Rangeはstd::iter::Iteratorトレイトを実装している
    for v in 0..3 {
        println!("{}",v);
    }
}

pub fn iter_sum() {
    let v = vec![10,20,30];
    let v_iter = v.iter();

    // sum()はv_iter所有権を奪う
    let sum: i32 = v_iter.sum();
    println!("{}", sum);
}

pub fn hash_map_values() {
    let mut h = HashMap::new();
    h.insert(String::from("福岡"), 130);
    h.insert(String::from("長崎"), 95);

    // .valuesはHashMapの値だけのイテレータを返す
    // for文でループ処理する際に値のみ抽出される
    // .valuesの場合、イテレータの各値への不変参照を返す
    // (unstableだが、into_values()の場合は、各値の所有権を奪う)
    let vals = h.values();
    let sum: i32 = vals.sum();
    println!("{}", sum);

    // .into_iter()の場合、イテレータの各値の所有権を奪う
    let nums = h.into_iter();
    for (k, v) in nums {
        println!("key：{}, val：{}", k, v);
    }
}