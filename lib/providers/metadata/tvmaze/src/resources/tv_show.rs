use resources::general::*;

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
    pub runtime: Option<u64>,
    pub premiered: Option<String>,
    pub official_site: Option<String>,
    pub schedule: TVShowSchedule,
    pub rating: TVShowRating,
    pub weight: u64,
    pub network: Option<TVShowNetwork>,
    pub web_channel: Option<TVShowWebChannel>,
    pub externals: TVShowExternals,
    pub image: Option<ImageLinks>,
    pub summary: Option<String>,
    pub updated: u64,
    #[serde(rename = "_embedded")]
    pub embeds: Option<Embed>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TVShowSeason {
    pub id: u64,
    pub url: String,
    pub number: u64,
    pub name: String,
    pub episode_order: Option<u64>,
    pub premiere_date: String,
    pub end_date: String,
    pub network: Option<TVShowNetwork>,
    pub web_channel: Option<TVShowWebChannel>,
    pub image: Option<ImageLinks>,
    pub summary: Option<String>,
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
pub struct TVShowNetwork {
    pub id: u64,
    pub name: String,
    pub country: Country,
}

#[derive(Debug, Deserialize)]
pub struct TVShowWebChannel {
    pub id: u64,
    pub name: String,
    pub country: Option<Country>,
}

#[derive(Debug, Deserialize)]
pub struct TVShowExternals {
    pub tvrage: Option<u64>,
    pub thetvdb: Option<u64>,
    pub imdb: Option<String>,
}
