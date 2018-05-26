use resources::general::*;

pub type CastPerson = CastEmbed;

#[derive(Debug, Deserialize)]
pub struct CrewPerson {
    pub person: Person,
    #[serde(rename = "type")]
    pub _type: String,
}

#[derive(Debug, Deserialize)]
pub struct CastEmbed {
    pub person: Person,
    pub character: Character,
}

#[derive(Debug, Deserialize)]
pub struct Person {
    pub id: u64,
    pub url: String,
    pub name: String,
    pub country: Option<Country>,
    pub birthday: Option<String>,
    pub deathday: Option<String>,
    pub gender: Option<String>,
    pub image: Option<ImageLinks>,
}

#[derive(Debug, Deserialize)]
pub struct Character {
    pub id: u64,
    pub url: String,
    pub name: String,
    pub image: Option<ImageLinks>,
}
