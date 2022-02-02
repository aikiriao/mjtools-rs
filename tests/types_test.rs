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
