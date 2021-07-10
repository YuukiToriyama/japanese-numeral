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
    return String::from_iter(vec_dst);
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
    return String::from_iter(vec_dst);
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
    return String::from_iter(vec_dst);
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
