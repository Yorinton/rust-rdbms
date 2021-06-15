use std::thread;
use std::time::Duration;

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
    let expensive_closure = |num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };
    // 最初に呼び出されるタイミングで型推論が行われる
    // それ以降に呼び出された場合、最初に推論した型と異なっていた場合はコンパイルエラーになる
    // expensive_closure(String::from("あああ"));

    // 本体が１つの式のみからなる場合、{}は不要
    let shortest_closure = |x| x + 1;

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_closure(intensity)
        );
        println!(
            // 次に、{}回腹筋をしてください！
            "Next, do {} situps!",
            expensive_closure(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_closure(intensity)
            )
        }
    }
}
