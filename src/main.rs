use std::io;
use rand::Rng;

fn main() {
    println!("猜字游戏");
    println!("猜一个 1 到 100 之间的数字");

    // 生成 1 到 100 之间的随机数
    let secret_number = rand::rng().random_range(1..=100);

    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("读取失败");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("请输入一个有效的数字！");
                continue;
            }
        };

        if guess < 1 || guess > 100 {
            println!("请输入一个 1 到 100 之间的数字");
            continue;
        }

        println!("你猜的数字是: {}", guess);

        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("太小了！"),
            std::cmp::Ordering::Greater => println!("太大了！"),
            std::cmp::Ordering::Equal => {
                println!("猜对了！");
                break;
            }
        }
    }
}
