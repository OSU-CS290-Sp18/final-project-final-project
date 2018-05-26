#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TVShow {
    pub id: u64,
    pub url: String,
    pub name: String,
    #[serde(rename = "type")]
    pub _type: String,
    pub language: String,
    pub genres: Vec<String>,
    pub status: String,
    pub runtime: u64,
    pub premiered: String,
    pub official_site: Option<String>,
    pub schedule: TVShowSchedule,
    pub rating: TVShowRating,
    pub weight: u64,
    pub network: Option<TVShowNetwork>,
    pub web_channel: Option<TVShowWebChannel>,
    pub externals: TVShowExternals,
    pub image: Option<TVShowImage>,
    pub summary: String,
    pub updated: u64,
}

#[derive(Debug, Deserialize)]
pub struct TVShowSchedule {
    pub time: String,
    pub days: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct TVShowRating {
    pub average: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub struct TVShowCountry {
    pub name: String,
    pub code: String,
    pub timezone: String,
}

#[derive(Debug, Deserialize)]
pub struct TVShowNetwork {
    pub id: u64,
    pub name: String,
    pub country: TVShowCountry,
}

#[derive(Debug, Deserialize)]
pub struct TVShowWebChannel {
    pub id: u64,
    pub name: String,
    pub country: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TVShowExternals {
    pub tvrage: Option<u64>,
    pub thetvdb: Option<u64>,
    pub imdb: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TVShowImage {
    pub medium: String,
    pub original: String,
}
