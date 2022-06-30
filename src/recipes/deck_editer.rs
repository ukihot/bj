use rand::Rng;

pub(crate) fn pull_card(cards_status: &mut[bool; 52]) -> u32 {
    // cards_statusの中からfalseのカード候補を検索
    let candidates = serach_available(*cards_status);
    // 候補からランダム抽出
    let pull_number = candidates[rand::thread_rng().gen_range(0..(candidates.len() - 1))];
    // 抽出したカードはデッキから抜く
    cards_status[pull_number] = false;
    // 抜いたカードを返す
    pull_number as u32
}

fn serach_available(cards_status: [bool; 52]) -> Vec<usize> {
    let mut candidates = vec![];
    for (index, status) in cards_status.iter().enumerate() {
        if *status == true {
            candidates.push(index)
        }
    }
    candidates
}
