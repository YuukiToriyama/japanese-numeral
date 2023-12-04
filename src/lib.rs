use regex::Regex;
use std::collections::HashMap;

pub mod converter {
    use std::iter::FromIterator;
    fn convert_oldish_char(c: char) -> Option<char> {
        match c {
            '零' => Some('〇'),
            '壱' => Some('一'),
            '壹' => Some('一'),
            '弐' => Some('二'),
            '貳' => Some('二'),
            '貮' => Some('二'),
            '参' => Some('三'),
            '參' => Some('三'),
            '肆' => Some('四'),
            '伍' => Some('五'),
            '陸' => Some('六'),
            '漆' => Some('七'),
            '捌' => Some('八'),
            '玖' => Some('九'),
            '拾' => Some('十'),
            '陌' => Some('百'),
            '佰' => Some('百'),
            '阡' => Some('千'),
            '仟' => Some('千'),
            '萬' => Some('万'),
            _ => None,
        }
    }

    pub fn normalize_oldish_string(input: String) -> String {
        let mut vec_dst: Vec<char> = vec![];
        for c in input.chars().collect::<Vec<_>>() {
            match convert_oldish_char(c) {
                Some(v) => vec_dst.push(v),
                None => match c {
                    '廿' => {
                        vec_dst.push('二');
                        vec_dst.push('十');
                    }
                    _ => vec_dst.push(c),
                },
            }
        }
        String::from_iter(vec_dst)
    }

    pub fn normalize_kanjinum_string(input: String) -> String {
        let mut vec_dst: Vec<char> = vec![];
        for c in input.chars().collect::<Vec<_>>() {
            let filtered = match c {
                '〇' => '0',
                '一' => '1',
                '二' => '2',
                '三' => '3',
                '四' => '4',
                '五' => '5',
                '六' => '6',
                '七' => '7',
                '八' => '8',
                '九' => '9',
                _ => c,
            };
            vec_dst.push(filtered);
        }
        String::from_iter(vec_dst)
    }

    pub fn normalize_zenkakunum_string(input: String) -> String {
        let mut vec_dst: Vec<char> = vec![];
        for c in input.chars().collect::<Vec<_>>() {
            let filtered = match c {
                '０' => '0',
                '１' => '1',
                '２' => '2',
                '３' => '3',
                '４' => '4',
                '５' => '5',
                '６' => '6',
                '７' => '7',
                '８' => '8',
                '９' => '9',
                _ => c,
            };
            vec_dst.push(filtered);
        }
        String::from_iter(vec_dst)
    }

    #[test]
    fn test_normalize_oldish_string() {
        assert_eq!(
            normalize_oldish_string(String::from("伍伍カレー")),
            String::from("五五カレー")
        );
        assert_eq!(
            normalize_oldish_string(String::from("CoCo壱番屋廿日市駅前店")),
            String::from("CoCo一番屋二十日市駅前店")
        );
    }
    #[test]
    fn test_normalize_kanjinum_string() {
        assert_eq!(
            normalize_kanjinum_string(String::from("一九九九年一〇月")),
            String::from("1999年10月")
        );
    }
    #[test]
    fn test_normalize_zenkakunum_string() {
        assert_eq!(
            normalize_zenkakunum_string(String::from("２０２１年")),
            String::from("2021年")
        );
    }
}

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
