use crate::recipes::deck_editer;

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
    println!("あなたの手札：{:?}", hands_player);

    // プレイヤーは3枚目を引いた場合に3枚の合計が「21」を超えそうだと思うなら「スタンド」を選択
    println!("ヒットまたはスタンドを入力してください");
    let mut input_text = String::new();
    std::io::stdin().read_line(&mut input_text).ok();
    let hit_or_stand = input_text.trim().to_string();
    // 次を引いても「21」を超えなさそうなら「ヒット」を選択
    // JとQとKは「10」として扱う
    // Aは「1」もしくは「11」どちらか都合のいいように扱う
    // スタンド ... カードを引かずに勝負する / ヒット ... もう一枚引く
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
    let mut cards_status: [bool; 52] = [false; 52];
    // プレイヤーはカードを2枚引く
    (0..pull_max).for_each(|n| {
        hands.push(deck_editer::pull_card(&mut cards_status));
    });
}
