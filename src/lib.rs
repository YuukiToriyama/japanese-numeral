mod converter;

use regex::Regex;
use std::collections::HashMap;

/**
 * 漢数字を単位ごとに分割する
 */
pub fn split_large_number(japanese: String) -> HashMap<&'static str, i32> {
    let mut input = japanese;
    let mut parsed_num: HashMap<&str, i32> = HashMap::new();
    // 京、兆、億、万の順でそれぞれの桁についてパースしていく
    for unit in ["京", "兆", "億", "万"] {
        // 正規表現を使い、◯◯万のような文字列をキャプチャする
        let expression = "(?P<number>.+)".to_string() + unit;
        let reg = Regex::new(&expression).unwrap();
        let mut remove_part = String::new();
        let result = match reg.captures(&input) {
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

    // 一万未満の桁がある場合処理を続ける
    if input.as_str() != "" {
        parsed_num.insert("千", kanjinum_to_int(&input));
    } else {
        parsed_num.insert("千", 0);
    }

    parsed_num
}

fn kanjinum_to_int(input: &str) -> i32 {
    // 入力がすべてアラビア数字で構成されている場合
    if Regex::new(r"^[0-9]+$").unwrap().is_match(input) {
        let num: i32 = input.parse().unwrap();
        return num;
    }

    let mut number = 0;
    for (unit, base) in [("千", 1000), ("百", 100), ("十", 10)] {
        let expression = "(?P<number>.*)".to_string() + unit;
        let reg = Regex::new(&expression).unwrap();
        let capture = reg.captures(input);
        let n = match capture {
            Some(v) => {
                if Regex::new("^[0-9]+$").unwrap().is_match(&v["number"]) {
                    let num: i32 = v["number"].parse().unwrap();
                    return num;
                } else {
                    let num: i32 = converter::normalize_kanjinum_string(v["number"].to_string())
                        .parse()
                        .unwrap();
                    return num;
                }
            }
            None => 0,
        };
        number = number + n * base;
    }
    number
}

#[test]
fn test_kanjinum_to_int() {
    assert_eq!(kanjinum_to_int("三千"), 3000);
    assert_eq!(kanjinum_to_int("千九百一"), 1901);
    assert_eq!(kanjinum_to_int("九九"), 99);
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
let unit_large: HashMap<&str, i64> = vec![
    ("万", 1_0000),
    ("億", 1_0000_0000),
    ("兆", 1_0000_0000_0000),
    ("京", 1_0000_0000_0000_0000)
].into_iter().collect();
*/
