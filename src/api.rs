use resources::TVShow;

pub const TVMAZE_BASE_URL: &'static str = "http://api.tvmaze.com";

#[derive(Debug, Deserialize)]
pub struct ShowSearchResult {
    score: f64,
    show: TVShow,
}

#[derive(Debug)]
pub enum LinkEmbed<'a> {
    Single(&'a str),
    Multiple(Vec<&'a str>),
}
