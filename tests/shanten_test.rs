use mjtools::shanten::*;
use mjtools::types::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[test]
fn test_shanten_problems() {
    let problem_files = [
        "./data/p_hon_10000.txt",
        "./data/p_koku_10000.txt",
        "./data/p_normal_10000.txt",
        "./data/p_tin_10000.txt",
    ];
    let tileidmap: HashMap<i32, TileId> = HashMap::from([
        (0, TileId::Id1man),
        (1, TileId::Id2man),
        (2, TileId::Id3man),
        (3, TileId::Id4man),
        (4, TileId::Id5man),
        (5, TileId::Id6man),
        (6, TileId::Id7man),
        (7, TileId::Id8man),
        (8, TileId::Id9man),
        (9, TileId::Id1pin),
        (10, TileId::Id2pin),
        (11, TileId::Id3pin),
        (12, TileId::Id4pin),
        (13, TileId::Id5pin),
        (14, TileId::Id6pin),
        (15, TileId::Id7pin),
        (16, TileId::Id8pin),
        (17, TileId::Id9pin),
        (18, TileId::Id1sou),
        (19, TileId::Id2sou),
        (20, TileId::Id3sou),
        (21, TileId::Id4sou),
        (22, TileId::Id5sou),
        (23, TileId::Id6sou),
        (24, TileId::Id7sou),
        (25, TileId::Id8sou),
        (26, TileId::Id9sou),
        (27, TileId::IdTon),
        (28, TileId::IdNan),
        (29, TileId::IdSha),
        (30, TileId::IdPee),
        (31, TileId::IdHaku),
        (32, TileId::IdHatu),
        (33, TileId::IdChun),
    ]);

    for filename in problem_files {
        let file = match File::open(filename) {
            Err(e) => panic!("couldn't open testfile: {}", e),
            Ok(f) => f,
        };
        for line in BufReader::new(file).lines() {
            let line_str = line.unwrap().clone();
            let entry: Vec<i32> = line_str
                .split(' ')
                .map(|s| s.parse::<i32>().unwrap())
                .collect();
            let tiles: Vec<TileId> =
                entry[0..14]
                    .iter()
                    .fold(Vec::<TileId>::new(), |mut vec, t| {
                        vec.push(*tileidmap.get(&t).unwrap());
                        vec
                    });
            let normal_answer = entry[14];
            let kokushi_answer = entry[15];
            let chitoi_answer = entry[16];
            assert_eq!(normal_answer, calculate_normal_shanten(tiles.as_slice()));
            assert_eq!(
                kokushi_answer,
                calculate_kokushimusou_shanten(tiles.as_slice())
            );
            assert_eq!(chitoi_answer, calculate_chitoitsu_shanten(tiles.as_slice()));
        }
    }
}

#[test]
fn test_effective_tiles() {
    struct TestCase(&'static str, &'static str);
    let normal_tests = [
        TestCase("🀉🀊🀋🀏🀑🀒🀔🀔🀔🀕🀖🀅🀅", "🀐🀓🀔🀗🀅"),
        TestCase("🀇🀎🀎🀛🀛🀜🀞🀟🀠🀡🀐🀐🀒", "🀎🀛🀝🀐🀑"),
        TestCase("🀈🀊🀌🀎🀚🀚🀡🀑🀒🀓🀔🀖🀆", "🀉🀍🀕"),
        TestCase("🀈🀊🀌🀎🀚🀚🀝🀑🀒🀓🀔🀖🀆", "🀉🀍🀕"),
        TestCase("🀈🀊🀌🀎🀚🀚🀝🀡🀑🀒🀓🀔🀖", "🀉🀍🀕"),
    ];
    let chitoitsu_tests = [
        TestCase("🀊🀊🀙🀜🀐🀒🀒🀔🀔🀖🀗🀀🀂", "🀙🀜🀐🀖🀗🀀🀂"),
        TestCase("🀉🀉🀍🀏🀏🀛🀜🀜🀐🀑🀑🀔🀂", "🀍🀛🀐🀔🀂"),
        TestCase("🀚🀝🀝🀞🀞🀡🀐🀓🀗🀘🀘🀂🀆", "🀚🀡🀐🀓🀗🀂🀆"),
        TestCase("🀝🀝🀝🀞🀞🀡🀐🀓🀗🀘🀘🀂🀆", "🀡🀐🀓🀗🀂🀆"),
    ];
    let kokushimusou_tests = [
        TestCase("🀇🀏🀙🀡🀐🀘🀀🀁🀂🀃🀆🀅🀄", "🀇🀏🀙🀡🀐🀘🀀🀁🀂🀃🀆🀅🀄"),
        TestCase("🀇🀏🀙🀜🀠🀠🀐🀖🀗🀀🀂🀆🀆", "🀡🀘🀁🀃🀅🀄"),
    ];

    for case in normal_tests {
        assert_eq!(
            listup_normal_effective_tiles(TileId::from_str(case.0).unwrap().as_slice()).unwrap(),
            TileId::from_str(case.1).unwrap()
        );
    }

    for case in chitoitsu_tests {
        assert_eq!(
            listup_chitoitsu_effective_tiles(TileId::from_str(case.0).unwrap().as_slice()).unwrap(),
            TileId::from_str(case.1).unwrap()
        );
    }

    for case in kokushimusou_tests {
        assert_eq!(
            listup_kokushimusou_effective_tiles(TileId::from_str(case.0).unwrap().as_slice())
                .unwrap(),
            TileId::from_str(case.1).unwrap()
        );
    }
}
