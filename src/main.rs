use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
  println!("猜数字!");

  let secret_number = rand::thread_rng().gen_range(1..101);

  loop {
    println!("请输入猜想的数字。");

    let mut guess = String::new();

    io::stdin()
      .read_line(&mut guess)
      .expect("Failed to read line!");
    
    let guess: u32 = match guess.trim().parse() {
      Ok(num) => num,
      Err(_) => {
        println!("请输入有效的数字！");
        continue
      },
    };

    match guess.cmp(&secret_number) {
      Ordering::Less => println!("你猜的太小了!"),
      Ordering::Greater => println!("你猜的太大了！"),
      Ordering::Equal => {
        println!("恭喜你猜中了！");
        break;
      },
    }
  }
}
