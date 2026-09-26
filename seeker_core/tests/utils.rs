use seeker_core::model::{Separator, Token};
use seeker_core::utils;

#[test]
fn tokenize_test() {
    let input = "foo bar-[7key_hard] baz";

    assert_eq!(
        utils::tokenize(input),
        vec![
            Token {
                value: "foo".to_string(),
                separator: Separator::Space
            },
            Token {
                value: "bar-".to_string(),
                separator: Separator::Split
            },
            Token {
                value: "[7key_hard]".to_string(),
                separator: Separator::Space
            },
            Token {
                value: "baz".to_string(),
                separator: Separator::None
            },
        ]
    );
}

#[test]
fn untokenize_test() {
    let input = "foo bar-[7key_hard] baz";
    let tokens = utils::tokenize(input);

    assert_eq!(utils::untokenize(&tokens), "foo bar-[7key_hard] baz");
}

#[test]
fn common_prefix_test() {
    let titles = [
        "ひつぎとふたご",
        "ひつぎとふたご [7KEY/NORMAL]",
        "ひつぎとふたご [HYPER]",
        "HITSUGILUNATIC",
    ];

    let titles2 = [
        "Witch of Dimension ～次元の魔女～ [Administration Bureau's White Devil]",
        "Witch of Dimension ～次元の魔女～ [made2]",
        "Witch of Dimension ～次元の魔女～ [made]",
        "Witch of Dimension ～次元の魔女～ [Storage]",
        "Witch of Dimension ～次元の魔女～ [Introduction7]",
        "Witch of Dimension ～次元の魔女～ [Intelligent]",
        "Witch of Dimension ～次元の魔女～ [Introduction5]",
        "Witch of Dimension ～次元の魔女～ [Armed]",
        "Witch of Dimension ～次元の魔女～ [Unison]",
    ];

    let titles3 = [
        "Xiper 2026[SP ANOTHER]",
        "Xiper 2026[SP BEGINNER]",
        "Xiper 2026[SP HYPER]",
        "Xiper 2026[SP INSANE]",
        "Xiper 2026[SP NORMAL]",
    ];

    let titles4 = [
        "クレイジー土下座.smd (編集用)",
        "クレイジー土下座.smdが\u{3000}夢\u{3000}の\u{3000}よ\u{3000}う\u{3000}ソ\u{3000}フ\u{3000}ラ\u{3000}ン E D I  T",
        "クレイジー土下座.smdが\u{3000}夢\u{3000}の\u{3000}よ\u{3000}う",
        "クレイジー土下座.smd (ふつう)",
        "クレイジー土下座.smd (スコアアタック用)",
        "クソクソクソクソクソクソイジー土下座.smd",
        "クレイジー土下座.smd (長い)",
        "クレイジー土下座.smd (肉)",
        "ロングクレイジー土下座.longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglongllonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglongoncmd\u{3000}夢\u{3000}の\u{3000}よ\u{3000}う",
        "ロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングクレイジー土下座.smdが\u{3000}夢\u{3000}の\u{3000}よ\u{3000}う",
        "ロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングロングクレイジー土下座.smdが\u{3000}夢\u{3000}の\u{3000}よ\u{3000}う",
        "クレイジー土下座.smd (くそ長い)",
        "夢\u{3000}の\u{3000}よ\u{3000}う な\u{3000}ク\u{3000}ソ\u{3000}土\u{3000}下\u{3000}座",
        "夢\u{3000}の\u{3000}よ\u{3000}う な\u{3000}ク\u{3000}ソ\u{3000}土\u{3000}下\u{3000}座\u{3000}ソ\u{3000}フ\u{3000}ラ\u{3000}ン\u{3000}ス\u{3000}ペ\u{3000}シ\u{3000}ャ\u{3000}ル",
    ];

    let artists = [
        "plastic feat.サキト",
        "plastic feat.サキト / obj:夢瑠",
        "plastic feat.サキト / obj.boxpurin",
        "plastic feat.sakito",
    ];

    let artists2 = [
        "-45 / obj.MENNY",
        "-45/わなな・BANI",
        "-45 / わなな・BANI /reobj:hamburger",
        "-45 / わなな・BANI",
        "-45/わなな・BANI",
        "-45/わなな・BANI",
        "-45/わなな・BANI",
        "-45/わなな・BANI",
        "-45/わなな・BANI",
        "-45/わなな・BANI / obj:スノート",
        "-45/わなな・BANI / obj:スノート",
        "-45/わなな・BANI / obj:スノート",
        "-45 / わなな・BANI / dj K'",
        "-45 / わなな・BANI / dj K'",
        "-45 / わなな・BANI / dj K'",
        "-45/わなな・BANI / obj:発汗BMS",
        "-45/わなな・BANI / obj:発汗BMS / 穴抜き",
        "-45/わなな・BANI / obj:発汗BMS",
        "-45 / わなな・BANI / 、",
        "-45/わなな・BANI",
        "-45 / わなな・BANI / obj.mp",
        "-45 / わなな・BANI / obj.mp",
        "-45 / わなな・BANI / obj.mp",
        "-45/わなな・BANI",
        "-45 / わなな・BANI / 、",
        "-45 / わなな・BANI / 、",
        "-45 / わなな・BANI",
        "-45 / わなな・BANI mixed RYO-TA",
    ];

    assert_eq!(utils::common_prefix(titles), "ひつぎとふたご");
    assert_eq!(
        utils::common_prefix(titles2),
        "Witch of Dimension ～次元の魔女～"
    );
    assert_eq!(utils::common_prefix(titles3), "Xiper 2026");
    assert_eq!(utils::common_prefix(titles4), "クレイジー土下座.smd");
    assert_eq!(utils::common_prefix(artists), "plastic feat.サキト");
    assert_eq!(utils::common_prefix(artists2), "-45/わなな・BANI");
}
