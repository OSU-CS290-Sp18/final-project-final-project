use resources::general::*;

#[derive(Debug, Deserialize)]
pub struct Episode {
    pub id: u64,
    pub url: String,
    pub name: String,
    pub season: u64,
    pub number: Option<u64>,
    pub airdate: String,
    pub airtime: String,
    pub airstamp: Option<String>,
    pub runtime: Option<u64>,
    pub image: Option<ImageLinks>,
    pub summary: Option<String>,
    #[serde(rename = "_embedded")]
    pub embeds: Option<Embed>,
}
