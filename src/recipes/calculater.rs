// JとQとKは「10」として扱う
// Aは「1」もしくは「11」どちらか都合のいいように扱う
pub fn score_calculate(hands: &Vec<u32>) -> u32 {
    let mut score: u32 = 0;
    let mut use_ace: u32 = 0;

    for index in hands.iter() {
        match index {
            // スペード
            0 => use_ace += 1,
            1..=9 => score += index + 1,
            10..=12 => score += 10,
            // ハート
            13 => use_ace += 1,
            14..=22 => score += index - 12,
            23..=25 => score += 10,
            // ダイヤ
            26 => use_ace += 1,
            27..=35 => score += index - 25,
            36..=38 => score += 10,
            // クローバー
            39 => use_ace += 1,
            40..=48 => score += index - 38,
            49..=51 => score += 10,
            // 想定外
            _ => panic!(),
        }
    }
    if score > 21 {
        score
    } else if use_ace > 0 {
        decision_ace(score, use_ace)
    } else {
        score
    }
}

// use_ace個の可能な限りの1と11の組み合わせの合計値を求めたい
fn decision_ace(score: u32, use_ace: u32) -> u32 {
    // 11の枚数
    let eleven: u32 = (22 - score - use_ace) / 10;
    // 1の枚数
    let one: u32 = use_ace - eleven;

    score + 11 * eleven + one
}
