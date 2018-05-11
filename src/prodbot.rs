use failure::Error;
use failure::ResultExt;
use reqwest;
use rss;
use serde_json;
use std::collections::HashMap;
use std::io::BufReader;

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
    #[serde(rename = "addedUser")] pub added_user: String,
    #[serde(rename = "addedDate")] pub added_date: String,
    pub acronym: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub nickname: String,
    pub level: String,
    #[serde(rename = "permissionSubmitItems")] pub permission_submit_items: Option<String>,
    #[serde(rename = "permissionPostBBS")] pub permission_post_bbs: Option<String>,
    pub avatar: String,
    pub glops: String,
    #[serde(rename = "registerDate")] pub register_date: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DownloadLink {
    #[serde(rename = "type")] pub _type: String,
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
    #[serde(rename = "type")] pub _type: String,
    #[serde(rename = "addedUser")] pub added_user: String,
    #[serde(rename = "addedDate")] pub added_date: String,
    #[serde(rename = "releaseDate")] pub release_date: String,
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
    pub demozoo: Option<String>,
    pub csdb: String,
    pub zxdemo: String,
    pub invitation: Option<String>,
    pub invitationyear: String,
    #[serde(rename = "boardID")] pub board_id: Option<String>,
    pub rank: String,
    pub cdc: i64,
    #[serde(rename = "downloadLinks")] pub download_links: Vec<DownloadLink>,
    pub screenshot: String,
    pub party_compo_name: String,
    pub credits: Vec<Credit>,
}

impl Prod {
    pub fn vote_count(&self) -> usize {
        self.voteup.parse::<usize>().unwrap() + self.votepig.parse::<usize>().unwrap()
            + self.votedown.parse::<usize>().unwrap()
    }

    pub fn vote_string(&self) -> String {
        format!(
            "[voteup: {}, votepig: {}, votedown: {}, cdc: {}]",
            self.voteup, self.votepig, self.votedown, self.cdc
        )
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProdResponse {
    pub success: bool,
    pub prod: Prod,
}

pub struct PouetAPIClient {}

impl PouetAPIClient {
    pub fn new() -> Self {
        PouetAPIClient {}
    }

    pub fn get_prod(&self, id: usize) -> Result<ProdResponse, Error> {
        let result = reqwest::get(&format!("http://api.pouet.net/v1/prod/?id={}", id))?;
        Ok(serde_json::from_reader(result)
            .context(format!("Couldn't deserialize response for prod {}", id))?)
    }

    pub fn get_comments(&self, id: usize) -> Result<rss::Channel, Error> {
        let response = reqwest::get(&format!(
            "https://www.pouet.net/export/lastprodcomments.rss.php?prod={}",
            id
        ))?;
        Ok(rss::Channel::read_from(BufReader::new(response))
            .or_else(|error| Err(format_err!("{}", error)))?)
    }
}
