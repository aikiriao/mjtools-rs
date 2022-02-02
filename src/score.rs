use crate::shanten::*;
use crate::types::*;
use lazy_static::lazy_static;
use std::collections::{HashMap, HashSet};
use std::sync::RwLock;

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

/// 役の識別
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Yaku {
    /// 立直（リーチ）
    Riichi,
    /// ダブル立直（リーチ）
    Doubleriichi,
    /// 一発
    Ippatsu,
    /// 自摸（ツモ）
    Tsumo,
    /// 断么九（タンヤオ）
    Tanyao,
    /// 平和（ピンフ）
    Pinfu,
    /// 一盃口（イーペーコー）
    Ipeko,
    /// 場風
    Bakaze,
    /// 自風
    Jikaze,
    /// 役牌：白
    Haku,
    /// 役牌：發
    Hatu,
    /// 役牌：中
    Chun,
    /// 嶺上開花
    Rinshan,
    /// 槍槓（チャンカン）
    Chankan,
    /// 海底撈月（ハイテイツモ）
    Haiteitsumo,
    /// 河底撈魚（ハイテイロン）    
    Houteiron,
    /// 三色同順
    Sansyokudoujyun,
    /// 一気通貫
    Ikkitsukan,
    /// 混全帯么九（チャンタ）
    Chanta,
    /// 七対子（チートイツ）
    Chitoitsu,
    /// 対々和（トイトイ）
    Toitoiho,
    /// 三暗刻
    Sananko,
    /// 混老頭
    Honrouto,
    /// 三色同刻
    Sansyokudoukoku,
    /// 三槓子
    Sankantsu,
    /// 小三元
    Syosangen,
    /// 混一色（ホンイツ）
    Honitsu,
    /// 純全帯么九（純チャン）
    Jyunchanta,
    /// 二盃口
    Ryanpeko,
    /// 清一色（チンイツ）
    Chinitsu,
    /// 天和
    Tenho,
    /// 地和
    Chiho,
    /// 国士無双
    Kokushimusou,
    /// 国士無双13面待ち
    Kokushimusou13,
    /// 九蓮宝燈
    Churenpouton,
    /// 九蓮宝燈9面待ち
    Churenpouton9,
    /// 四暗刻
    Suanko,
    /// 四暗刻単騎待ち
    Suankotanki,
    /// 大四喜
    Daisushi,
    /// 小四喜
    Syosushi,
    /// 大三元
    Daisangen,
    /// 字一色
    Tsuiso,
    /// 清老頭
    Chinroto,
    /// 緑一色
    Ryuiso,
    /// 四槓子
    Sukantsu,
    /// ドラ
    Dora,
    /// 流し満貫
    Nagashimangan,
}

/// 和了時の状況
pub struct AgariInformation {
    /// 和了牌
    pub wining_tile: Tile,
    /// 手牌
    pub hand: Hand,
    /// 本場
    pub nhonba: i32,
    /// 供託立直棒数
    pub nriichi: i32,
    /// 場風
    pub round: Wind,
    /// 自風
    pub player: Wind,
    /// ツモ？
    pub tsumo: bool,
    /// 立直？
    pub riichi: bool,
    /// 一発？
    pub ippatsu: bool,
    /// ダブル立直？
    pub doubleriichi: bool,
    /// ハイテイロン？
    pub haitei: bool,
    /// 嶺上開花？
    pub rinshan: bool,
    /// 搶槓？
    pub chankan: bool,
    /// 流し満貫？
    pub nagashimangan: bool,
    /// 天和？
    pub tenho: bool,
    /// 地和？
    pub chiho: bool,
    /// ドラ表示牌
    pub dora: Dora,
}

/// 支払い情報
#[derive(Debug, PartialEq, Eq)]
pub enum Feed {
    /// 放銃時に支払う点数
    Duck { point: i32 },
    /// ツモ時に親と子が支払う点数
    Tsumo { ko: i32, oya: i32 },
}

/// 得点情報
#[derive(Debug, PartialEq, Eq)]
pub struct Point {
    /// 加点
    pub get: i32,
    /// 支払い点
    pub feed: Feed,
}

/// 得点計算結果
#[derive(Debug, PartialEq, Eq)]
pub struct Score {
    /// 翻
    pub han: i32,
    /// 符
    pub fu: i32,
    /// 成立した役
    pub yaku: HashSet<Yaku>,
    /// 得点情報
    pub point: Point,
}

/// 得点計算ルール
#[derive(Copy, Clone)]
pub struct ScoreRuleConfig {
    /// 喰いタンあり？
    pub kuitan: bool,
    /// 国士無双13面待ちをダブル役満にする？
    pub kokushi13_as_double: bool,
    /// 四暗刻単騎待ちをダブル役満にする？
    pub suankotanki_as_double: bool,
    /// 場1500？
    pub ba1500: bool,
    /// 満貫切り上げする？
    pub mangan_roundup: bool,
    /// 流し満貫あり？
    pub nagashimangan: bool,
}

// 面子の情報
#[derive(Clone)]
enum Mentsu {
    Pung { t: TileId },
    Chow { min: TileId },
    Ankan { t: TileId },
    Minkan { t: TileId },
    Anko { t: TileId },
    Syuntsu { min: TileId },
}

// 面子が切り分けられた手牌
struct DividedHand {
    atama: TileId,
    mentsu: Vec<Mentsu>,
}

// 得点計算の中間結果
struct ScoreTriple {
    han: i32,
    fu: i32,
    yaku: HashSet<Yaku>,
}

// 役満の翻数
const HAN_YAKUMAN: i32 = 13;
const HAN_2YAKUMAN: i32 = 26;
const HAN_3YAKUMAN: i32 = 39;
const HAN_4YAKUMAN: i32 = 52;
const HAN_5YAKUMAN: i32 = 65;
const HAN_6YAKUMAN: i32 = 78;
const HAN_7YAKUMAN: i32 = 91;

lazy_static! {
    // ルール設定
    static ref RULE_CONFIG: RwLock<ScoreRuleConfig> = RwLock::new(ScoreRuleConfig {
        kuitan: true,
        kokushi13_as_double: true,
        suankotanki_as_double: true,
        ba1500: false,
        mangan_roundup: false,
        nagashimangan: true,
    });

    // ドラ対応テーブル
    static ref DORATABLE: HashMap<TileId, TileId> = HashMap::from([
        (TileId::Id1man, TileId::Id2man),
        (TileId::Id2man, TileId::Id3man),
        (TileId::Id3man, TileId::Id4man),
        (TileId::Id4man, TileId::Id5man),
        (TileId::Id5man, TileId::Id6man),
        (TileId::Id6man, TileId::Id7man),
        (TileId::Id7man, TileId::Id8man),
        (TileId::Id8man, TileId::Id9man),
        (TileId::Id9man, TileId::Id1man),
        (TileId::Id1pin, TileId::Id2pin),
        (TileId::Id2pin, TileId::Id3pin),
        (TileId::Id3pin, TileId::Id4pin),
        (TileId::Id4pin, TileId::Id5pin),
        (TileId::Id5pin, TileId::Id6pin),
        (TileId::Id6pin, TileId::Id7pin),
        (TileId::Id7pin, TileId::Id8pin),
        (TileId::Id8pin, TileId::Id9pin),
        (TileId::Id9pin, TileId::Id1pin),
        (TileId::Id1sou, TileId::Id2sou),
        (TileId::Id2sou, TileId::Id3sou),
        (TileId::Id3sou, TileId::Id4sou),
        (TileId::Id4sou, TileId::Id5sou),
        (TileId::Id5sou, TileId::Id6sou),
        (TileId::Id6sou, TileId::Id7sou),
        (TileId::Id7sou, TileId::Id8sou),
        (TileId::Id8sou, TileId::Id9sou),
        (TileId::Id9sou, TileId::Id1sou),
        (TileId::IdTon, TileId::IdNan),
        (TileId::IdNan, TileId::IdSha),
        (TileId::IdSha, TileId::IdPee),
        (TileId::IdPee, TileId::IdTon),
        (TileId::IdHaku, TileId::IdHatu),
        (TileId::IdHatu, TileId::IdChun),
        (TileId::IdChun, TileId::IdHaku),
    ]);
}

// nの倍数へ切り上げ
fn roundup(val: i32, n: i32) -> i32 {
    ((val + n - 1) / n) * n
}

// ドラ数の数え上げ
fn count_num_doras(info: &AgariInformation) -> i32 {
    let update_num_dora = |t: Tile, num_dora: &mut i32| {
        if t.aka {
            *num_dora += 1;
        }
        for d in &info.dora.omote {
            if t.id == *DORATABLE.get(&d.id).unwrap() {
                *num_dora += 1;
            }
        }
        for d in &info.dora.ura {
            if t.id == *DORATABLE.get(&d.id).unwrap() {
                *num_dora += 1;
            }
        }
    };

    let mut ndoras: i32 = 0;

    // 和了牌
    update_num_dora(info.wining_tile, &mut ndoras);
    for t in &info.hand.hand {
        update_num_dora(*t, &mut ndoras);
    }
    for m in &info.hand.melds {
        match m {
            Meld::Pung { tiles } | Meld::Chow { tiles } => {
                for t in tiles {
                    update_num_dora(*t, &mut ndoras);
                }
            }
            Meld::Ankan { tiles } | Meld::Minkan { tiles } | Meld::Kakan { tiles } => {
                for t in tiles {
                    update_num_dora(*t, &mut ndoras);
                }
            }
        }
    }

    ndoras
}

/// 得点計算ルールの取得
pub fn get_rule_config() -> ScoreRuleConfig {
    *RULE_CONFIG.read().unwrap()
}

/// 得点計算ルールの設定
pub fn set_rule_config(config: &ScoreRuleConfig) {
    let mut current = RULE_CONFIG.write().unwrap();
    *current = *config;
}

/// 得点計算
pub fn calculate_score(info: &AgariInformation) -> Result<Score, Error> {
    // 面前でないのに立直
    if !info.hand.is_menzen() && (info.riichi || info.doubleriichi) {
        return Err(Error::from("Invalid agari: melded but also riichied."));
    }

    // 立直とダブルリーチが両立
    if info.riichi && info.doubleriichi {
        return Err(Error::from(
            "Invaild agari: riichi and double riichi occurd simultaneously.",
        ));
    }

    // 嶺上開花に自摸和了がついていない
    if info.rinshan && !info.tsumo {
        return Err(Error::from("Invaild agari: rinshan but not tsumo."));
    }

    // 流し満貫判定
    if RULE_CONFIG.read().unwrap().nagashimangan && info.nagashimangan {
        let score = Score {
            han: 5,
            fu: 0,
            yaku: HashSet::from([Yaku::Nagashimangan]),
            point: calculate_point(info, 5, 0),
        };
        return Ok(score);
    }

    // 手牌と和了牌を結合
    let marged = {
        let mut tmp = info.hand.marged_tiles();
        tmp.push(info.wining_tile.id);
        tmp
    };
    let marged_counts = calculate_tile_counts(marged.as_slice());

    // 向聴数チェック
    if calculate_shanten(marged.as_slice()) != -1 {
        return Err(Error::from("Invalid agari: specified hand is not agari"));
    }

    // 国士無双の翻/複合役計算
    if calculate_kokushimusou_shanten(marged.as_slice()) == -1 {
        return Ok(calculate_kokushimusou_score(info, &marged_counts));
    }

    // 七対子の翻/複合役計算
    if calculate_chitoitsu_shanten(marged.as_slice()) == -1 {
        return Ok(calculate_chitoitsu_score(info, &marged_counts));
    }

    // 国士無双以外の役満の翻/複合役計算
    if let Some(score) = calculate_yakuman_score(info, &marged_counts) {
        return Ok(score);
    }

    // 切り分けが必要な役の判定
    let mut score_triple = calculate_dividedhand_score(info);
    // 切り分けが不要な役の判定
    let (marged_yaku, marged_han) = calculate_margedand_yaku_han(info, &marged_counts);

    // 結果をマージ
    score_triple.han += marged_han;
    score_triple.yaku.extend(marged_yaku);

    // 役がついてない
    if score_triple.han == 0 {
        return Err(Error::from("Invalid agari: There are no yaku"));
    }

    Ok(Score {
        han: score_triple.han,
        fu: score_triple.fu,
        yaku: score_triple.yaku,
        point: calculate_point(info, score_triple.han, score_triple.fu),
    })
}

// 基本点計算
fn calculate_basic_point(han: i32, fu: i32) -> i32 {
    // 満貫切り上げ
    if RULE_CONFIG.read().unwrap().mangan_roundup {
        if (han == 4 && fu == 30) || (han == 3 && fu == 60) {
            return 2000;
        }
    }

    match han {
        1..=4 => {
            let tmp = fu * (1 << (han + 2));
            if tmp >= 2000 {
                2000
            } else {
                tmp
            }
        }
        5 => 2000,
        6..=7 => 3000,
        8..=10 => 4000,
        11..=12 => 6000,
        HAN_YAKUMAN => 8000,
        HAN_2YAKUMAN => 2 * 8000,
        HAN_3YAKUMAN => 3 * 8000,
        HAN_4YAKUMAN => 4 * 8000,
        HAN_5YAKUMAN => 5 * 8000,
        HAN_6YAKUMAN => 6 * 8000,
        HAN_7YAKUMAN => 7 * 8000,
        _ => {
            // 数え役満
            assert!(han >= 13);
            8000
        }
    }
}

// 得点計算
fn calculate_point(info: &AgariInformation, han: i32, fu: i32) -> Point {
    // 基本点計算
    let basic_point = calculate_basic_point(han, fu);
    // 積み棒得点
    let tsumibo_point = if RULE_CONFIG.read().unwrap().ba1500 {
        1500 * info.nhonba
    } else {
        300 * info.nhonba
    };
    // 供託立直棒得点
    let riichibo_point = 1000 * info.nriichi;

    match info.player {
        Wind::Ton => {
            if info.tsumo {
                // 親の自摸
                let payment = roundup(2 * basic_point, 100) + tsumibo_point / 3;
                Point {
                    get: 3 * payment + riichibo_point,
                    feed: Feed::Tsumo {
                        ko: payment,
                        oya: 0,
                    },
                }
            } else {
                // 親の和了
                let payment = roundup(6 * basic_point, 100) + tsumibo_point;
                Point {
                    get: payment + riichibo_point,
                    feed: Feed::Duck { point: payment },
                }
            }
        }
        _ => {
            if info.tsumo {
                // 子の自摸
                let ko_payment = roundup(basic_point, 100) + tsumibo_point / 3;
                let oya_payment = roundup(2 * basic_point, 100) + tsumibo_point / 3;
                Point {
                    get: 2 * ko_payment + oya_payment + riichibo_point,
                    feed: Feed::Tsumo {
                        ko: ko_payment,
                        oya: oya_payment,
                    },
                }
            } else {
                // 子の和了
                let payment = roundup(4 * basic_point, 100) + tsumibo_point;
                Point {
                    get: payment + riichibo_point,
                    feed: Feed::Duck { point: payment },
                }
            }
        }
    }
}

// 国士無双の翻/複合役計算
fn calculate_kokushimusou_score(info: &AgariInformation, counts: &TileCount) -> Score {
    let mut han = 0;
    let mut yaku: HashSet<Yaku> = HashSet::new();

    // 天和/地和判定
    if info.tenho {
        yaku.insert(Yaku::Tenho);
        han += HAN_YAKUMAN;
    } else if info.chiho {
        yaku.insert(Yaku::Chiho);
        han += HAN_YAKUMAN;
    }

    // 13面待ちか否か
    if is_kokushimusou13(info, counts) {
        yaku.insert(Yaku::Kokushimusou13);
        han += HAN_YAKUMAN;
        if RULE_CONFIG.read().unwrap().kokushi13_as_double {
            han += HAN_YAKUMAN;
        }
    } else {
        yaku.insert(Yaku::Kokushimusou);
        han += HAN_YAKUMAN;
    }

    Score {
        han: han,
        fu: 0,
        yaku: yaku,
        point: calculate_point(info, han, 0),
    }
}

// 七対子の翻/複合役計算
fn calculate_chitoitsu_score(info: &AgariInformation, counts: &TileCount) -> Score {
    let mut han = 0;
    let mut yaku: HashSet<Yaku> = HashSet::new();

    // 天和/地和判定
    if info.tenho {
        yaku.insert(Yaku::Tenho);
        han += HAN_YAKUMAN;
    } else if info.chiho {
        yaku.insert(Yaku::Chiho);
        han += HAN_YAKUMAN;
    }
    // 字一色
    if is_tsuiso(info, counts) {
        yaku.insert(Yaku::Tsuiso);
        han += HAN_YAKUMAN;
    }
    // 役満成立時は終わり
    if han >= HAN_YAKUMAN {
        return Score {
            han: han,
            fu: 0,
            yaku: yaku,
            point: calculate_point(info, han, 0),
        };
    }

    // 七対子の基本翻/符をセット
    yaku.insert(Yaku::Chitoitsu);
    han = 2;
    let fu = 25;

    // 自摸
    if info.tsumo {
        yaku.insert(Yaku::Tsumo);
        han += 1;
    }
    // 立直/ダブルリーチ
    if info.doubleriichi {
        yaku.insert(Yaku::Doubleriichi);
        han += 2;
    } else if info.riichi {
        yaku.insert(Yaku::Riichi);
        han += 1;
    }
    // 一発
    if info.riichi && info.ippatsu {
        yaku.insert(Yaku::Ippatsu);
        han += 1;
    }
    // 海底摸月/河底撈魚
    if info.haitei {
        if info.tsumo {
            yaku.insert(Yaku::Haiteitsumo);
        } else {
            yaku.insert(Yaku::Houteiron);
        }
        han += 1;
    }
    // ドラ
    let ndora = count_num_doras(info);
    if ndora > 0 {
        yaku.insert(Yaku::Dora);
        han += ndora;
    }
    // 混老頭
    if is_honrouto(info, counts) {
        yaku.insert(Yaku::Honrouto);
        han += 2;
    }
    // 清一色
    if is_chinitsu(info, counts) {
        yaku.insert(Yaku::Chinitsu);
        han += 6;
    }
    // 混一色
    if is_honitsu(info, counts) {
        yaku.insert(Yaku::Honitsu);
        han += 3;
    }
    // 断么九
    if is_tanyao(info, counts) {
        yaku.insert(Yaku::Tanyao);
        han += 1;
    }

    return Score {
        han: han,
        fu: fu,
        yaku: yaku,
        point: calculate_point(info, han, fu),
    };
}

// 国士無双以外の翻/複合役計算
fn calculate_yakuman_score(info: &AgariInformation, counts: &TileCount) -> Option<Score> {
    let mut yaku: HashSet<Yaku> = HashSet::new();
    let mut han = 0;

    // 天和
    if info.tenho {
        yaku.insert(Yaku::Tenho);
        han += HAN_YAKUMAN;
    }
    // 地和
    if info.chiho {
        yaku.insert(Yaku::Chiho);
        han += HAN_YAKUMAN;
    }
    // 九蓮宝燈
    if is_churenpouton(info, counts) {
        yaku.insert(Yaku::Churenpouton);
        han += HAN_YAKUMAN;
    }
    // 九蓮宝燈9面待ち
    if is_churenpouton9(info, counts) {
        yaku.insert(Yaku::Churenpouton9);
        han += HAN_2YAKUMAN;
    }
    // 四暗刻単騎待ち
    if is_suankotanki(info, counts) {
        yaku.insert(Yaku::Suankotanki);
        han += HAN_YAKUMAN;
        if RULE_CONFIG.read().unwrap().suankotanki_as_double {
            han += HAN_YAKUMAN;
        }
    }
    // 四暗刻
    if is_suanko(info, counts) {
        yaku.insert(Yaku::Suanko);
        han += HAN_YAKUMAN;
    }
    // 緑一色
    if is_ryuiso(info, counts) {
        yaku.insert(Yaku::Ryuiso);
        han += HAN_YAKUMAN;
    }
    // 清老頭
    if is_chinroto(info, counts) {
        yaku.insert(Yaku::Chinroto);
        han += HAN_YAKUMAN;
    }
    // 大四喜
    if is_daisushi(info, counts) {
        yaku.insert(Yaku::Daisushi);
        han += HAN_YAKUMAN;
    }
    // 小四喜
    if is_syosushi(info, counts) {
        yaku.insert(Yaku::Syosushi);
        han += HAN_YAKUMAN;
    }
    // 字一色
    if is_tsuiso(info, counts) {
        yaku.insert(Yaku::Tsuiso);
        han += HAN_YAKUMAN;
    }
    // 四槓子
    if is_sukantsu(info, counts) {
        yaku.insert(Yaku::Sukantsu);
        han += HAN_YAKUMAN;
    }
    // 大三元
    if is_daisangen(info, counts) {
        yaku.insert(Yaku::Daisangen);
        han += HAN_YAKUMAN;
    }

    // 役満成立か？
    if han >= HAN_YAKUMAN {
        return Some(Score {
            han: han,
            fu: 0,
            yaku: yaku,
            point: calculate_point(info, han, 0),
        });
    }

    None
}

// 切り分けが不要な役の判定/翻計算
fn calculate_margedand_yaku_han(
    info: &AgariInformation,
    counts: &TileCount,
) -> (HashSet<Yaku>, i32) {
    let mut yaku: HashSet<Yaku> = HashSet::new();
    let mut han = 0;

    // 門前自摸
    if info.hand.is_menzen() && info.tsumo {
        yaku.insert(Yaku::Tsumo);
        han += 1;
    }
    // 立直/ダブルリーチ
    if info.riichi {
        yaku.insert(Yaku::Riichi);
        han += 1;
    } else if info.doubleriichi {
        yaku.insert(Yaku::Doubleriichi);
        han += 2;
    }
    // 一発
    if info.ippatsu && (info.riichi || info.doubleriichi) {
        yaku.insert(Yaku::Ippatsu);
        han += 1;
    }
    // ドラ
    let ndora = count_num_doras(info);
    if ndora > 0 {
        yaku.insert(Yaku::Dora);
        han += ndora;
    }
    // 海底摸月/河底撈魚
    if info.haitei {
        if info.tsumo {
            yaku.insert(Yaku::Haiteitsumo);
        } else {
            yaku.insert(Yaku::Houteiron);
        }
        han += 1;
    }
    // 混老頭
    if is_honrouto(info, counts) {
        yaku.insert(Yaku::Honrouto);
        han += 2;
    }
    // 清一色
    if is_chinitsu(info, counts) {
        yaku.insert(Yaku::Chinitsu);
        // 食い下がり
        if info.hand.is_menzen() {
            han += 6;
        } else {
            han += 5;
        }
    }
    // 混一色
    if is_honitsu(info, counts) {
        yaku.insert(Yaku::Honitsu);
        // 食い下がり
        if info.hand.is_menzen() {
            han += 3;
        } else {
            han += 2;
        }
    }
    // 断么九
    if is_tanyao(info, counts) {
        yaku.insert(Yaku::Tanyao);
        han += 1;
    }
    // 三槓子
    if is_sankantsu(info, counts) {
        yaku.insert(Yaku::Sankantsu);
        han += 2;
    }
    // 小三元
    if is_syosangen(info, counts) {
        yaku.insert(Yaku::Syosangen);
        han += 2;
    }
    // 槍槓
    if info.chankan {
        yaku.insert(Yaku::Chankan);
        han += 1;
    }
    // 嶺上開花
    if info.rinshan {
        yaku.insert(Yaku::Rinshan);
        han += 1;
    }
    // 白
    if counts[&TileId::IdHaku] >= 3 {
        yaku.insert(Yaku::Haku);
        han += 1;
    }
    // 發
    if counts[&TileId::IdHatu] >= 3 {
        yaku.insert(Yaku::Hatu);
        han += 1;
    }
    // 中
    if counts[&TileId::IdChun] >= 3 {
        yaku.insert(Yaku::Chun);
        han += 1;
    }
    // 場風
    if (info.round == Wind::Ton && counts[&TileId::IdTon] >= 3)
        || (info.round == Wind::Nan && counts[&TileId::IdNan] >= 3)
        || (info.round == Wind::Sha && counts[&TileId::IdSha] >= 3)
        || (info.round == Wind::Pee && counts[&TileId::IdPee] >= 3)
    {
        yaku.insert(Yaku::Bakaze);
        han += 1;
    }
    // 自風
    if (info.player == Wind::Ton && counts[&TileId::IdTon] >= 3)
        || (info.player == Wind::Nan && counts[&TileId::IdNan] >= 3)
        || (info.player == Wind::Sha && counts[&TileId::IdSha] >= 3)
        || (info.player == Wind::Pee && counts[&TileId::IdPee] >= 3)
    {
        yaku.insert(Yaku::Jikaze);
        han += 1;
    }

    (yaku, han)
}

// 切り分けが必要な手の得点計算
// 高点法に従い、最大の得点を持つ役/翻/符を持つ結果を返す
fn calculate_dividedhand_score(info: &AgariInformation) -> ScoreTriple {
    // 副露牌を面子に読み替え
    let mut meld_mentsu: Vec<Mentsu> = vec![];
    for m in &info.hand.melds {
        match m {
            Meld::Pung { tiles } => {
                meld_mentsu.push(Mentsu::Pung { t: tiles[0].id });
            }
            Meld::Chow { tiles } => {
                meld_mentsu.push(Mentsu::Chow { min: tiles[0].id });
            }
            Meld::Ankan { tiles } => {
                meld_mentsu.push(Mentsu::Ankan { t: tiles[0].id });
            }
            Meld::Minkan { tiles } | Meld::Kakan { tiles } => {
                meld_mentsu.push(Mentsu::Minkan { t: tiles[0].id });
            }
        }
    }

    // 純手牌+和了牌をカウント
    let pure_counts = calculate_tile_counts(
        {
            let mut hand_counts = info.hand.hand.iter().map(|t| t.id).collect::<Vec<TileId>>();
            hand_counts.push(info.wining_tile.id);
            hand_counts
        }
        .as_slice(),
    );

    // 得点の初期化
    let mut score = ScoreTriple {
        han: 0,
        fu: 0,
        yaku: HashSet::<Yaku>::new(),
    };

    // 手牌の面子を切り分けながら翻/符の計算
    for (t, c) in &pure_counts {
        if *c >= 2 {
            let mut headless_counts = pure_counts.clone();
            headless_counts[t] -= 2;
            let mut div_hand = DividedHand {
                atama: *t,
                mentsu: meld_mentsu.clone(),
            };
            divide_mentsu(info, &mut headless_counts, &mut div_hand, &mut score)
        }
    }

    score
}

fn divide_mentsu(
    info: &AgariInformation,
    remain_counts: &mut TileCount,
    div_hand: &mut DividedHand,
    score: &mut ScoreTriple,
) {
    // 面子が4つになったら切り分け終了
    if div_hand.mentsu.len() >= 4 {
        // 役/翻/符の更新
        update_score_from_dividedhand(info, div_hand, score);
        return;
    }

    for (t, c) in remain_counts.iter() {
        // 暗刻を抜き出して調べる
        if *c >= 3 {
            let mut ankoless = remain_counts.clone();
            let kotsu = if (*t == info.wining_tile.id) && !info.tsumo {
                // 和了牌の場合は明刻に
                Mentsu::Pung { t: *t }
            } else {
                Mentsu::Anko { t: *t }
            };
            div_hand.mentsu.push(kotsu);
            ankoless[t] -= 3;
            divide_mentsu(info, &mut ankoless, div_hand, score);
            div_hand.mentsu.pop();
        }
        // 順子を抜き出して調べる
        if t.is_suhai()
            && t.getnumber() <= 7
            && remain_counts[&t.nth(0)] > 0
            && remain_counts[&t.nth(1)] > 0
            && remain_counts[&t.nth(2)] > 0
        {
            let mut syuntsuless = remain_counts.clone();
            div_hand.mentsu.push(Mentsu::Syuntsu { min: *t });
            syuntsuless[&t.nth(0)] -= 1;
            syuntsuless[&t.nth(1)] -= 1;
            syuntsuless[&t.nth(2)] -= 1;
            divide_mentsu(info, &mut syuntsuless, div_hand, score);
            div_hand.mentsu.pop();
        }
    }
}

// 得点計算と最大得点情報の更新
fn update_score_from_dividedhand(
    info: &AgariInformation,
    div_hand: &DividedHand,
    max_score: &mut ScoreTriple,
) {
    // 役/翻/符の計算
    let (yaku, han) = calculate_yaku_han_from_dividedhand(info, div_hand);
    let fu = calculate_fu_from_dividedhand(info, div_hand);

    // これまでで最大の翻/符であれば得点情報を更新
    if (han > max_score.han) || (han == max_score.han && fu > max_score.fu) {
        max_score.han = han;
        max_score.fu = fu;
        max_score.yaku = yaku;
    }
}

// 役と翻数計算
fn calculate_yaku_han_from_dividedhand(
    info: &AgariInformation,
    div_hand: &DividedHand,
) -> (HashSet<Yaku>, i32) {
    let mut yaku: HashSet<Yaku> = HashSet::new();
    let mut han = 0;

    // 平和
    if is_pinfu(info, div_hand) {
        yaku.insert(Yaku::Pinfu);
        han += 1;
    }
    // 二盃口
    if is_ryanpeko(info, div_hand) {
        yaku.insert(Yaku::Ryanpeko);
        han += 3;
    }
    // 一盃口
    if is_ipeko(info, div_hand) {
        yaku.insert(Yaku::Ipeko);
        han += 1;
    }
    // 一気通貫
    if is_ikkitsukan(info, div_hand) {
        yaku.insert(Yaku::Ikkitsukan);
        // 食い下がり
        if info.hand.is_menzen() {
            han += 2;
        } else {
            han += 1;
        }
    }
    // 三色同順
    if is_sansyokudoujyun(info, div_hand) {
        yaku.insert(Yaku::Sansyokudoujyun);
        // 食い下がり
        if info.hand.is_menzen() {
            han += 2;
        } else {
            han += 1;
        }
    }
    // 三色同刻
    if is_sansyokudoukoku(info, div_hand) {
        yaku.insert(Yaku::Sansyokudoukoku);
        han += 2;
    }
    // 純全帯么九
    if is_jyunchanta(info, div_hand) {
        yaku.insert(Yaku::Jyunchanta);
        // 食い下がり
        if info.hand.is_menzen() {
            han += 3;
        } else {
            han += 2;
        }
    }
    // 混全帯么九
    if is_chanta(info, div_hand) {
        yaku.insert(Yaku::Chanta);
        // 食い下がり
        if info.hand.is_menzen() {
            han += 2;
        } else {
            han += 1;
        }
    }
    // 対々和
    if is_toitoiho(info, div_hand) {
        yaku.insert(Yaku::Toitoiho);
        han += 2;
    }
    // 三暗刻
    if is_sananko(info, div_hand) {
        yaku.insert(Yaku::Sananko);
        han += 2;
    }

    (yaku, han)
}

// 符計算
fn calculate_fu_from_dividedhand(info: &AgariInformation, div_hand: &DividedHand) -> i32 {
    // 平和
    if is_pinfu(info, div_hand) {
        return if info.tsumo { 20 } else { 30 };
    }

    // 副底による符
    let mut fu = 20;
    // 門前による符
    if info.hand.is_menzen() && !info.tsumo {
        fu += 10;
    }
    // 自摸による符
    if info.tsumo {
        fu += 2;
    }
    // 待ち牌による符
    if info.wining_tile.id == div_hand.atama {
        fu += 2;
    } else if info.wining_tile.id.is_suhai() {
        for m in &div_hand.mentsu {
            let min = match m {
                Mentsu::Chow { min } | Mentsu::Syuntsu { min } => min,
                _ => {
                    continue;
                }
            };
            // 辺張/嵌張待ち
            if (info.wining_tile.id.getnumber() == 3 && info.wining_tile.id == min.nth(2))
                || (info.wining_tile.id.getnumber() == 7 && info.wining_tile.id == min.nth(0))
                || info.wining_tile.id == min.nth(1)
            {
                fu += 2;
                break;
            }
        }
    }
    // 雀頭の状態による符
    // 役牌
    if div_hand.atama.is_sangen()
        || (div_hand.atama == TileId::IdTon && info.round == Wind::Ton)
        || (div_hand.atama == TileId::IdNan && info.round == Wind::Nan)
        || (div_hand.atama == TileId::IdSha && info.round == Wind::Sha)
        || (div_hand.atama == TileId::IdPee && info.round == Wind::Pee)
    {
        fu += 2;
    }
    // 連風牌
    if (div_hand.atama == TileId::IdTon && info.player == Wind::Ton)
        || (div_hand.atama == TileId::IdNan && info.player == Wind::Nan)
        || (div_hand.atama == TileId::IdSha && info.player == Wind::Sha)
        || (div_hand.atama == TileId::IdPee && info.player == Wind::Pee)
    {
        fu += 2;
    }
    // 面子の構成による符
    for m in &div_hand.mentsu {
        match m {
            Mentsu::Anko { t } => {
                if t.is_yaochu() {
                    fu += 8;
                } else {
                    fu += 4
                }
            }
            Mentsu::Pung { t } => {
                if t.is_yaochu() {
                    fu += 4;
                } else {
                    fu += 2;
                }
            }
            Mentsu::Ankan { t } => {
                if t.is_yaochu() {
                    fu += 32;
                } else {
                    fu += 16;
                }
            }
            Mentsu::Minkan { t } => {
                if t.is_yaochu() {
                    fu += 16;
                } else {
                    fu += 8;
                }
            }
            _ => {}
        }
    }

    // ここまで副底の加符がないなら、30符に
    if fu == 20 {
        fu = 30;
    }

    // 1の位を切り上げたものが最終結果
    roundup(fu, 10)
}

// 盃口数（同一順子数）のカウント
fn count_num_peko(div_hand: &DividedHand) -> i32 {
    let mut syuntsu_count = TileCount::new();
    for m in &div_hand.mentsu {
        match m {
            Mentsu::Syuntsu { min } => {
                syuntsu_count[&min] += 1;
            }
            _ => {}
        }
    }

    let mut num_peko = 0;
    for (_, c) in &syuntsu_count {
        num_peko += *c / 2;
    }

    num_peko as i32
}

// 平和が成立しているか？
fn is_pinfu(info: &AgariInformation, hand: &DividedHand) -> bool {
    // 鳴いている（暗槓も不可）
    if info.hand.melds.len() > 0 {
        return false;
    }

    // 雀頭が役牌ではないか？
    match hand.atama {
        TileId::IdHaku | TileId::IdHatu | TileId::IdChun => {
            return false;
        }
        TileId::IdTon => {
            if info.player == Wind::Ton {
                return false;
            }
        }
        TileId::IdNan => {
            if info.player == Wind::Nan {
                return false;
            }
        }
        TileId::IdSha => {
            if info.player == Wind::Sha {
                return false;
            }
        }
        TileId::IdPee => {
            if info.player == Wind::Pee {
                return false;
            }
        }
        _ => {}
    }

    // 面子のチェック
    let mut is_established = false;
    for m in &hand.mentsu {
        match m {
            Mentsu::Syuntsu { min } => {
                if (info.wining_tile.id == min.nth(0) && min.nth(1).getnumber() != 8)
                    || (info.wining_tile.id == min.nth(2) && min.getnumber() != 1)
                {
                    is_established = true;
                }
            }
            _ => {
                // 順子以外が出現
                return false;
            }
        }
    }

    is_established
}

// 一盃口が成立しているか？
fn is_ipeko(info: &AgariInformation, hand: &DividedHand) -> bool {
    // 門前でない
    if !info.hand.is_menzen() {
        return false;
    }

    // 盃口数で判断
    count_num_peko(hand) == 1
}

// 三色同順が成立しているか？
fn is_sansyokudoujyun(_info: &AgariInformation, hand: &DividedHand) -> bool {
    for i in 0..4 {
        for j in (i + 1)..4 {
            for k in (j + 1)..4 {
                match hand.mentsu[i] {
                    Mentsu::Syuntsu { min: ti } | Mentsu::Chow { min: ti } => {
                        match hand.mentsu[j] {
                            Mentsu::Syuntsu { min: tj } | Mentsu::Chow { min: tj } => {
                                match hand.mentsu[k] {
                                    Mentsu::Syuntsu { min: tk } | Mentsu::Chow { min: tk } => {
                                        // 同じ数字かつ種類が全て異なる
                                        if (ti.getnumber() == tj.getnumber())
                                            && (tj.getnumber() == tk.getnumber())
                                            && (ti != tj)
                                            && (tj != tk)
                                            && (tk != ti)
                                        {
                                            return true;
                                        }
                                    }
                                    _ => {}
                                }
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    // 無かった
    false
}

// 一気通貫が成立しているか？
fn is_ikkitsukan(_info: &AgariInformation, hand: &DividedHand) -> bool {
    let mut suhai_count: [i8; 30] = [0; 30];

    for m in &hand.mentsu {
        match m {
            Mentsu::Syuntsu { min: t } | Mentsu::Chow { min: t } => {
                suhai_count[*t as usize] += 1;
                suhai_count[*t as usize + 1] += 1;
                suhai_count[*t as usize + 2] += 1;
            }
            _ => {}
        }
    }

    for orign in [TileId::Id1man, TileId::Id1pin, TileId::Id1sou] {
        for i in 0..=8 {
            if suhai_count[orign.nth(i) as usize] == 0 {
                break;
            }
            // 1つの種類についてすべて揃っている
            if i == 8 {
                return true;
            }
        }
    }

    // 揃っている種類が無かった
    false
}

// 全帯九が成立しているか？の共通処理
fn is_chanta_common(_info: &AgariInformation, hand: &DividedHand) -> bool {
    // 頭は么九牌か？
    if !hand.atama.is_yaochu() {
        return false;
    }

    // 全ての面子で么九牌が絡んでいるかチェック
    let mut syuntsu = false;
    for m in &hand.mentsu {
        match m {
            Mentsu::Chow { min } | Mentsu::Syuntsu { min } => {
                if (min.getnumber() != 1) && (min.getnumber() != 7) {
                    return false;
                }
                // 順子の出現をマーク（混老頭との複合防止）
                syuntsu = true;
            }
            Mentsu::Pung { t }
            | Mentsu::Anko { t }
            | Mentsu::Minkan { t }
            | Mentsu::Ankan { t } => {
                if !t.is_yaochu() {
                    return false;
                }
            }
        }
    }

    syuntsu
}

// 全帯么九が成立しているか？
fn is_chanta(info: &AgariInformation, hand: &DividedHand) -> bool {
    let mut jihai = false;

    // 雀頭は么九牌か？
    if !hand.atama.is_yaochu() {
        return false;
    } else if hand.atama.is_jihai() {
        jihai = true;
    }

    // 字牌の出現をチェック
    for m in &hand.mentsu {
        match m {
            Mentsu::Chow { min } | Mentsu::Syuntsu { min } => {
                if min.is_jihai() {
                    jihai = true;
                }
            }
            Mentsu::Pung { t }
            | Mentsu::Anko { t }
            | Mentsu::Minkan { t }
            | Mentsu::Ankan { t } => {
                if t.is_jihai() {
                    jihai = true;
                }
            }
        }
    }

    // 順子の並びは共通関数で判定
    return jihai && is_chanta_common(info, hand);
}

// 対々和が成立しているか？
fn is_toitoiho(_info: &AgariInformation, hand: &DividedHand) -> bool {
    // 順子が含まれていたら不成立
    for m in &hand.mentsu {
        match m {
            Mentsu::Chow { .. } | Mentsu::Syuntsu { .. } => {
                return false;
            }
            _ => {}
        }
    }

    true
}

// 三暗刻が成立しているか？
fn is_sananko(_info: &AgariInformation, hand: &DividedHand) -> bool {
    // 暗刻（暗槓）をカウント
    let mut num_anko = 0;
    for m in &hand.mentsu {
        match m {
            Mentsu::Anko { .. } | Mentsu::Ankan { .. } => {
                num_anko += 1;
            }
            _ => {}
        }
    }

    num_anko >= 3
}

// 三色同刻が成立しているか？
fn is_sansyokudoukoku(_info: &AgariInformation, hand: &DividedHand) -> bool {
    let mut num_dokoku = 0;

    for i in 0..4 {
        for j in (i + 1)..4 {
            match hand.mentsu[i] {
                Mentsu::Pung { t: ti }
                | Mentsu::Anko { t: ti }
                | Mentsu::Minkan { t: ti }
                | Mentsu::Ankan { t: ti } => match hand.mentsu[j] {
                    Mentsu::Pung { t: tj }
                    | Mentsu::Anko { t: tj }
                    | Mentsu::Minkan { t: tj }
                    | Mentsu::Ankan { t: tj } => {
                        if ti.is_suhai() && tj.is_suhai() && (ti.getnumber() == tj.getnumber()) {
                            num_dokoku += 1;
                        }
                    }
                    _ => {}
                },

                _ => {}
            }
        }
    }

    num_dokoku >= 3
}

// 純全帯么が成立しているか？
fn is_jyunchanta(info: &AgariInformation, hand: &DividedHand) -> bool {
    // 雀頭は老頭牌か？
    if !hand.atama.is_routou() {
        return false;
    }

    // 字牌が出てないか確認
    for m in &hand.mentsu {
        match m {
            Mentsu::Chow { min } | Mentsu::Syuntsu { min } => {
                if min.is_jihai() {
                    return false;
                }
            }
            Mentsu::Pung { t }
            | Mentsu::Anko { t }
            | Mentsu::Minkan { t }
            | Mentsu::Ankan { t } => {
                if t.is_jihai() {
                    return false;
                }
            }
        }
    }

    // 残りは共通関数で判定
    is_chanta_common(info, hand)
}

// 二盃口が成立しているか？
fn is_ryanpeko(info: &AgariInformation, hand: &DividedHand) -> bool {
    // 鳴いている（暗槓も不可）
    if info.hand.melds.len() > 0 {
        return false;
    }

    // 盃口数で判断
    count_num_peko(hand) == 2
}

// 断么九が成立しているか？
fn is_tanyao(info: &AgariInformation, counts: &TileCount) -> bool {
    // 喰いタン判定
    if !RULE_CONFIG.read().unwrap().kuitan && !info.hand.is_menzen() {
        return false;
    }

    // 么九牌が含まれていたら不成立
    for (t, _) in counts {
        if t.is_yaochu() {
            return false;
        }
    }

    true
}

// 混老頭が成立しているか？
fn is_honrouto(_info: &AgariInformation, counts: &TileCount) -> bool {
    let mut routou = false;
    let mut jihai = false;

    for (t, _) in counts {
        if !t.is_yaochu() {
            return false;
        } else {
            if t.is_routou() {
                routou = true;
            }
            if t.is_jihai() {
                jihai = true;
            }
        }
    }

    // 老頭牌と字牌両方の出現を要求
    routou && jihai
}

// 混一色が成立しているか？
fn is_honitsu(_info: &AgariInformation, counts: &TileCount) -> bool {
    let mut jihai = false;
    let mut ntypes = 0;
    let mut ty = TileType::MANZU;

    for (t, _) in counts {
        if t.is_jihai() {
            jihai = true;
        }
        if t.is_suhai() {
            // 最初に確認した種類を記録
            if ntypes == 0 {
                ntypes += 1;
                ty = t.gettype();
            }
            // 2種類以上ある
            if ty != t.gettype() {
                return false;
            }
        }
    }

    // 字牌の出現を要求
    jihai
}

// 清一色が成立しているか？
fn is_chinitsu(_info: &AgariInformation, counts: &TileCount) -> bool {
    let mut ntypes = 0;
    let mut ty = TileType::MANZU;

    for (t, _) in counts {
        // 字牌の出現は不可
        if t.is_jihai() {
            return false;
        }
        if t.is_suhai() {
            // 最初に確認した種類を記録
            if ntypes == 0 {
                ntypes += 1;
                ty = t.gettype();
            }
            // 2種類以上ある
            if ty != t.gettype() {
                return false;
            }
        }
    }

    true
}

// 小三元が成立しているか？
fn is_syosangen(_info: &AgariInformation, counts: &TileCount) -> bool {
    let mut sangen_head = false;
    let mut num_sangen_mentsu = 0;

    for (t, c) in counts {
        if t.is_sangen() {
            if *c == 2 {
                sangen_head = true;
            } else if *c >= 3 {
                num_sangen_mentsu += 1;
            }
        }
    }

    // 三元牌の頭と面子2を要求
    sangen_head && (num_sangen_mentsu == 2)
}

// 三槓子が成立しているか？
fn is_sankantsu(info: &AgariInformation, _counts: &TileCount) -> bool {
    // 3副露必須（簡易判定）
    if info.hand.melds.len() != 3 {
        return false;
    }

    // カン数のチェック
    info.hand.num_kan() == 3
}

// 国士無双13面待ちが成立しているか？
fn is_kokushimusou13(info: &AgariInformation, counts: &TileCount) -> bool {
    // 13面待ちの牌姿
    static PATTERN13: [TileId; 13] = [
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
    ];

    // 鳴いている（暗槓も不可）
    if info.hand.melds.len() > 0 {
        return false;
    }

    // 和了牌が么九牌か？
    if !info.wining_tile.id.is_yaochu() {
        return false;
    }

    // 和了牌を抜いたカウントを作成
    let mut removed = counts.clone();
    removed[&info.wining_tile.id] -= 1;

    // 和了牌がない状態で13面待ちの牌をすべて持っているか？
    for yc in PATTERN13 {
        if removed[&yc] != 1 {
            return false;
        }
    }

    true
}

// 九蓮宝燈が成立しているか？（共通関数）
fn is_churenpouton_common(counts: &TileCount) -> bool {
    for orign in [TileId::Id1man, TileId::Id1pin, TileId::Id1sou] {
        if counts[&orign] > 0 {
            // 1,9は3枚以上持つべし
            if (counts[&orign] < 3) || (counts[&orign.nth(8)] < 3) {
                return false;
            }
            // 2-8は1枚以上持つべし
            for i in 1..=7 {
                if counts[&orign.nth(i)] == 0 {
                    return false;
                }
            }
            // 成立確定
            return true;
        }
    }

    false
}

// 九蓮宝燈が成立しているか？
fn is_churenpouton(info: &AgariInformation, counts: &TileCount) -> bool {
    // 鳴いている
    if info.hand.melds.len() > 0 {
        return false;
    }

    // 9面待ちを弾く
    if info.wining_tile.id.is_chunchan() {
        if counts[&info.wining_tile.id] == 2 {
            return false;
        }
    } else if info.wining_tile.id.is_routou() {
        if counts[&info.wining_tile.id] == 4 {
            return false;
        }
    }

    is_churenpouton_common(counts)
}

// 九蓮宝燈9面街待ちが成立しているか？
fn is_churenpouton9(info: &AgariInformation, counts: &TileCount) -> bool {
    // 鳴いている
    if info.hand.melds.len() > 0 {
        return false;
    }

    // 和了牌を抜いてからチェック
    let mut removed = counts.clone();
    removed[&info.wining_tile.id] -= 1;
    is_churenpouton_common(&removed)
}

// 四暗刻が成立しているか？
fn is_suanko(info: &AgariInformation, counts: &TileCount) -> bool {
    // 門前でない or ツモ和了でない
    if !info.hand.is_menzen() || !info.tsumo {
        return false;
    }

    // 頭と和了牌は一致しない（単騎待ちを除外）
    if counts[&info.wining_tile.id] == 2 {
        return false;
    }

    // 暗刻と対子をカウント
    let mut num_anko = 0;
    let mut num_toitsu = 0;
    for (_, c) in counts {
        match *c {
            3 => {
                num_anko += 1;
            }
            2 => {
                num_toitsu += 1;
            }
            _ => {}
        }
    }

    return (num_anko == 4) && (num_toitsu == 1);
}

// 四暗刻単騎が成立しているか？
fn is_suankotanki(info: &AgariInformation, counts: &TileCount) -> bool {
    // 門前でない
    if !info.hand.is_menzen() {
        return false;
    }

    // 暗刻と対子をカウント
    let mut num_anko = 0;
    for (t, c) in counts {
        match *c {
            3 => {
                num_anko += 1;
            }
            2 => {
                // 和了牌のはず
                if *t != info.wining_tile.id {
                    return false;
                }
            }
            _ => {}
        }
    }

    num_anko == 4
}

// 大四喜が成立しているか？
fn is_daisushi(_info: &AgariInformation, counts: &TileCount) -> bool {
    // 東南西北が全て3枚以上あるか？
    (counts[&TileId::IdTon] >= 3)
        && (counts[&TileId::IdNan] >= 3)
        && (counts[&TileId::IdSha] >= 3)
        && (counts[&TileId::IdPee] >= 3)
}

// 小四喜が成立しているか？
fn is_syosushi(_info: &AgariInformation, counts: &TileCount) -> bool {
    // 東南西北の暗刻と対子をカウント
    let mut num_anko = 0;
    let mut num_toitsu = 0;
    for t in 0..=3 {
        match counts[&TileId::IdTon.nth(t)] {
            3 => {
                num_anko += 1;
            }
            2 => {
                num_toitsu += 1;
            }
            _ => {}
        }
    }

    (num_anko == 3) && (num_toitsu == 1)
}

// 大三元が成立しているか？
fn is_daisangen(_info: &AgariInformation, counts: &TileCount) -> bool {
    // 白發中がすべて3枚以上あるか？
    (counts[&TileId::IdHaku] >= 3)
        && (counts[&TileId::IdHatu] >= 3)
        && (counts[&TileId::IdChun] >= 3)
}

// 字一色が成立しているか？
fn is_tsuiso(_info: &AgariInformation, counts: &TileCount) -> bool {
    // 字牌以外が含まれていたら不成立
    for (t, _) in counts {
        if !t.is_jihai() {
            return false;
        }
    }
    true
}

// 清老頭が成立しているか？
fn is_chinroto(_info: &AgariInformation, counts: &TileCount) -> bool {
    // 老頭牌以外が含まれていたら不成立
    for (t, _) in counts {
        if !t.is_routou() {
            return false;
        }
    }
    true
}

// 緑一色が成立しているか？
fn is_ryuiso(_info: &AgariInformation, counts: &TileCount) -> bool {
    // 緑牌以外が含まれていたら不成立
    for (t, _) in counts {
        match *t {
            TileId::Id2sou
            | TileId::Id3sou
            | TileId::Id4sou
            | TileId::Id6sou
            | TileId::Id8sou
            | TileId::IdHatu => {}
            _ => {
                return false;
            }
        }
    }
    true
}

// 四槓子が成立しているか？
fn is_sukantsu(info: &AgariInformation, _counts: &TileCount) -> bool {
    // 4副露必須（簡易判定）
    if info.hand.melds.len() != 4 {
        return false;
    }

    // カン数のチェック
    info.hand.num_kan() == 4
}
