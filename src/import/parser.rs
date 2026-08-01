use anyhow::Result;
use bms_rs::bms::{BmsOutput, default_config, parse_bms};
use encoding_rs::SHIFT_JIS;
use std::collections::{HashMap, HashSet};
use std::{fs, path::Path};

use crate::model::{Chart, Song};
use crate::utils;

pub fn parse_chart(path: &Path) -> Result<Chart> {
    let bytes = fs::read(path)?;

    let md5 = utils::md5sum(&bytes);
    let sha256 = utils::sha256sum(&bytes);

    let (source, _, _) = SHIFT_JIS.decode(&bytes);

    let BmsOutput { bms, warnings } = parse_bms(source.as_ref(), default_config());
    let bms = bms.expect("must be parsed");

    let wavs: HashSet<_> = bms.wav.wav_files.values().cloned().collect();
    let bgas: HashSet<_> = bms
        .bmp
        .bmp_files
        .values()
        .map(|bmp| bmp.file.clone())
        .collect();

    let wavs = wavs.into_iter().collect();
    let bgas = bgas.into_iter().collect();

    Ok(Chart {
        id: None,
        song_id: None,

        genre: bms.music_info.genre.unwrap_or_default(),
        title: bms.music_info.title.unwrap_or_default(),
        subtitle: bms.music_info.subtitle,
        artist: bms.music_info.artist.unwrap_or_default(),
        sub_artist: bms.music_info.sub_artist,

        wavs,
        bgas,

        filename: path.file_name().unwrap().into(),

        md5,
        sha256,
    })
}

pub fn build_song(charts: &[Chart]) -> Song {
    let wavs: HashSet<_> = charts.iter().flat_map(|c| c.wavs.iter().cloned()).collect();
    let bgas: HashSet<_> = charts.iter().flat_map(|c| c.bgas.iter().cloned()).collect();
    let mut wavs_sort: Vec<_> = wavs.into_iter().collect();
    let mut bgas_sort: Vec<_> = bgas.into_iter().collect();
    wavs_sort.sort();
    bgas_sort.sort();

    Song {
        id: None,
        title: utils::common_prefix(charts.iter().map(|c| c.title.as_str())),
        artist: utils::common_prefix(charts.iter().map(|c| c.artist.as_str())),

        wavs: wavs_sort,
        bgas: bgas_sort,
    }
}
