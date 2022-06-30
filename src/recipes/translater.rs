pub fn soot_translate(hands: &Vec<u32>) -> Vec<&str> {
    let mut with_soot: Vec<&str> = vec![];
    for alias in hands.iter() {
        match alias {
            // スペード
            0..=12 => with_soot.push("スペード"),
            // ハート
            13..=25 => with_soot.push("ハート"),
            // ダイヤ
            26..=38 => with_soot.push("ダイヤ"),
            // クローバー
            39..=51 => with_soot.push("クローバー"),
            // 想定外
            _ => panic!(),
        }
    }
    with_soot
}
