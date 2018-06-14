use resources::{CastEmbed, Episode};

#[derive(Debug, Deserialize)]
pub struct AKA {
    pub name: String,
    pub country: Option<Country>,
}

#[derive(Debug, Deserialize)]
pub struct Country {
    pub name: String,
    pub code: String,
    pub timezone: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ImageLinks {
    pub medium: String,
    pub original: String,
}

#[derive(Debug, Deserialize)]
pub struct Embed {
    pub cast: Option<Vec<CastEmbed>>,
    pub episodes: Option<Vec<Episode>>,
}
