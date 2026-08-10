use regex::Regex;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::model::Song;

type Token = (String, bool);

pub fn md5sum(data: &[u8]) -> String {
    format!("{:x}", md5::compute(data))
}

pub fn sha256sum(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);

    hex::encode(hasher.finalize())
}

pub fn tokenize(s: &str) -> Vec<Token> {
    let re = Regex::new(r"\[\S*\]|\(\S*\)|-[^-]+-").unwrap();
    let re_playside = Regex::new(r"[\[(-](SP|sp|DP|dp)$").unwrap();

    let words: Vec<Vec<&str>> = s
        .split_whitespace()
        .map(|word| {
            let word = if let Some(m) = re_playside.find(word) {
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

    let word_count = words.len();
    let mut result = Vec::new();

    for (word_index, tokens) in words.into_iter().enumerate() {
        let is_last_word = word_index + 1 == word_count;
        let token_count = tokens.len();

        for (token_index, token) in tokens.into_iter().enumerate() {
            let is_last_token = token_index + 1 == token_count;

            result.push((token.to_owned(), is_last_token && !is_last_word));
        }
    }

    result
}

pub fn untokenize(tokens: &[Token]) -> String {
    let mut result = String::new();

    for (token, space) in tokens {
        result.push_str(token);

        if *space {
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

    let mut result = Vec::new();
    let mut candidates: Vec<&Vec<Token>> = tokenized.iter().collect();
    let mut index = 0;

    loop {
        let current: Vec<&Token> = candidates.iter().filter_map(|seq| seq.get(index)).collect();

        if current.is_empty() {
            break;
        }

        let mut counts = HashMap::<&str, usize>::new();

        for (token, _) in &current {
            *counts.entry(token.as_str()).or_default() += 1;
        }

        let Some((best, best_count)) = counts.into_iter().max_by_key(|(_, count)| *count) else {
            break;
        };

        let total = current.len();

        // 現在の token が過半数未満しか一致していないなら、
        // その token は結果に含めず終了する。
        if best_count * 2 <= total {
            break;
        }

        let matching: Vec<&Vec<Token>> = candidates
            .into_iter()
            .filter(|seq| seq.get(index).is_some_and(|(token, _)| token == best))
            .collect();

        // 結果に追加
        result.push(matching[0][index].clone());

        // true / false が混在していたらここで終了
        let has_true = matching.iter().any(|seq| seq[index].1);
        let has_false = matching.iter().any(|seq| !seq[index].1);

        if has_true && has_false {
            break;
        }

        candidates = matching;
        index += 1;
    }

    if let Some((_, space)) = result.last_mut() {
        *space = false;
    }

    untokenize(&result)
}

fn sanitize_filename(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            '<' | '>' | ':' | '"' | '/' | '\\' | '|' | '?' | '*' => '_',
            c if c.is_control() => '_',
            _ => c,
        })
        .collect::<String>()
        .trim()
        .trim_end_matches('.')
        .to_string()
}

impl Song {
    pub fn directory_name(&self) -> String {
        format!(
            "[{}] {}",
            sanitize_filename(&self.artist),
            sanitize_filename(&self.title)
        )
    }

    pub fn library_dir(&self, library: &Path) -> PathBuf {
        library.join(self.directory_name())
    }
}
