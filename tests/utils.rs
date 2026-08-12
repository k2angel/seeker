use seeker::model::{Separator, Token};
use seeker::utils;

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
    assert_eq!(utils::common_prefix(artists), "plastic feat.サキト");
    assert_eq!(utils::common_prefix(artists2), "-45/わなな・BANI");
}
