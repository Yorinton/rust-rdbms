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
