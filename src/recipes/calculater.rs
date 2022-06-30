// JとQとKは「10」として扱う
// Aは「1」もしくは「11」どちらか都合のいいように扱う
pub fn score_calculate(hands: &Vec<u32>) -> u32{
    let mut score :u32 = 0;

    for index in hands {
        match index {
            // スペード
            0..=12 => {
                // fd
            }
            // ハート
            13..=25 => {
                //
            }
            // ダイヤ
            26..=38 => {
                //
            }
            // クローバー
            39..=51 => {
                //
            }
            // 想定外
            _ => panic!(),
        }
        score += index;
    }
    score
}