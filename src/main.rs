extern crate clap;
use clap::{Command, AppSettings, Arg};
use mjtools::score::*;
use mjtools::shanten::*;
use mjtools::types::*;
use std::collections::HashMap;

fn main() {
    let command = Command::new("mjtools")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Mahjong analyzing tools")
        .subcommand(
            Command::new("shanten")
                .about("Calculate shanten and listup effective tiles")
                .arg(Arg::new("hand").help("Specify hand tiles").required(true)),
        )
        .subcommand(
            Command::new("score")
                .about("Calculate score")
                .setting(AppSettings::DeriveDisplayOrder)
                .arg(Arg::new("hand").help("specify hand tiles").required(true))
                .arg(
                    Arg::new("wining tile")
                        .help("Specify wining(tsumo/ron) tile")
                        .required(true),
                )
                .arg(
                    Arg::new("player wind")
                        .help("Specify player wind(ton or nan or sha or pee)")
                        .long("player")
                        .short('p')
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::new("round wind")
                        .help("Specify round wind(ton or nan or sha or pee)")
                        .long("round")
                        .short('r')
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::new("chow")
                        .help("Specify chow tiles")
                        .long("chow")
                        .takes_value(true),
                )
                .arg(
                    Arg::new("pung")
                        .help("Specify pung tiles")
                        .long("pung")
                        .takes_value(true),
                )
                .arg(
                    Arg::new("ankan")
                        .help("Specify ankan tiles")
                        .long("ankan")
                        .takes_value(true),
                )
                .arg(
                    Arg::new("minkan")
                        .help("Specify minkan/kakan tiles")
                        .long("minkan")
                        .takes_value(true),
                )
                .arg(
                    Arg::new("omote dora")
                        .help("Specify omote dora tiles")
                        .long("omotedora")
                        .takes_value(true),
                )
                .arg(
                    Arg::new("ura dora")
                        .help("Specify ura dora tiles")
                        .long("uradora")
                        .takes_value(true),
                )
                .arg(
                    Arg::new("nhonba")
                        .help("Specify n honba")
                        .long("nhonba")
                        .takes_value(true),
                )
                .arg(
                    Arg::new("nriichi")
                        .help("Specify number of riichi bar")
                        .long("nriichi")
                        .takes_value(true),
                )
                .arg(
                    Arg::new("tsumo")
                        .help("Specify whether tsumo or not(ron)")
                        .long("tsumo"),
                )
                .arg(
                    Arg::new("riichi")
                        .help("Specify whether riichi or not")
                        .long("riichi"),
                )
                .arg(
                    Arg::new("ippatsu")
                        .help("Specify whether ippatsu or not")
                        .long("ippatsu"),
                )
                .arg(
                    Arg::new("doubleriichi")
                        .help("Specify whether double riichi or not")
                        .long("doubleriichi"),
                )
                .arg(
                    Arg::new("haitei")
                        .help("Specify whether haitei or not")
                        .long("haitei"),
                )
                .arg(
                    Arg::new("rinshan")
                        .help("Specify whether rinshan or not")
                        .long("rinshan"),
                )
                .arg(
                    Arg::new("chankan")
                        .help("Specify whether chankan or not")
                        .long("chankan"),
                ),
        );

    // 引数を解析
    let matches = command.get_matches();

    // 向聴数計算
    if let Some(matches) = matches.subcommand_matches("shanten") {
        // 手牌
        let hand =
            Tile::from_mjscorestr(matches.value_of("hand").unwrap()).expect("Faild to parse hand");
        let idmarged: Vec<TileId> = hand.iter().map(|t| t.id).collect();
        // 向聴数計算
        let shanten = calculate_shanten(&idmarged);
        println!(
            "Shanten: {} {}",
            shanten,
            match shanten {
                -1 => {
                    "(agari)"
                }
                0 => {
                    "(tenpai)"
                }
                _ => "",
            }
        );
        // 有効牌列挙（自摸前に可能）
        if idmarged.len() == 13 {
            let effective_tiles =
                listup_effective_tiles(&idmarged).expect("Faild to listup effective tiles");
            let effective_tiles_str: Vec<String> = effective_tiles
                .into_iter()
                .map(|id| Tile { id, aka: false }.to_mjscorestr())
                .collect();
            print!("Effective tiles: ");
            for s in effective_tiles_str {
                print!("{}, ", s)
            }
            println!();
        }
    }

    // 得点計算
    if let Some(matches) = matches.subcommand_matches("score") {
        let strwindmap = HashMap::from([
            ("ton", Wind::Ton),
            ("nan", Wind::Nan),
            ("sha", Wind::Sha),
            ("pee", Wind::Pee),
        ]);
        let mut melds: Vec<Meld> = vec![];
        let mut dora = Dora {
            omote: vec![],
            ura: vec![],
        };
        // 純手牌
        let hand =
            Tile::from_mjscorestr(matches.value_of("hand").unwrap()).expect("Faild to parse hand");
        // 和了牌
        let get = Tile::from_mjscorestr(matches.value_of("wining tile").unwrap())
            .expect("Faild to parse wining tile");
        if get.len() > 1 {
            panic!("wining tile must be one");
        }
        let wining_tile = get[0];
        // 自風
        let player = *strwindmap
            .get(matches.value_of("player wind").unwrap())
            .expect("Invalid player wind string specified");
        // 場風
        let round = *strwindmap
            .get(matches.value_of("round wind").unwrap())
            .expect("Invalid round wind string specified");
        // チー牌
        if let Some(c) = matches.value_of("chow") {
            let chow = Tile::from_mjscorestr(c).expect("Faild to parse chow");
            if chow.len() % 3 != 0 {
                panic!("invalid number of tiles in chow");
            }
            if chow.iter().any(|c| !c.id.is_suhai()) {
                panic!("chow tiles must include suhai only");
            }
            for i in (0..chow.len()).step_by(3) {
                if (chow[i].id.nth(1) != chow[i + 1].id) || (chow[i].id.nth(2) != chow[i + 2].id) {
                    panic!("invalid order of tiles in chow");
                }
                melds.push(Meld::Chow {
                    tiles: [chow[i], chow[i + 1], chow[i + 2]],
                });
            }
        }
        // ポン牌
        if let Some(p) = matches.value_of("pung") {
            let pung = Tile::from_mjscorestr(p).expect("Faild to parse pung");
            if pung.len() % 3 != 0 {
                panic!("invalid number of tiles in pung");
            }
            for i in (0..pung.len()).step_by(3) {
                if (pung[i].id != pung[i + 1].id) || (pung[i].id != pung[i + 2].id) {
                    panic!("pung tiles are must be same");
                }
                melds.push(Meld::Pung {
                    tiles: [pung[i], pung[i + 1], pung[i + 2]],
                });
            }
        }
        // 暗槓牌
        if let Some(ak) = matches.value_of("ankan") {
            let ankan = Tile::from_mjscorestr(ak).expect("Faild to parse ankan");
            if ankan.len() % 4 != 0 {
                panic!("invalid number of tiles in ankan");
            }
            for i in (0..ankan.len()).step_by(4) {
                if (ankan[i].id != ankan[i + 1].id)
                    || (ankan[i].id != ankan[i + 2].id)
                    || (ankan[i].id != ankan[i + 3].id)
                {
                    panic!("ankan tiles are must be same");
                }
                melds.push(Meld::Ankan {
                    tiles: [ankan[i], ankan[i + 1], ankan[i + 2], ankan[i + 4]],
                });
            }
        }
        // 明槓牌
        if let Some(mk) = matches.value_of("minkan") {
            let minkan = Tile::from_mjscorestr(mk).expect("Faild to parse minkan");
            if minkan.len() % 4 != 0 {
                panic!("invalid number of tiles in minkan");
            }
            for i in (0..minkan.len()).step_by(4) {
                if (minkan[i].id != minkan[i + 1].id)
                    || (minkan[i].id != minkan[i + 2].id)
                    || (minkan[i].id != minkan[i + 3].id)
                {
                    panic!("minkan tiles are must be same");
                }
                melds.push(Meld::Minkan {
                    tiles: [minkan[i], minkan[i + 1], minkan[i + 2], minkan[i + 4]],
                });
            }
        }
        // 表ドラ牌
        if let Some(d) = matches.value_of("omote dora") {
            let ts = Tile::from_mjscorestr(d).expect("Failed to parse omote dora");
            dora.omote.extend(&ts);
        }
        // 裏ドラ牌
        if let Some(d) = matches.value_of("ura dora") {
            let ts = Tile::from_mjscorestr(d).expect("Failed to parse ura dora");
            dora.ura.extend(&ts);
        }
        // 本場数
        let nhonba: i32 = if let Some(n) = matches.value_of("nhonba") {
            n.parse().expect("Failed to parse integer in nhonba")
        } else {
            0
        };
        // 供託リーチ棒本数
        let nriichi: i32 = if let Some(n) = matches.value_of("nriichi") {
            n.parse().expect("Failed to parse integer in nriichi")
        } else {
            0
        };
        // 得点計算
        let score = calculate_score(&AgariInformation {
            wining_tile,
            hand: Hand { hand, melds },
            nhonba,
            nriichi,
            round,
            player,
            tsumo: matches.is_present("tsumo"),
            riichi: matches.is_present("riichi"),
            ippatsu: matches.is_present("ippatsu"),
            doubleriichi: matches.is_present("doubleriichi"),
            haitei: matches.is_present("haitei"),
            rinshan: matches.is_present("rinshan"),
            chankan: matches.is_present("chankan"),
            nagashimangan: false,
            tenho: false,
            chiho: false,
            dora,
        })
        .expect("Failed to calculate socre");
        // 翻/符/取得点数
        println!(
            "{} han {} fu, {} point",
            score.han, score.fu, score.point.get
        );
        // 成立役リスト
        print!("yaku: ");
        for y in score.yaku {
            print!("{}, ", y.to_jpstr());
        }
        // 支払い情報
        println!();
        println!(
            "feed: {}",
            match score.point.feed {
                Feed::Duck { point } => format!("{}", point),
                Feed::Tsumo { ko, oya } => format!("oya:{} ko:{}", oya, ko),
            }
        );
    }
}
