use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;

fn main() {
    // ビルドスクリプトに変更があれば実行
    println!("cargo:rerun-if-changed=build.rs");

    // 向聴数データを開く
    let infile = match File::open("./data/shanten.dat") {
        Err(e) => panic!("couldn't open shanten.dat: {}", e),
        Ok(f) => f,
    };

    let mut builder = phf_codegen::Map::new();
    for line in BufReader::new(infile).lines() {
        let line_str = line.unwrap();
        let entry: Vec<&str> = line_str.split(' ').collect();
        // 2通りの面子・塔子が入っているので向聴数が少ない方を選択
        let num_mentsu_a = entry[1].parse::<i32>().unwrap();
        let num_tatsu_a = entry[2].parse::<i32>().unwrap();
        let num_mentsu_b = entry[3].parse::<i32>().unwrap();
        let num_tatsu_b = entry[4].parse::<i32>().unwrap();
        let (num_mentsu, num_tatsu) =
            if (2 * num_mentsu_a + num_tatsu_a) >= (2 * num_mentsu_b + num_tatsu_b) {
                (num_mentsu_a, num_tatsu_a)
            } else {
                (num_mentsu_b, num_tatsu_b)
            };
        let value_str = format!("({}, {})", num_mentsu, num_tatsu);
        builder.entry(entry[0].to_owned(), &value_str);
    }

    // phfでハッシュテーブル出力
    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("shanten_hash_table.rs");
    let mut outfile = BufWriter::new(File::create(&path).unwrap());
    writeln!(
        &mut outfile,
        "static SHANTEN_HASH: phf::Map<&'static str, (i32, i32)> = \n{};\n",
        builder.build()
    )
    .unwrap();
}
