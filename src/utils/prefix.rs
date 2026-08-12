use regex::Regex;

use crate::model::{Separator, Token};

pub fn tokenize(s: &str) -> Vec<Token> {
    let re = Regex::new(r"\[\S*\]|\(\S*\)|-[^-]+-|\/").unwrap();
    let re_playside = Regex::new(r"[\[(-](sp|dp|\d+keys?)$").unwrap();

    let words_list: Vec<Vec<&str>> = s
        .split_whitespace()
        .map(|word| {
            let word = if let Some(m) = re_playside.find(&word.to_lowercase()) {
                &word[..m.start()]
            } else {
                word
            };

            let mut result = Vec::new();
            let mut pos = 0;

            for m in re.find_iter(word) {
                if pos < m.start() {
                    result.push(&word[pos..m.start()]);
                }

                result.push(m.as_str());
                pos = m.end();
            }

            if pos < word.len() {
                result.push(&word[pos..]);
            }

            result
        })
        .collect();

    let words_count = words_list.len();
    let mut result = Vec::new();

    for (words_index, values) in words_list.into_iter().enumerate() {
        let is_last_words = words_index + 1 == words_count;
        let value_count = values.len();

        for (value_index, value) in values.into_iter().enumerate() {
            let is_last_value = value_index + 1 == value_count;

            let separator = if is_last_value && !is_last_words {
                Separator::Space
            } else if is_last_value && is_last_words {
                Separator::None
            } else {
                Separator::Split
            };

            result.push(Token {
                value: value.to_owned(),
                separator,
            });
        }
    }

    result
}

pub fn untokenize(tokens: &[Token]) -> String {
    let mut result = String::new();

    for token in tokens {
        result.push_str(&token.value);

        if token.separator == Separator::Space {
            result.push(' ');
        }
    }

    result
}

pub fn common_prefix<'a, I>(strings: I) -> String
where
    I: IntoIterator<Item = &'a str>,
{
    let tokenized: Vec<Vec<Token>> = strings.into_iter().map(tokenize).collect();

    if tokenized.is_empty() {
        return String::new();
    }

    let total = tokenized.len();
    let mut result = Vec::new();
    let mut index = 0;

    loop {
        // 現在位置にTokenが存在する系列だけを対象にする
        let tokens: Vec<&Token> = tokenized
            .iter()
            .filter_map(|tokens| tokens.get(index))
            .collect();

        if tokens.is_empty() {
            break;
        }

        // valueが最も多いTokenを選ぶ
        let mut best_token: Option<&Token> = None;
        let mut best_count = 0;

        for token in &tokens {
            let count = tokens
                .iter()
                .filter(|other| other.value == token.value)
                .count();

            if count > best_count {
                best_token = Some(*token);
                best_count = count;
            }
        }

        if best_count * 2 <= total {
            best_token = None;
            best_count = 0;

            // 完全一致では半数未満なので、前方一致にフォールバック
            for token in &tokens {
                let count = tokens
                    .iter()
                    .filter(|other| other.value.starts_with(&token.value))
                    .count();

                if count > best_count {
                    best_token = Some(*token);
                    best_count = count;
                }
            }
        }

        if best_count * 2 < total {
            break;
        }

        let best_token = best_token.unwrap();

        let matching_tokens: Vec<&Token> = tokens
            .into_iter()
            .filter(|token| token.value == best_token.value)
            .collect();

        // [Space, Split] => Split
        // [Space, Space] => Space
        // [None, None] => None
        let separator = if matching_tokens
            .iter()
            .any(|token| token.separator == Separator::Split)
        {
            Separator::Split
        } else if matching_tokens
            .iter()
            .any(|token| token.separator == Separator::Space)
        {
            Separator::Space
        } else {
            Separator::None
        };

        // value が一致したTokenは、まず結果に追加する
        result.push(Token {
            value: best_token.value.clone(),
            separator,
        });

        // None / Space が混在していたら、ここを最後にして終了
        let has_none = matching_tokens
            .iter()
            .any(|token| token.separator == Separator::None);

        let has_space = matching_tokens
            .iter()
            .any(|token| token.separator == Separator::Space);

        if has_none && has_space {
            break;
        }

        index += 1;
    }

    // 最後のTokenのseparatorは必ずNone
    if let Some(token) = result.last_mut() {
        token.separator = Separator::None;
    }

    untokenize(&result)
}
