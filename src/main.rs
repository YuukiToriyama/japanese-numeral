use std::collections::HashMap;

fn normalize_old_style(input: &str) -> String {
    let old_japanese_numerics: HashMap<String, String> = vec![
        ("零".to_owned(), "〇".to_owned()),
        ("壱".to_owned(), "一".to_owned()),
        ("壹".to_owned(), "一".to_owned()),
        ("弐".to_owned(), "二".to_owned()),
        ("貳".to_owned(), "二".to_owned()),
        ("貮".to_owned(), "二".to_owned()),
        ("参".to_owned(), "三".to_owned()),
        ("參".to_owned(), "三".to_owned()),
        ("肆".to_owned(), "四".to_owned()),
        ("伍".to_owned(), "五".to_owned()),
        ("陸".to_owned(), "六".to_owned()),
        ("漆".to_owned(), "七".to_owned()),
        ("捌".to_owned(), "八".to_owned()),
        ("玖".to_owned(), "九".to_owned()),
        ("拾".to_owned(), "十".to_owned()),
        ("廿".to_owned(), "二十".to_owned()),
        ("陌".to_owned(), "百".to_owned()),
        ("佰".to_owned(), "百".to_owned()),
        ("阡".to_owned(), "千".to_owned()),
        ("仟".to_owned(), "千".to_owned()),
        ("萬".to_owned(), "万".to_owned()),
    ]
    .into_iter()
    .collect::<HashMap<String, String>>();

    let mut vec_dst: Vec<String> = vec![];
    for c in input.chars().collect::<Vec<_>>() {
        match old_japanese_numerics.get(c) {
            Some(value) => vec_dst.push(value.to_string()),
            None => vec_dst.push(c),
        }
    }
    return "hoge".to_string();
}

fn main() {
    println!("Hello, world!");
    let japanese_numerics = vec![
        ("〇".to_owned(), "0".to_owned()),
        ("一".to_owned(), "1".to_owned()),
        ("二".to_owned(), "2".to_owned()),
        ("三".to_owned(), "3".to_owned()),
        ("四".to_owned(), "4".to_owned()),
        ("五".to_owned(), "5".to_owned()),
        ("六".to_owned(), "6".to_owned()),
        ("七".to_owned(), "7".to_owned()),
        ("八".to_owned(), "8".to_owned()),
        ("九".to_owned(), "9".to_owned()),
        ("０".to_owned(), "0".to_owned()),
        ("１".to_owned(), "1".to_owned()),
        ("２".to_owned(), "2".to_owned()),
        ("３".to_owned(), "3".to_owned()),
        ("４".to_owned(), "4".to_owned()),
        ("５".to_owned(), "5".to_owned()),
        ("６".to_owned(), "6".to_owned()),
        ("７".to_owned(), "7".to_owned()),
        ("８".to_owned(), "8".to_owned()),
        ("９".to_owned(), "9".to_owned()),
    ]
    .into_iter()
    .collect::<HashMap<_, _>>();
}
