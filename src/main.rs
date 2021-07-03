use std::hash::Hash;
use std::thread;
use std::time::Duration;
use std::collections::HashMap;
mod iter;
use iter::Shoe;
use crate::iter::ShoeCounter;

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    // クロージャは定義したスコープ外では呼べない
    // expensive_closure("aaaa");

    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );
}

/// workoutを生成
/// # Examples
///
/// ```
/// let intensity = 4;
/// let random_number = 22;
/// assert_eq!(4, generate_workout(4, 22));
/// ```
pub fn generate_workout(intensity: u32, random_number: u32) -> u32 {
    // クロージャ
    // クロージャは型注釈がなくてもコンパイルが通る
    // 通常クロージャは小さいスコープの中で使用される + インターフェースを公開する必要がない + あらゆる任意の文脈ではなく狭い文脈でのみ関係する、ため
    // 明示性のために型注釈することも可能
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    // 最初に呼び出されるタイミングで型推論が行われる
    // それ以降に呼び出された場合、最初に推論した型と異なっていた場合はコンパイルエラーになる
    // expensive_closure(String::from("あああ"));

    // 本体が１つの式のみからなる場合、{}は不要
    let shortest_closure = |x| x + 1;
    shortest_closure(2);

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_result.value(&intensity)
        );
        println!(
            // 次に、{}回腹筋をしてください！
            "Next, do {} situps!",
            expensive_result.value(&intensity)
        );
        *expensive_result.value(&intensity)
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(&intensity)
            )
        }
        *expensive_result.value(&intensity)
    }
}

struct Cacher<K, V, F> {
    calculation: F, // クロージャ
    value: HashMap<K, V>, // クロージャの実行結果 クロージャが一度でも実行されたら値がセットされる
}
// struct Cacher<T>
//     where T: Fn(u32) -> u32
// {
//     calculation: T,
//     value: Option<u32>,
// }

impl<K, V, F> Cacher<K, V, F>
    where K: Hash + Eq + Copy,
          V: Clone,
          F: Fn(K) -> V
{
    fn new(calculation: F) -> Self {
        Cacher {
            calculation,
            value: HashMap::new()
        }
    }

    fn value(&mut self, arg: K) -> V {
        // valueに値が設定されている場合はその値を、
        // 設定されてない場合はcalculationの実行結果をvalueに設定した上で返す
        match self.value.get(&arg) {
            // HashMapに保持されたVをcloneして返す
            Some(val) => val.clone(),
            None => {
                let calculated = (self.calculation)(arg);
                let v = self.value.entry(arg).or_insert(calculated);
                // HashMapに保持されたVのcloneを返す
                v.clone()
                // or_insertではなく、self.valueにinsertして、値を再度getした上でcloneを返す方法
                // self.value.insert(arg, calculated);
                // self.value.get(&arg).unwrap().clone()
            }
        }
    }
}

#[test]
fn test_vec_iter() {
    iter::vec_iter();
    iter::vec_iter_next();
}

#[test]
fn test_hash_map_iter() {
    iter::hash_map_iter();
    iter::hash_map_iter_mut();
    iter::create_hash_map_by_map();
    iter::hash_map_values();
}

#[test]
fn test_iter_sum() {
    iter::iter_sum();
}

#[test]
fn test_iter_map() {
    iter::iter_map();
}

#[test]
fn test_iter_filter() {
    iter::iter_filter();
}

#[test]
fn test_filter_struct() {
    let shoes = vec![
        Shoe::new(22, String::from("スニーカー")),
        Shoe::new(23, String::from("サンダル")),
        Shoe::new(24, String::from("ブーツ")),
        Shoe::new(33, String::from("革靴")),
    ];
    let my_shoes = iter::shoes_in_my_size(shoes, 24);
    println!("{:?}", my_shoes);
}

#[test]
fn test_shoe_count() {
    let shoes = vec![
        Shoe::new(22, String::from("スニーカー")),
        Shoe::new(23, String::from("サンダル")),
        Shoe::new(24, String::from("ブーツ")),
        Shoe::new(33, String::from("革靴")),
    ];
    let mut shoe_counter = ShoeCounter::new(shoes);
    iter::shoe_count(&mut shoe_counter);
}

#[test]
fn call_with_different_values() {
    let mut closure_result = Cacher::new(|n| n * n);
    let v = closure_result.value(3);
    let v2 = closure_result.value(4);

    assert_eq!(v, 9);
    assert_eq!(v2, 16);
}

#[test]
fn call_with_strings() {
    let mut closure_result = Cacher::new(|n: &str| -> String {
        let m = n.to_string();
        // m.push_str("でがんす");
        // m
        m + "でがんす"
    });
    let v = closure_result.value("こんにちわ");
    let v2 = closure_result.value("こんばんわ");

    assert_eq!(v, "こんにちわでがんす");
    assert_eq!(v2, "こんばんわでがんす");
}

#[test]
fn call_with_slice() {
    let mut closure_result = Cacher::new(|n: &str| -> usize {
        n.len()
    });
    let res = closure_result.value("abcd");
    assert_eq!(res, 4);
}
