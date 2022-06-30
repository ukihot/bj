use crate::recipes::deck_editer;
use crate::recipes::translater::soot_translate;
mod recipes;

fn main() {
    let mut hands_player = vec![];
    let mut hands_dealer = vec![];

    // 挨拶
    println!("ブラックジャックを開始します。");
    // プレイヤーが2枚引いて準備
    preparation(&mut hands_player, 2);
    // ディーラーが1枚引いて準備
    preparation(&mut hands_dealer, 1);
    // プレイヤーの1枚目と2枚目を表示
    println!("あなたの手札：{:?}", soot_translate(&hands_player));
    // ディーラーの手札を1枚だけ表示
    println!("相手の手札：[{}]", soot_translate(&hands_dealer)[0]);

    // プレイヤーは3枚目を引いた場合に3枚の合計が「21」を超えそうだと思うなら「スタンド」を選択

    let mut answer = String::new();
    // 入力文字のバリデーション
    while answer_validate(&answer) {
        let mut word = String::new();
        std::io::stdin().read_line(&mut word).ok();
        answer = word.trim().to_string();
    }
    // ヒットを選択して合計値が22以上ならバースト(プレイヤーの敗北)
    // ヒットを選択して合計値が20以下なら再度ヒット or スタンドを選択（ヒットは何回でもOK）
    // プレイヤーが合計値21以下で勝負を待っている状態になったらディーラーは合計値が17以上になるまで無条件にカードを引く
    // バーストしたらプレイヤーの勝利
    // プレイヤーとディーラーが引き終えたら勝負。より21に近い方の勝ち
}

/*
プレイヤーとディーラーの共通処理
*/
fn preparation(hands: &mut Vec<u32>, pull_max: u32) {
    // カードは52枚
    let mut cards_status: [bool; 52] = [true; 52];
    // プレイヤーはカードを2枚引く
    (0..pull_max).for_each(|n| {
        hands.push(deck_editer::pull_card(&mut cards_status));
    });
}

fn answer_validate(c: &str) -> bool {
    if c == "h" {
        false
    } else if c == "s" {
        false
    } else {
        println!("ヒットならh、スタンドならsを入力してください[h / s]");
        true
    }
}
