pub fn soot_translate(hands: &Vec<u32>) -> Vec<String> {
    let mut with_soot: Vec<String> = vec![];
    for alias in hands.iter() {
        match alias {
            // スペード
            0..=12 => {
                let n: &str = &(alias + 1).to_string();
                with_soot.push("スペード♠の".to_string() + n);
            }
            // ハート
            13..=25 => {
                let n: &str = &(alias - 12).to_string();
                with_soot.push("ハート♡の".to_string() + n);
            }
            // ダイヤ
            26..=38 => {
                let n: &str = &(alias - 25).to_string();
                with_soot.push("ダイヤ♦の".to_string() + n);
            }
            // クローバー
            39..=51 => {
                let n: &str = &(alias - 38).to_string();
                with_soot.push("クローバー♧の".to_string() + n);
            }
            // 想定外
            _ => panic!(),
        }
    }
    with_soot
}
