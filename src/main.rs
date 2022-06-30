use crate::recipes::deck_editer;
use crate::recipes::translater::soot_translate;
mod recipes;

fn main() {
    // カードは52枚
    let mut cards_status: [bool; 52] = [true; 52];
    let mut hands_player = vec![];
    let mut hands_dealer = vec![];

    // 挨拶
    println!("ブラックジャックを開始します。");
    // プレイヤーが2枚引いて準備
    preparation(&mut hands_player, 2, &mut cards_status);
    // ディーラーが1枚引いて準備
    preparation(&mut hands_dealer, 2, &mut cards_status);
    // プレイヤーの手札をすべて表示
    println!("あなたの手札：{:?}", soot_translate(&hands_player));
    // ディーラーの手札を1枚だけ表示
    println!("相手の手札：[{}]", soot_translate(&hands_dealer)[0]);

    // ヒットを尋ね続ける
    while ask_hit() == "h" {
        println!("新しくカードを引きました。");
        preparation(&mut hands_player, 1, &mut cards_status);
        // プレイヤーの1枚目と2枚目を表示
        println!("あなたの手札：{:?}", soot_translate(&hands_player));
    }

    // ヒットを選択して合計値が22以上ならバースト(プレイヤーの敗北)
    // ヒットを選択して合計値が20以下なら再度ヒット or スタンドを選択（ヒットは何回でもOK）
    // プレイヤーが合計値21以下で勝負を待っている状態になったらディーラーは合計値が17以上になるまで無条件にカードを引く
    // バーストしたらプレイヤーの勝利
    // プレイヤーとディーラーが引き終えたら勝負。より21に近い方の勝ち
}

fn ask_hit() -> String {
    // ヒットかスタンドか選択
    let mut hit_or_stand = String::new();

    // 入力文字のバリデーション
    while hit_or_stand_validate(&hit_or_stand) {
        let mut input_text = String::new();
        std::io::stdin().read_line(&mut input_text).ok();
        hit_or_stand = input_text.trim().to_string();
    }
    hit_or_stand
}

/*
プレイヤーとディーラーの共通処理
*/
fn preparation(hands: &mut Vec<u32>, pull_max: u32, cards_status: &mut [bool; 52]) {
    // カードは52枚
    // プレイヤーはカードを2枚引く
    (0..pull_max).for_each(|n| {
        hands.push(deck_editer::pull_card(cards_status));
    });
}

fn hit_or_stand_validate(c: &str) -> bool {
    if c == "h" {
        false
    } else if c == "s" {
        false
    } else {
        println!("ヒットならh、スタンドならsを入力してください[h / s]");
        true
    }
}
