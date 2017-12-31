#![feature(nll)]
#![feature(slice_patterns)]
#![feature(conservative_impl_trait)]

#[macro_use] extern crate serde_json;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate structopt_derive;
extern crate failure;
extern crate kuchiki;
extern crate reqwest;
extern crate serde;
extern crate structopt;

use failure::Error;
use std::fs::File;
use std::fs::create_dir;
use std::path::Path;
use std::thread;
use std::time;
use structopt::StructOpt;

mod pouet_client;

#[derive(StructOpt, Debug)]
#[structopt(name = "prodbot", about = "Scraper for pouet.net")]
struct Opt {
    #[structopt(long = "clear_cache", help = "Clear cache directory")]
    clear_cache: bool,

    #[structopt(long = "slack_webhook_url", help = "Target slack webhook url. Omitting will only print to console instead")]
    slack_webhook_url: Option<String>,

    #[structopt(long = "pouet_prod_ids", help = "Which pouet prod ids to listen to")]
    pouet_prod_ids: Vec<usize>,
}

fn check_prods(options: &Opt) -> Result<(), Error> {
    let reqwest_client = reqwest::Client::new();
    let pouet_api_client = pouet_client::PouetAPIClient::new();

    for prod_id in &options.pouet_prod_ids {
        if let Ok(prod_response) = pouet_api_client.get_prod(*prod_id) {
            let cache_key = &format!("cache/{}.json", prod_id);

            let mut cached_prod_response: Option<pouet_client::ProdResponse> = None;
            if let Ok(file) = File::open(cache_key) {
                let shadowed_cached_prod_response: pouet_client::ProdResponse = serde_json::from_reader(file)?;
                if shadowed_cached_prod_response.prod.vote_string() == prod_response.prod.vote_string() {
                    println!("Prod {} has no difference between pouet and cache. Skipping webhook delivery", prod_response.prod.name);
                    continue;
                }

                cached_prod_response = Some(shadowed_cached_prod_response);
            }

            let postfix = cached_prod_response
                .map_or("[no cached value]".to_string(), |response| response.prod.vote_string());
            let slack_text = format!("Prod <(https://www.pouet.net/prod.php?which={}|{}> now has {:#?} versus {}",
                                     prod_id,
                                     prod_response.prod.name,
                                     prod_response.prod.vote_string(),
                                     postfix);

            println!("{}", slack_text);
            if let Some(ref slack_webhook_url) = options.slack_webhook_url {
                reqwest_client.post(slack_webhook_url)
                    .json(&json!({
                            "text": slack_text,
                        }))
                    .send()?;
                println!("Delivered slack webhook");
            }

            serde_json::to_writer(File::create(cache_key)?, &prod_response)?;
        }
    }

    Ok(())
}


fn run() -> Result<(), Error> {
    let options: Opt = Opt::from_args();

    if options.clear_cache {
        if Path::new("cache").exists() {
            std::fs::remove_dir_all("cache")?;
        }
    }

    if !Path::new("cache").exists() {
        create_dir("cache")?;
    }

    let sleep_duration = time::Duration::from_secs(60);
    loop {
        println!("Checking prods {:?}", options.pouet_prod_ids);
        if let Err(error) = check_prods(&options) {
            println!("Encountered error checking prods: {:?}", error);
        }
        println!("Sleeping for {:?}", sleep_duration);
        thread::sleep(sleep_duration);
    }
}

fn main() {
    if let Err(error) = run() {
        for cause in error.causes() {
            println!("{:?}", cause);
        }
    }
}
