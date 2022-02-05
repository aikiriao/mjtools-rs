use counter::Counter;
use num_traits::FromPrimitive;

/// 牌の出現カウント
pub type TileCount = Counter<TileId, i8>;

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

/// 数牌の種類
#[derive(PartialEq, Eq)]
pub enum TileType {
    /// 萬子
    MANZU,
    /// 筒子
    PINZU,
    /// 索子
    SOUZU,
}

/// 牌の識別子
#[derive(Copy, Clone, PartialOrd, PartialEq, Hash, Eq, Debug, FromPrimitive)]
pub enum TileId {
    /// 一萬
    Id1man = 1,
    /// 二萬
    Id2man = 2,
    /// 三萬
    Id3man = 3,
    /// 四萬
    Id4man = 4,
    /// 五萬    
    Id5man = 5,
    /// 六萬
    Id6man = 6,
    /// 七萬
    Id7man = 7,
    /// 八萬
    Id8man = 8,
    /// 九萬
    Id9man = 9,
    /// 一筒
    Id1pin = 11,
    /// 二筒
    Id2pin = 12,
    /// 三筒
    Id3pin = 13,
    /// 四筒
    Id4pin = 14,
    /// 五筒
    Id5pin = 15,
    /// 六筒
    Id6pin = 16,
    /// 七筒
    Id7pin = 17,
    /// 八筒
    Id8pin = 18,
    /// 九筒
    Id9pin = 19,
    /// 一索
    Id1sou = 21,
    /// 二索
    Id2sou = 22,
    /// 三索
    Id3sou = 23,
    /// 四索
    Id4sou = 24,
    /// 五索
    Id5sou = 25,
    /// 六索
    Id6sou = 26,
    /// 七索
    Id7sou = 27,
    /// 八索
    Id8sou = 28,
    /// 九索
    Id9sou = 29,
    /// 東
    IdTon = 31,
    /// 南
    IdNan = 32,
    /// 西
    IdSha = 33,
    /// 北
    IdPee = 34,
    /// 白
    IdHaku = 35,
    /// 發
    IdHatu = 36,
    /// 中
    IdChun = 37,
}

/// 牌
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Tile {
    /// 牌識別子
    pub id: TileId,
    /// 赤ドラか？    
    pub aka: bool,
}

/// 風（場）
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Wind {
    /// 東
    Ton,
    /// 南
    Nan,
    /// 西
    Sha,
    /// 北
    Pee,
}

/// 副露
#[derive(Clone, Copy)]
pub enum Meld {
    /// ポン
    Pung { tiles: [Tile; 3] },
    /// チー
    Chow { tiles: [Tile; 3] },
    /// 暗槓（アンカン）
    Ankan { tiles: [Tile; 4] },
    /// 大明槓（ミンカン）
    Minkan { tiles: [Tile; 4] },
    /// 加槓（カカン）
    Kakan { tiles: [Tile; 4] },
}

/// 手牌
pub struct Hand {
    /// 純手牌
    pub hand: Vec<Tile>,
    /// 副露
    pub melds: Vec<Meld>,
}

/// ドラ表示牌
pub struct Dora {
    /// 表ドラ
    pub omote: Vec<Tile>,
    /// 裏ドラ
    pub ura: Vec<Tile>,
}

impl Hand {
    /// 門前か？
    pub fn is_menzen(&self) -> bool {
        for m in &self.melds {
            match m {
                Meld::Ankan { .. } => {
                    continue;
                }
                _ => {
                    return false;
                }
            }
        }
        true
    }

    /// カン数をカウント
    pub fn num_kan(&self) -> i32 {
        let mut num = 0;
        for m in &self.melds {
            match m {
                Meld::Ankan { .. } | Meld::Minkan { .. } | Meld::Kakan { .. } => {
                    num += 1;
                }
                _ => {}
            }
        }
        num
    }

    /// 鳴いた牌を結合
    pub fn marged_tiles(&self) -> Vec<TileId> {
        let mut marged: Vec<TileId> = self.hand.iter().map(|Tile { id, .. }| *id).collect();
        for m in &self.melds {
            let ts: Vec<TileId> = match m {
                Meld::Pung { tiles: ts } | Meld::Chow { tiles: ts } => {
                    ts.iter().map(|Tile { id, .. }| *id).collect()
                }
                Meld::Ankan { tiles: ts }
                | Meld::Minkan { tiles: ts }
                | Meld::Kakan { tiles: ts } => ts.iter().map(|Tile { id, .. }| *id).collect(),
            };
            marged.extend_from_slice(ts.as_slice());
        }
        marged
    }
}

impl TileId {
    /// 識別子の相対位置
    pub fn nth(&self, index: i32) -> Self {
        FromPrimitive::from_i32(*self as i32 + index).unwrap()
    }

    /// 字牌か？
    pub fn is_jihai(&self) -> bool {
        *self >= TileId::IdTon && *self <= TileId::IdChun
    }

    /// 数牌か？
    pub fn is_suhai(&self) -> bool {
        (*self >= TileId::Id1man && *self <= TileId::Id9man)
            || (*self >= TileId::Id1pin && *self <= TileId::Id9pin)
            || (*self >= TileId::Id1sou && *self <= TileId::Id9sou)
    }

    /// 中張牌か？
    pub fn is_chunchan(&self) -> bool {
        (*self >= TileId::Id2man && *self <= TileId::Id8man)
            || (*self >= TileId::Id2pin && *self <= TileId::Id8pin)
            || (*self >= TileId::Id2sou && *self <= TileId::Id8sou)
    }

    /// 老頭牌か？
    pub fn is_routou(&self) -> bool {
        self.is_suhai() && !self.is_chunchan()
    }

    /// 么九牌か？
    pub fn is_yaochu(&self) -> bool {
        self.is_routou() || self.is_jihai()
    }

    /// 三元牌か？
    pub fn is_sangen(&self) -> bool {
        (*self == TileId::IdHaku) || (*self == TileId::IdHatu) || (*self == TileId::IdChun)
    }

    /// 数牌の数字取得
    pub fn getnumber(&self) -> i32 {
        assert!(self.is_suhai());
        *self as i32 % 10
    }

    /// 数牌の種類取得
    pub fn gettype(&self) -> TileType {
        let ty = *self as i32 / 10;
        match ty {
            0 => TileType::MANZU,
            1 => TileType::PINZU,
            2 => TileType::SOUZU,
            _ => panic!("must be suhai"),
        }
    }

    /// 文字を識別子に変換
    pub fn from_char(c: char) -> Result<Self, Error> {
        let ret = match c {
            '\u{1f000}' => TileId::IdTon,
            '\u{1f001}' => TileId::IdNan,
            '\u{1f002}' => TileId::IdSha,
            '\u{1f003}' => TileId::IdPee,
            '\u{1f004}' => TileId::IdChun,
            '\u{1f005}' => TileId::IdHatu,
            '\u{1f006}' => TileId::IdHaku,
            '\u{1f007}' => TileId::Id1man,
            '\u{1f008}' => TileId::Id2man,
            '\u{1f009}' => TileId::Id3man,
            '\u{1f00a}' => TileId::Id4man,
            '\u{1f00b}' => TileId::Id5man,
            '\u{1f00c}' => TileId::Id6man,
            '\u{1f00d}' => TileId::Id7man,
            '\u{1f00e}' => TileId::Id8man,
            '\u{1f00f}' => TileId::Id9man,
            '\u{1f010}' => TileId::Id1sou,
            '\u{1f011}' => TileId::Id2sou,
            '\u{1f012}' => TileId::Id3sou,
            '\u{1f013}' => TileId::Id4sou,
            '\u{1f014}' => TileId::Id5sou,
            '\u{1f015}' => TileId::Id6sou,
            '\u{1f016}' => TileId::Id7sou,
            '\u{1f017}' => TileId::Id8sou,
            '\u{1f018}' => TileId::Id9sou,
            '\u{1f019}' => TileId::Id1pin,
            '\u{1f01a}' => TileId::Id2pin,
            '\u{1f01b}' => TileId::Id3pin,
            '\u{1f01c}' => TileId::Id4pin,
            '\u{1f01d}' => TileId::Id5pin,
            '\u{1f01e}' => TileId::Id6pin,
            '\u{1f01f}' => TileId::Id7pin,
            '\u{1f020}' => TileId::Id8pin,
            '\u{1f021}' => TileId::Id9pin,
            _ => {
                return Err(Error::from("invalid Mahjong char"));
            }
        };
        Ok(ret)
    }

    /// 牌文字列を識別子に変換
    pub fn from_tilestr(s: &str) -> Result<Vec<Self>, Error> {
        let mut tiles = vec![];
        for c in s.chars() {
            tiles.push(TileId::from_char(c)?);
        }
        Ok(tiles)
    }
}

impl Tile {
    /// mjscoreの牌表示から牌に変換
    pub fn from_mjscorestr(s: &str) -> Result<Vec<Self>, Error> {
        let mut tiles = vec![];
        let mut chars_iter = s.chars();
        while let Some(c) = chars_iter.next() {
            match c {
                '1'..='9' => {
                    let ty = chars_iter.next().unwrap();
                    let offset = c.to_digit(10).unwrap() as i32 - 1;
                    let id = match ty.to_ascii_lowercase() {
                        'm' => TileId::Id1man.nth(offset),
                        'p' => TileId::Id1pin.nth(offset),
                        's' => TileId::Id1sou.nth(offset),
                        _ => {
                            return Err(Error::from("invalid mjscore hai char"));
                        }
                    };
                    tiles.push(Tile {
                        id,
                        aka: ty.is_uppercase(),
                    });
                }
                '東' => {
                    tiles.push(Tile {
                        id: TileId::IdTon,
                        aka: false,
                    });
                }
                '南' => {
                    tiles.push(Tile {
                        id: TileId::IdNan,
                        aka: false,
                    });
                }
                '西' => {
                    tiles.push(Tile {
                        id: TileId::IdSha,
                        aka: false,
                    });
                }
                '北' => {
                    tiles.push(Tile {
                        id: TileId::IdPee,
                        aka: false,
                    });
                }
                '白' => {
                    tiles.push(Tile {
                        id: TileId::IdHaku,
                        aka: false,
                    });
                }
                '発' => {
                    tiles.push(Tile {
                        id: TileId::IdHatu,
                        aka: false,
                    });
                }
                '中' => {
                    tiles.push(Tile {
                        id: TileId::IdChun,
                        aka: false,
                    });
                }
                _ => {
                    return Err(Error::from("invalid mjscore hai char"));
                }
            }
        }
        Ok(tiles)
    }
}

/// 牌の出現回数を計算
pub fn calculate_tile_counts(tiles: &[TileId]) -> TileCount {
    tiles.iter().cloned().collect()
}
