use failure::Error;
use reqwest;
use serde_json;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Platform {
    pub name: String,
    pub icon: String,
    pub slug: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Party {
    pub id: String,
    pub name: String,
    pub web: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Placing {
    pub party: Party,
    pub compo: String,
    pub ranking: String,
    pub year: String,
    pub compo_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Group {
    pub id: String,
    pub name: String,
    pub web: String,
    #[allow(non_snake_case)]
    pub addedUser: String,
    #[allow(non_snake_case)]
    pub addedDate: String,
    pub acronym: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub nickname: String,
    pub level: String,
    #[allow(non_snake_case)]
    pub permissionSubmitItems: String,
    #[allow(non_snake_case)]
    pub permissionPostBBS: String,
    pub avatar: String,
    pub glops: String,
    #[allow(non_snake_case)]
    pub registerDate: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DownloadLink {
    #[serde(rename = "type")]
    pub _type: String,
    pub link: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Credit {
    pub user: User,
    pub role: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Prod {
    pub types: Vec<String>,
    pub platforms: HashMap<String, Platform>,
    pub placings: Vec<Placing>,
    pub groups: Vec<Group>,
    pub awards: Vec<serde_json::Value>,
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub _type: String,
    #[allow(non_snake_case)]
    pub addedUser: String,
    #[allow(non_snake_case)]
    pub addedDate: String,
    #[allow(non_snake_case)]
    pub releaseDate: String,
    pub voteup: String,
    pub votepig: String,
    pub votedown: String,
    pub voteavg: String,
    pub download: String,
    pub party_compo: String,
    pub party_place: String,
    pub party_year: String,
    pub party: Party,
    pub addeduser: User,
    pub sceneorg: String,
    pub demozoo: String,
    pub csdb: String,
    pub zxdemo: String,
    pub invitation: String,
    pub invitationyear: String,
    #[allow(non_snake_case)]
    pub boardID: Option<String>,
    pub rank: String,
    pub cdc: i64,
    #[allow(non_snake_case)]
    pub downloadLinks: Vec<DownloadLink>,
    pub screenshot: String,
    pub party_compo_name: String,
    pub credits: Vec<Credit>,
}

impl Prod {
    pub fn vote_string(&self) -> String {
        format!("[voteup: {}, votepig: {}, votedown: {}, cdc: {}]", self.voteup, self.votepig, self.votedown, self.cdc)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProdResponse {
    pub success: bool,
    pub prod: Prod,
}

pub struct PouetAPIClient {
    client: reqwest::Client,
}

impl PouetAPIClient {
    pub fn new() -> Self {
        PouetAPIClient{
            client: reqwest::Client::new()
        }
    }

    pub fn get_prod(&self, id: usize) -> Result<ProdResponse, Error> {
        Ok(serde_json::from_reader(
            reqwest::get(
                &format!("http://api.pouet.net/v1/prod/?id={}", id))?)?)
    }
}
