use mjtools::types::*;

#[test]
fn test_from_tilestr() {
    assert_eq!(
        TileId::from_tilestr("🀇🀈🀉🀊🀋🀌🀍🀎🀏").unwrap(),
        [
            TileId::Id1man,
            TileId::Id2man,
            TileId::Id3man,
            TileId::Id4man,
            TileId::Id5man,
            TileId::Id6man,
            TileId::Id7man,
            TileId::Id8man,
            TileId::Id9man,
        ]
    );
    assert_eq!(
        TileId::from_tilestr("🀐🀑🀒🀓🀔🀕🀖🀗🀘").unwrap(),
        [
            TileId::Id1sou,
            TileId::Id2sou,
            TileId::Id3sou,
            TileId::Id4sou,
            TileId::Id5sou,
            TileId::Id6sou,
            TileId::Id7sou,
            TileId::Id8sou,
            TileId::Id9sou,
        ]
    );
    assert_eq!(
        TileId::from_tilestr("🀙🀚🀛🀜🀝🀞🀟🀠🀡").unwrap(),
        [
            TileId::Id1pin,
            TileId::Id2pin,
            TileId::Id3pin,
            TileId::Id4pin,
            TileId::Id5pin,
            TileId::Id6pin,
            TileId::Id7pin,
            TileId::Id8pin,
            TileId::Id9pin,
        ]
    );
    assert_eq!(
        TileId::from_tilestr("🀀🀁🀂🀃").unwrap(),
        [TileId::IdTon, TileId::IdNan, TileId::IdSha, TileId::IdPee]
    );
    assert_eq!(
        TileId::from_tilestr("🀆🀅🀄").unwrap(),
        [TileId::IdHaku, TileId::IdHatu, TileId::IdChun]
    );
}

#[test]
#[should_panic]
fn test_from_invalid_char() {
    TileId::from_char('🀢').unwrap();
}

#[test]
fn test_from_haifustr() {
    assert_eq!(
        Tile::from_mjscorestr("1m2m3m4m5m6m7m8m9m").unwrap(),
        [
            Tile {
                id: TileId::Id1man,
                aka: false
            },
            Tile {
                id: TileId::Id2man,
                aka: false
            },
            Tile {
                id: TileId::Id3man,
                aka: false
            },
            Tile {
                id: TileId::Id4man,
                aka: false
            },
            Tile {
                id: TileId::Id5man,
                aka: false
            },
            Tile {
                id: TileId::Id6man,
                aka: false
            },
            Tile {
                id: TileId::Id7man,
                aka: false
            },
            Tile {
                id: TileId::Id8man,
                aka: false
            },
            Tile {
                id: TileId::Id9man,
                aka: false
            }
        ]
    );
    assert_eq!(
        Tile::from_mjscorestr("1s2s3s4s5s6s7s8s9s").unwrap(),
        [
            Tile {
                id: TileId::Id1sou,
                aka: false
            },
            Tile {
                id: TileId::Id2sou,
                aka: false
            },
            Tile {
                id: TileId::Id3sou,
                aka: false
            },
            Tile {
                id: TileId::Id4sou,
                aka: false
            },
            Tile {
                id: TileId::Id5sou,
                aka: false
            },
            Tile {
                id: TileId::Id6sou,
                aka: false
            },
            Tile {
                id: TileId::Id7sou,
                aka: false
            },
            Tile {
                id: TileId::Id8sou,
                aka: false
            },
            Tile {
                id: TileId::Id9sou,
                aka: false
            }
        ]
    );
    assert_eq!(
        Tile::from_mjscorestr("1p2p3p4p5p6p7p8p9p").unwrap(),
        [
            Tile {
                id: TileId::Id1pin,
                aka: false
            },
            Tile {
                id: TileId::Id2pin,
                aka: false
            },
            Tile {
                id: TileId::Id3pin,
                aka: false
            },
            Tile {
                id: TileId::Id4pin,
                aka: false
            },
            Tile {
                id: TileId::Id5pin,
                aka: false
            },
            Tile {
                id: TileId::Id6pin,
                aka: false
            },
            Tile {
                id: TileId::Id7pin,
                aka: false
            },
            Tile {
                id: TileId::Id8pin,
                aka: false
            },
            Tile {
                id: TileId::Id9pin,
                aka: false
            }
        ]
    );
    assert_eq!(
        Tile::from_mjscorestr("東南西北").unwrap(),
        [
            Tile {
                id: TileId::IdTon,
                aka: false
            },
            Tile {
                id: TileId::IdNan,
                aka: false
            },
            Tile {
                id: TileId::IdSha,
                aka: false
            },
            Tile {
                id: TileId::IdPee,
                aka: false
            }
        ]
    );
    assert_eq!(
        Tile::from_mjscorestr("白発中").unwrap(),
        [
            Tile {
                id: TileId::IdHaku,
                aka: false
            },
            Tile {
                id: TileId::IdHatu,
                aka: false
            },
            Tile {
                id: TileId::IdChun,
                aka: false
            },
        ]
    );
    assert_eq!(
        Tile::from_mjscorestr("5M5P5S").unwrap(),
        [
            Tile {
                id: TileId::Id5man,
                aka: true
            },
            Tile {
                id: TileId::Id5pin,
                aka: true
            },
            Tile {
                id: TileId::Id5sou,
                aka: true
            },
        ]
    );
}

#[test]
#[should_panic]
fn test_from_invalid_mjscorestr() {
    Tile::from_mjscorestr("發").unwrap();
    Tile::from_mjscorestr("1").unwrap();
}
