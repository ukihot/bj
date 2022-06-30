use crate::recipes::{calculater, deck_editer, translater};
mod recipes;

fn main() {
    // カードは52枚
    let mut cards_status: [bool; 52] = [true; 52];
    let mut hands_player = vec![];
    let mut hands_dealer = vec![];
    let mut score_player: u32 = 0;
    let mut score_dealer: u32 = 0;

    // 挨拶
    println!("ブラックジャックを開始します。");
    // プレイヤーが2枚引いて準備
    preparation(&mut hands_player, 2, &mut cards_status);
    // ディーラーが1枚引いて準備
    preparation(&mut hands_dealer, 2, &mut cards_status);
    // プレイヤーの手札をすべて表示
    println!(
        "あなたの手札：{:?}",
        translater::soot_translate(&hands_player)
    );
    // ディーラーの手札を1枚だけ表示
    println!(
        "相手の手札：[{}]",
        translater::soot_translate(&hands_dealer)[0]
    );

    // ヒットを尋ね続ける
    while ask_hit() == "h" {
        println!("新しくカードを引きました。");
        preparation(&mut hands_player, 1, &mut cards_status);
        // プレイヤーの1枚目と2枚目を表示
        println!(
            "あなたの手札：{:?}",
            translater::soot_translate(&hands_player)
        );
        score_player = calculater::score_calculate(&hands_player);
        // 点数の表示
        println!("現在の点数：{}", score_player);
        if score_player > 21 {
            println!("バーストしました。");
            std::process::exit(0);
        }
    }
    // プレイヤーが合計値21以下でスタンドしたらディーラーは合計値が17以上になるまでカードを引く
    println!("スタンドしました。\nディーラーがカードを引きます。");
    while calculater::score_calculate(&hands_dealer) < 18 {
        preparation(&mut hands_dealer, 1, &mut cards_status);
    }
    score_dealer = calculater::score_calculate(&hands_dealer);
    // プレイヤーとディーラーが引き終えたら勝負。より21に近い方の勝ち
    if score_dealer > 21 {
        println!("ディーラーがバーストしました：{}", score_dealer);
        println!("Victory");
    } else if score_player > score_dealer {
        println!("ディーラーの点数に勝ちました：{}", score_dealer);
        println!("Victory");
    } else if score_player == score_dealer {
        println!("Draw");
    } else {
        println!("ディーラーの点数に負けました：{}", score_dealer);
        println!("Lose");
    }
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
    // カードを手札に加える
    (0..pull_max).for_each(|n| {
        hands.push(deck_editer::pull_card(cards_status));
    });
}

// hかsしか入力は受け付けない
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
