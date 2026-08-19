use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum Class {
    #[serde(rename = "bms.player.beatoraja.TableData$TableFolder")]
    TableFolder,

    #[serde(rename = "bms.player.beatoraja.song.SongData")]
    SongData,

    #[serde(rename = "bms.player.beatoraja.CourseData")]
    CourseData,

    #[serde(rename = "bms.player.beatoraja.CourseData$TrophyData")]
    TrophyData,
}

#[derive(Debug, Serialize)]
pub enum Constraint {
    #[serde(rename = "CLASS")]
    Class,
    #[serde(rename = "MIRROR")]
    Mirror,
    #[serde(rename = "RANDOM")]
    Random,
    #[serde(rename = "NO_SPEED")]
    NoSpeed,
    #[serde(rename = "NO_GOOD")]
    NoGood,
    #[serde(rename = "NO_GREAT")]
    NoGreat,
    #[serde(rename = "GAUGE_LR2")]
    GaugeLr2,
    #[serde(rename = "GAUGE_5KEYS")]
    Gauge5Keys,
    #[serde(rename = "GAUGE_7KEYS")]
    Gauge7Keys,
    #[serde(rename = "GAUGE_9KEYS")]
    Gauge9Keys,
    #[serde(rename = "GAUGE_24KEYS")]
    Gauge24Keys,
    #[serde(rename = "LN")]
    Ln,
    #[serde(rename = "CN")]
    Cn,
    #[serde(rename = "HCN")]
    Hcn,
}

#[derive(Debug, Serialize)]
pub struct Bmt {
    pub url: String,
    pub name: String,
    pub tag: String,
    pub folder: Vec<Folder>,
    pub course: Option<Vec<Course>>,
}

#[derive(Debug, Serialize)]
pub struct Folder {
    pub class: Class,
    pub name: String,
    pub songs: Vec<Song>,
}

#[derive(Debug, Serialize)]
pub struct Song {
    pub class: Class,
    pub title: String,
    pub artist: String,
    pub md5: Option<String>,
    pub sha256: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct Course {
    pub class: Class,
    pub name: String,
    pub hash: Vec<Hash>,
    pub constraint: Vec<Constraint>,
    pub trophy: Vec<Trophy>,
}

#[derive(Debug, Serialize)]
pub struct Hash {
    pub title: String,
    pub md5: String,
}

#[derive(Debug, Serialize)]
pub struct Trophy {
    pub class: Class,
    pub name: String,
    pub missrate: f32,
    pub scorerate: f32,
}

impl TryFrom<&str> for Constraint {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "grade" => Ok(Self::Class),
            "grade_mirror" => Ok(Self::Mirror),
            "grade_random" => Ok(Self::Random),
            "no_speed" => Ok(Self::NoSpeed),
            "no_good" => Ok(Self::NoGood),
            "no_great" => Ok(Self::NoGreat),
            "gauge_lr2" => Ok(Self::GaugeLr2),
            "gauge_5k" => Ok(Self::Gauge5Keys),
            "gauge_7k" => Ok(Self::Gauge7Keys),
            "gauge_9k" => Ok(Self::Gauge9Keys),
            "gauge_24k" => Ok(Self::Gauge24Keys),
            "ln" => Ok(Self::Ln),
            "cn" => Ok(Self::Cn),
            "hcn" => Ok(Self::Hcn),
            _ => Err(()),
        }
    }
}
