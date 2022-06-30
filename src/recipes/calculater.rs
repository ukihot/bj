// JとQとKは「10」として扱う
// Aは「1」もしくは「11」どちらか都合のいいように扱う
pub fn score_calculate(hands: &Vec<u32>) -> u32 {
    let mut score: u32 = 0;

    for index in hands.iter() {
        match index {
            // スペード
            0 => {},
            1..=9 => score += index + 1,
            10..=12 => score += 10,
            // ハート
            13 => {},
            14..=22 => score += index - 12,
            23..=25 => score += 10,
            // ダイヤ
            26 => {},
            27..=35 => score += index - 25,
            36..=38 => score += 10,
            // クローバー
            39 => {},
            40..=48 => score += index - 38,
            49..=51 => score += 10,
            // 想定外
            _ => panic!(),
        }
        println!("now ={}",score);
        score += index;
    }
    score
}
