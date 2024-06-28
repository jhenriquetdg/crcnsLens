use crate::net::get_url_html::get_url_html;
use crate::types::dataset::Dataset;

use bincode::{Decode, Encode};
use chrono::Utc;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use url as urllib;

#[derive(Debug, Clone, Encode, Decode)]
pub struct Collection {
    pub url: String,
    pub html: String,
    pub alias: String,
    pub descriptor: String,
    pub last_modified: String,
    pub datasets: Arc<Mutex<Vec<Dataset>>>,
}

impl PartialEq for Collection {
    fn eq(&self, other: &Collection) -> bool {
        self.alias == other.alias
    }
}
impl Eq for Collection {}

impl Default for Collection {
    fn default() -> Self {
        Collection {
            url: String::from_str("").unwrap(),
            html: String::from_str("<html></html>").unwrap(),
            alias: String::from_str("Default").unwrap(),
            descriptor: String::from_str("Default collection").unwrap(),
            last_modified: Utc::now().to_string(),
            datasets: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

impl Collection {
    pub async fn from_url(url: urllib::Url) -> Collection {
        let url_html = get_url_html(url.clone()).await;
        let url_package = sxd_html::parse_html(url_html.as_str());
        let url_document = url_package.as_document();

        let alias = url.clone();
        let alias: Vec<&str> = alias.path_segments().unwrap().skip(1).take(1).collect();
        let alias = alias.concat();

        let descriptor: String =
            sxd_xpath::evaluate_xpath(&url_document, "//h1[@id='parent-fieldname-title']/text()")
                .expect("Unable to find descriptor h1")
                .string()
                .trim()
                .into();

        let last_modified: String =
            sxd_xpath::evaluate_xpath(&url_document, "//span[@class='documentModified']//text()")
                .expect("Unable to find last modified span")
                .string()
                .trim()
                .into();

        let c = Collection {
            url: url.to_string(),
            html: url_html,
            alias,
            descriptor,
            last_modified,
            datasets: Arc::new(Mutex::new(Vec::new())),
        };

        println!("{:?}", c);
        c
    }

    pub fn from_filepath(fp: PathBuf) -> Self {
        let collection_fp = fp.join("ds.bin");
        let collection_fc_result = std::fs::read(collection_fp.clone());
        match collection_fc_result {
            Ok(collection_fc) => {
                let collection_decode_result =
                    bincode::decode_from_slice(&collection_fc[..], bincode::config::standard());
                match collection_decode_result {
                    Ok((collection, _len)) => collection,
                    Err(e) => {
                        println!(
                            "Unable to decode {}: {}",
                            collection_fp.to_str().unwrap(),
                            e
                        );
                        Collection::default()
                    }
                }
            }
            Err(e) => {
                println!("Unable to read {}: {}", collection_fp.to_str().unwrap(), e);
                Collection::default()
            }
        }
    }

    pub fn persist(&self, fp: PathBuf) {
        let collection_file_path = fp.join("ds.bin");
        let collection_encode_result = bincode::encode_to_vec(self, bincode::config::standard());
        match collection_encode_result {
            Ok(collection_encoded) => {
                let collection_write_result =
                    std::fs::write(collection_file_path.clone(), collection_encoded);
                match collection_write_result {
                    Ok(()) => {
                        println!("{} was written.", collection_file_path.to_str().unwrap())
                    }
                    Err(e) => {
                        println!(
                            "Unable to write {}: {}.",
                            collection_file_path.to_str().unwrap(),
                            e
                        )
                    }
                }
            }
            Err(e) => {
                println!(
                    "Unable to persist {}: {}.",
                    collection_file_path.to_str().unwrap(),
                    e
                );
            }
        }
    }
}
