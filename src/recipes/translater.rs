pub fn soot_translate(hands: &Vec<u32>) -> Vec<String> {
    let mut with_soot: Vec<String> = vec![];
    for alias in hands.iter() {
        match alias {
            // スペード
            0..=12 => {
                let n: &str = &(alias + 1).to_string();

                with_soot.push("スペードの".to_string() + n);
            }
            // ハート
            13..=25 => with_soot.push("ハート".to_string()),
            // ダイヤ
            26..=38 => with_soot.push("ダイヤ".to_string()),
            // クローバー
            39..=51 => with_soot.push("クローバー".to_string()),
            // 想定外
            _ => panic!(),
        }
    }
    with_soot
}
