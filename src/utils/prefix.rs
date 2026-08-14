use regex::Regex;
use std::collections::HashMap;
use std::sync::LazyLock;

use crate::model::{Separator, Token};

static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\[\S*\]|\(\S*\)|-[^-]+-|\/").unwrap());

static RE_PLAYSIDE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)[\[(-](sp|dp|\d+keys?|\d+k)$").unwrap());

pub fn tokenize(s: &str) -> Vec<Token> {
    let mut words = s.split_whitespace().peekable();
    let mut result = Vec::new();

    while let Some(word) = words.next() {
        let word = match RE_PLAYSIDE.find(word) {
            Some(m) => &word[..m.start()],
            None => word,
        };

        if word.is_empty() {
            continue;
        }

        let mut tokens_in_word = Vec::new();
        let mut pos = 0;

        for m in RE.find_iter(word) {
            if pos < m.start() {
                tokens_in_word.push(&word[pos..m.start()]);
            }

            tokens_in_word.push(m.as_str());
            pos = m.end();
        }

        if pos < word.len() {
            tokens_in_word.push(&word[pos..]);
        }

        let token_count = tokens_in_word.len();
        let is_last_word = words.peek().is_none();

        for (i, val) in tokens_in_word.into_iter().enumerate() {
            let is_last_value = i + 1 == token_count;

            let separator = match (is_last_value, is_last_word) {
                (true, false) => Separator::Space,
                (true, true) => Separator::None,
                (false, _) => Separator::Split,
            };

            result.push(Token {
                value: val.to_owned(),
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
        let mut counts: HashMap<&str, usize> = HashMap::with_capacity(tokens.len());
        for token in &tokens {
            *counts.entry(&token.value).or_default() += 1;
        }

        let (mut best_value, mut best_count) = counts
            .iter()
            .max_by_key(|&(_, count)| count)
            .map(|(&val, &count)| (val, count))
            .unwrap_or(("", 0));

        // 完全一致では半数未満なので、前方一致にフォールバック
        if best_count * 2 <= total {
            best_value = "";
            best_count = 0;

            for candidate in counts.keys() {
                let count = tokens
                    .iter()
                    .filter(|other| other.value.starts_with(candidate))
                    .count();

                if count > best_count {
                    best_value = candidate;
                    best_count = count;
                }
            }
        }

        if best_count * 2 < total && !result.is_empty() {
            break;
        }

        let mut separator = Separator::None;
        let mut has_none = false;
        let mut has_space = false;

        for token in tokens.iter().filter(|t| t.value == best_value) {
            match token.separator {
                Separator::Split => separator = Separator::Split,
                Separator::Space => {
                    if separator != Separator::Split {
                        separator = Separator::Space;
                    }
                    has_space = true;
                }
                Separator::None => {
                    has_none = true;
                }
            }
        }

        // value が一致したTokenは、まず結果に追加する
        result.push(Token {
            value: best_value.to_string(),
            separator,
        });

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
