use crate::types::*;
use std::collections::{HashMap, HashSet};

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

include!(concat!(env!("OUT_DIR"), "/shanten_hash_table.rs"));

/// 通常手の向聴数を計算（1で一向聴, 0で聴牌, -1で和了）
pub fn calculate_normal_shanten(tiles: &[TileId]) -> i32 {
    let counts = calculate_tile_counts(tiles);
    let mut min_shanten = calculate_normal_shanten_use_table(&counts);

    // 頭を抜いて調べる
    let mult_counts = counts
        .iter()
        .filter(|(_, c)| **c >= 2)
        .collect::<HashMap<_, _>>();
    for t in mult_counts.keys() {
        let mut headless = counts.clone();
        headless[t] -= 2;
        let shanten = calculate_normal_shanten_use_table(&headless) - 1;
        if shanten < min_shanten {
            min_shanten = shanten;
        }
    }

    min_shanten
}

/// 通常手の向聴数をテーブル引きにより計算
fn calculate_normal_shanten_use_table(counts: &TileCount) -> i32 {
    let mut num_mentsu = 0;
    let mut num_tatsu = 0;

    // 数牌の並びから面子と塔子を数える
    for ty in [TileType::MANZU, TileType::PINZU, TileType::SOUZU] {
        let typed_count: HashMap<_, _> = counts
            .iter()
            .filter(|(t, _)| t.is_suhai() && (t.gettype() == ty))
            .collect();
        let mut key: Vec<char> = "000000000".chars().collect();
        for (t, v) in typed_count.iter() {
            key[(t.getnumber() - 1) as usize] = char::from_digit(**v as u32, 10).unwrap();
        }
        let keystr = String::from_iter(key.iter());
        let (nments, ntatsu) = SHANTEN_HASH.get(&*keystr).unwrap();
        num_mentsu += nments;
        num_tatsu += ntatsu;
    }

    // 字牌は刻子と対子だけ数えればOK
    let ji_count: HashMap<_, _> = counts.iter().filter(|(t, _)| t.is_jihai()).collect();
    for c in ji_count.values() {
        match **c {
            2 => {
                num_tatsu += 1;
            }
            cnt if cnt > 2 => {
                num_mentsu += 1;
            }
            _ => {}
        }
    }

    // 刻子+塔子が4を超ていたら4に制限
    if (num_mentsu + num_tatsu) > 4 {
        num_tatsu = 4 - num_mentsu
    }

    8 - 2 * num_mentsu - num_tatsu
}

/// 七対子手の向聴数を計算（1で一向聴, 0で聴牌, -1で和了）
pub fn calculate_chitoitsu_shanten(tiles: &[TileId]) -> i32 {
    let counts = calculate_tile_counts(tiles);
    let num_toitsu = counts.iter().filter(|(_, c)| **c >= 2).count() as i32;
    let num_types = counts.iter().filter(|(_, c)| **c >= 1).count() as i32;
    let mut shanten = 6 - num_toitsu;

    // 3毎持ち以上の面子は無効なので向聴数を増やす
    if num_types < 7 {
        shanten += 7 - num_types;
    }

    shanten
}

/// 国士無双手の向聴数を計算（1で一向聴, 0で聴牌, -1で和了）
pub fn calculate_kokushimusou_shanten(tiles: &[TileId]) -> i32 {
    let counts = calculate_tile_counts(tiles);
    // 么九牌を数える
    let yaochu_counts: HashMap<_, _> = counts
        .iter()
        .filter(|(t, c)| **c >= 1 && t.is_yaochu())
        .collect();
    // 頭があるか？
    let head = yaochu_counts.iter().any(|(_, c)| **c >= 2) as i32;

    13 - yaochu_counts.keys().len() as i32 - head
}

/// 通常/七対子/国士無双手の中で最小の向聴数を計算（1で一向聴, 0で聴牌, -1で和了）
pub fn calculate_shanten(tiles: &[TileId]) -> i32 {
    *[
        calculate_normal_shanten(tiles),
        calculate_chitoitsu_shanten(tiles),
        calculate_kokushimusou_shanten(tiles),
    ]
    .iter()
    .min()
    .unwrap()
}

/// 有効牌（向聴数を下げる牌）を列挙
fn listup_effective_tiles_common(
    tiles: &[TileId],
    candidate_tiles: &[TileId],
    shanten_calculator: &dyn Fn(&[TileId]) -> i32,
) -> Result<Vec<TileId>, Error> {
    let current_shanten = shanten_calculator(tiles);
    let mut effective_tiles = vec![];
    for ct in candidate_tiles {
        let mut appended = tiles.to_vec();
        appended.push(*ct);
        if current_shanten > shanten_calculator(appended.as_slice()) {
            effective_tiles.push(*ct);
        }
    }
    effective_tiles.sort_by(|a, b| a.partial_cmp(b).unwrap());
    Ok(effective_tiles)
}

/// 通常手の有効牌候補を列挙
fn listup_normal_candidate_effective_tiles(tiles: &[TileId]) -> Result<HashSet<TileId>, Error> {
    let counts = calculate_tile_counts(tiles);
    let sum_counts = counts.iter().fold(0, |sum, c| sum + c.1);

    // 牌数チェック（自摸した後であることを要求）
    if !(((sum_counts % 3) == 1) && (sum_counts >= 1) && (sum_counts < 14)) {
        return Err(Error::from(
            "Cannot to calculate effective tiles: invalid number of tiles.",
        ));
    }

    // 候補牌の列挙
    let mut candidate_tiles = HashSet::new();
    for (t, c) in &counts {
        if *c > 0 {
            // 2枚以上になるときは候補
            candidate_tiles.insert(*t);
            // 順子候補の列挙
            if t.is_suhai() {
                match t.getnumber() {
                    1 => {
                        candidate_tiles.insert(t.nth(1));
                        candidate_tiles.insert(t.nth(2));
                    }
                    2 => {
                        candidate_tiles.insert(t.nth(-1));
                        candidate_tiles.insert(t.nth(1));
                        candidate_tiles.insert(t.nth(2));
                    }
                    8 => {
                        candidate_tiles.insert(t.nth(-2));
                        candidate_tiles.insert(t.nth(-1));
                        candidate_tiles.insert(t.nth(1));
                    }
                    9 => {
                        candidate_tiles.insert(t.nth(-2));
                        candidate_tiles.insert(t.nth(-1));
                    }
                    _ => {
                        candidate_tiles.insert(t.nth(-2));
                        candidate_tiles.insert(t.nth(-1));
                        candidate_tiles.insert(t.nth(1));
                        candidate_tiles.insert(t.nth(2));
                    }
                };
            }
        }
    }

    Ok(candidate_tiles)
}

/// 七対子手の有効牌候補を列挙
fn listup_chitoitsu_candidate_effective_tiles(tiles: &[TileId]) -> Result<HashSet<TileId>, Error> {
    let counts = calculate_tile_counts(tiles);
    let sum_counts = counts.iter().fold(0, |sum, c| sum + c.1);

    // 牌数チェック（自摸した後であることを要求）
    if !(((sum_counts % 3) == 1) && (sum_counts >= 1) && (sum_counts < 14)) {
        return Err(Error::from(
            "Cannot to calculate effective tiles: invalid number of tiles.",
        ));
    }

    // 候補牌の列挙
    let mut candidate_tiles = HashSet::new();
    for (t, c) in &counts {
        if *c > 0 {
            // 2枚以上になるときは候補
            candidate_tiles.insert(*t);
        }
    }

    Ok(candidate_tiles)
}

/// 国士無双手の有効牌候補を列挙
fn listup_kokushimusou_candidate_effective_tiles(
    tiles: &[TileId],
) -> Result<HashSet<TileId>, Error> {
    let counts = calculate_tile_counts(tiles);
    let sum_counts = counts.iter().fold(0, |sum, c| sum + c.1);

    // 牌数チェック（自摸した後であることを要求）
    if !(((sum_counts % 3) == 1) && (sum_counts >= 1) && (sum_counts < 14)) {
        return Err(Error::from(
            "Cannot to calculate effective tiles: invalid number of tiles.",
        ));
    }

    // 候補牌の列挙
    let candidate_tiles = HashSet::from([
        TileId::Id1man,
        TileId::Id9man,
        TileId::Id1pin,
        TileId::Id9pin,
        TileId::Id1sou,
        TileId::Id9sou,
        TileId::IdTon,
        TileId::IdNan,
        TileId::IdSha,
        TileId::IdPee,
        TileId::IdHaku,
        TileId::IdHatu,
        TileId::IdChun,
    ]);

    Ok(candidate_tiles)
}

/// 通常手の有効牌（向聴数を下げる牌）を列挙
pub fn listup_normal_effective_tiles(tiles: &[TileId]) -> Result<Vec<TileId>, Error> {
    let candidate_tiles = listup_normal_candidate_effective_tiles(tiles)?;
    listup_effective_tiles_common(
        tiles,
        Vec::from_iter(candidate_tiles).as_slice(),
        &calculate_normal_shanten,
    )
}

/// 七対子手の有効牌（向聴数を下げる牌）を列挙
pub fn listup_chitoitsu_effective_tiles(tiles: &[TileId]) -> Result<Vec<TileId>, Error> {
    let candidate_tiles = listup_chitoitsu_candidate_effective_tiles(tiles)?;
    listup_effective_tiles_common(
        tiles,
        Vec::from_iter(candidate_tiles).as_slice(),
        &calculate_chitoitsu_shanten,
    )
}

/// 国士無双手の有効牌（向聴数を下げる牌）を列挙
pub fn listup_kokushimusou_effective_tiles(tiles: &[TileId]) -> Result<Vec<TileId>, Error> {
    let candidate_tiles = listup_kokushimusou_candidate_effective_tiles(tiles)?;
    listup_effective_tiles_common(
        tiles,
        Vec::from_iter(candidate_tiles).as_slice(),
        &calculate_kokushimusou_shanten,
    )
}

/// 有効牌（向聴数を下げる牌）を列挙
pub fn listup_effective_tiles(tiles: &[TileId]) -> Result<Vec<TileId>, Error> {
    let normal_candidates = listup_normal_candidate_effective_tiles(tiles)?;
    let chitoitsu_candidates = listup_chitoitsu_candidate_effective_tiles(tiles)?;
    let kokushimusou_candidates = listup_kokushimusou_candidate_effective_tiles(tiles)?;

    // 候補牌を結合
    let mut marged_candidates = normal_candidates;
    marged_candidates.extend(&chitoitsu_candidates);
    marged_candidates.extend(&kokushimusou_candidates);

    listup_effective_tiles_common(
        tiles,
        Vec::from_iter(marged_candidates).as_slice(),
        &calculate_shanten,
    )
}
