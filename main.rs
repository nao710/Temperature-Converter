use std::io::stdin;

fn to_fahrenheit(celsius: &f64) -> f64 {
    celsius * 1.8 + 32.0
}

fn to_celsius(fahrenheit: &f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

fn main() {
    println!("どちらを変換しますか？");
    println!("(1) 摂氏⇒ 華氏");
    println!("(2) 華氏⇒ 摂氏");

    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .expect("1か2を入力してください");
    // println!(":{}", number);
    let number: i32 = input.trim().parse::<i32>().expect("1か2を入力してください");
    match number {
        1 => {
            let mut input = String::new();
            println!("摂氏を入力");
            stdin().read_line(&mut input).expect("摂氏を入力");
            let celsius = input.trim().parse::<f64>().expect("摂氏を入力");
            println!("華氏{}℉ です", to_fahrenheit(&celsius));
        }

        2 => {
            let mut input = String::new();
            println!("華氏を入力");
            stdin().read_line(&mut input).expect("華氏を入力");
            let fahrenheit = input.trim().parse::<f64>().expect("華氏を入力");
            println!("摂氏{}℃ です", to_celsius(&fahrenheit));
        }
        _ => println!("1か2を入力してください"),
    }
}
