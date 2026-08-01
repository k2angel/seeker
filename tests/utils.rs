use seeker::utils;

#[test]
fn tokenize_test() {
    let input = "foo bar-[7key_hard] baz";

    assert_eq!(
        utils::tokenize(input),
        vec![
            ("foo".to_string(), true),
            ("bar-".to_string(), false),
            ("[7key_hard]".to_string(), true),
            ("baz".to_string(), false)
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

    let artists = [
        "plastic feat.サキト",
        "plastic feat.サキト / obj:夢瑠",
        "plastic feat.サキト / obj.boxpurin",
        "plastic feat.sakito",
    ];

    assert_eq!(utils::common_prefix(titles), "ひつぎとふたご");
    assert_eq!(
        utils::common_prefix(titles2),
        "Witch of Dimension ～次元の魔女～"
    );
    assert_eq!(utils::common_prefix(artists), "plastic feat.サキト");
}
