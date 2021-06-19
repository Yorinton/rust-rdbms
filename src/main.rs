use std::thread;
use std::time::Duration;
use std::collections::HashMap;

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

fn generate_workout(intensity: u32, random_number: u32) {
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
            expensive_result.value(intensity)
        );
        println!(
            // 次に、{}回腹筋をしてください！
            "Next, do {} situps!",
            expensive_result.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            )
        }
    }
}

struct Cacher<T: Fn(u32) -> u32> {
    calculation: T, // クロージャ
    value: HashMap<u32, u32>, // クロージャの実行結果 クロージャが一度でも実行されたら値がセットされる
}
// struct Cacher<T>
//     where T: Fn(u32) -> u32
// {
//     calculation: T,
//     value: Option<u32>,
// }

impl<T> Cacher<T>
    where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Self {
        Cacher {
            calculation,
            value: HashMap::new()
        }
    }

    fn value(&mut self, key: u32) -> u32 {
        // valueに値が設定されている場合はその値を、
        // 設定されてない場合はcalculationの実行結果をvalueに設定した上で返す
        match self.value.get(&key) {
            Some(val) => *val,
            None => {
                let calculated = (self.calculation)(key);
                self.value.insert(key, calculated);
                calculated
            }
        }
    }
}

#[test]
fn call_with_different_values() {
    let mut closure_result = Cacher::new(|n| n * n);
    let v = closure_result.value(3);
    let v2 = closure_result.value(4);

    assert_eq!(v, 9);
    assert_eq!(v2, 16);
}