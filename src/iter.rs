use std::collections::HashMap;

pub fn vec_iter() {
    let v1 = vec![10,20,30];
    let v1_iter = v1.iter();
    for val in v1_iter {
        println!("{}", val);
    }
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
