use anyhow::Result;
use bms_rs::bms::{BmsOutput, default_config, parse_bms};
use bms_rs::bmson::{BmsonParseOutput, parse_bmson};
use chardetng::{EncodingDetector, Iso2022JpDetection, Utf8Detection};
use encoding_rs::{BIG5, EUC_KR, SHIFT_JIS};
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

use crate::cli::ImportEncoding;
use crate::model::{Chart, Song};
use crate::utils;

pub fn parse_chart(path: &Path, encoding: ImportEncoding) -> Result<Chart> {
    let bytes = fs::read(path)?;
    let sha256 = utils::sha256sum(&bytes);

    let (md5, genre, title, subtitle, artist, sub_artist, wavs, bgas) = if path.extension().unwrap()
        == "bmson"
    {
        let source = std::str::from_utf8(&bytes)?;

        let BmsonParseOutput { bmson, errors } = parse_bmson(source);
        let bmson = bmson.ok_or_else(|| anyhow::anyhow!("failed to parse bmson: {:?}", errors))?;

        let wavs: HashSet<_> = bmson
            .sound_channels
            .iter()
            .map(|channel| PathBuf::from(channel.name.as_ref()))
            .collect();

        let bgas: HashSet<_> = bmson
            .bga
            .bga_header
            .iter()
            .map(|header| PathBuf::from(header.name.as_ref()))
            .collect();

        let wavs = wavs.into_iter().collect();
        let bgas = bgas.into_iter().collect();

        (
            None,
            bmson.info.genre.to_string(),
            bmson.info.title.to_string(),
            bmson.info.subtitle.to_string(),
            bmson.info.artist.to_string(),
            bmson.info.subartists.join(" ").to_string(),
            wavs,
            bgas,
        )
    } else {
        let (source, _, _) = match encoding {
            ImportEncoding::ShiftJis => SHIFT_JIS.decode(&bytes),
            ImportEncoding::Big5 => BIG5.decode(&bytes),
            ImportEncoding::EucKr => EUC_KR.decode(&bytes),
            ImportEncoding::Auto => {
                let mut detector = EncodingDetector::new(Iso2022JpDetection::Deny);
                detector.feed(&bytes, true);

                let encoding = detector.guess(None, Utf8Detection::Deny);
                encoding.decode(&bytes)
            }
        };

        let BmsOutput { bms, warnings: _ } = parse_bms(source.as_ref(), default_config());
        let bms = bms?;

        let wavs: HashSet<_> = bms.wav.wav_files.values().cloned().collect();
        let bgas: HashSet<_> = bms
            .bmp
            .bmp_files
            .values()
            .map(|bmp| bmp.file.clone())
            .collect();

        let wavs = wavs.into_iter().collect();
        let bgas = bgas.into_iter().collect();

        (
            Some(utils::md5sum(&bytes)),
            bms.music_info.genre.unwrap_or_default(),
            bms.music_info.title.unwrap_or_default(),
            bms.music_info.subtitle.unwrap_or_default(),
            bms.music_info.artist.unwrap_or_default(),
            bms.music_info.sub_artist.unwrap_or_default(),
            wavs,
            bgas,
        )
    };

    let subtitle = (!subtitle.is_empty()).then_some(subtitle);
    let sub_artist = (!sub_artist.is_empty()).then_some(sub_artist);

    Ok(Chart {
        genre,
        title,
        subtitle,
        artist,
        sub_artist,

        wavs,
        bgas,

        filename: path.file_name().unwrap().into(),

        md5,
        sha256,
    })
}

pub fn build_song(charts: &[Chart]) -> Song {
    let (title, artist, mut wavs, mut bgas) = if charts.len() == 1 {
        let chart = &charts[0];

        (
            chart.title.clone(),
            chart.artist.clone(),
            chart.wavs.clone(),
            chart.bgas.clone(),
        )
    } else {
        let wavs: HashSet<_> = charts.iter().flat_map(|c| c.wavs.iter().cloned()).collect();
        let bgas: HashSet<_> = charts.iter().flat_map(|c| c.bgas.iter().cloned()).collect();

        (
            utils::common_prefix(charts.iter().map(|c| c.title.as_str())),
            utils::common_prefix(charts.iter().map(|c| c.artist.as_str())),
            wavs.into_iter().collect(),
            bgas.into_iter().collect(),
        )
    };

    wavs.sort();
    bgas.sort();

    Song {
        title,
        artist,
        wavs,
        bgas,
    }
}
