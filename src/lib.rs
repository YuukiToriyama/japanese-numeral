use regex::Regex;
use std::collections::HashMap;

//mod converter::{normalize_kanjinum_string, normalize_oldish_string, normalize_zenkakunum_string};

/**
 * 漢数字を単位ごとに分割する
 */
pub fn split_large_number(japanese: String) -> HashMap<&str, i32> {
    let mut input = japanese;
    let mut parsed_num: HashMap<&str, i32> = HashMap::new();
    // 京、兆、億、万の順でそれぞれの桁についてパースしていく
    for unit in vec!["京", "兆", "億", "万"] {
        // 正規表現を使い、◯◯万のような文字列をキャプチャする
        let expression = "(?P<number>.+)".to_string() + unit;
        let reg = Regex::new(&expression).unwrap();
        let capture = reg.captures(&input);
        let mut remove_part = String::new();
        let result = match capture {
            Some(v) => {
                remove_part = format!("{}{}", &v["number"], unit);
                kanjinum_to_int(&v["number"])
            }
            None => 0,
        };
        // キャプチャが存在した場合はもとの文字列からその部分を削除する
        if remove_part.as_str() != "" {
            input = input.replace(&remove_part, "");
        }
        // 抜き出したものはHashMapに保存
        parsed_num.insert(unit, result);
    }
    return parsed_num;
}

fn kanjinum_to_int(input: &str) -> i32 {
    return 123;
}
/*
const match = kanji.match(reg)
    if (match) {
      numbers[key] = kan2n(match[1])
      kanji = kanji.replace(match[0], '')
    } else {
      numbers[key] = 0
    }
*/

/*
let unit_small: Vec<(&str, i32)> = vec![
    ("十", 10),
    ("百", 100),
    ("千", 1000)
];
let unit_large: HashMap<&str, i64> = vec![
    ("万", 1_0000),
    ("億", 1_0000_0000),
    ("兆", 1_0000_0000_0000),
    ("京", 1_0000_0000_0000_0000)
].into_iter().collect();
*/
