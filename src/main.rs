mod recipes;
use recipes::hoge;

fn main() {
    hoge::fuga();
    println!("何か文字を入力して下さい:");
    let mut word = String::new();
    std::io::stdin().read_line(&mut word).ok();
    let answer = word.trim().to_string();
    println!("{}", answer);
}