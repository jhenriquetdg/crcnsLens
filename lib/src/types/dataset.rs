use std::borrow::BorrowMut;
use std::sync::{Arc, Mutex};

use crate::files::get_file;
use crate::global;
use crate::net::get_crcns_file;
use crate::net::get_url_html::get_url_html;
use crate::net::write_response::write_response;
use crate::types::state::SrPair;
// use crate::net::write_response::write_response_with_sender;
use egui::TextBuffer;
use std::str::FromStr;

#[derive(Debug, Clone, bincode::Encode, bincode::Decode)]
pub struct Dataset {
    pub url: String,
    pub html: String,
    pub alias: String,
    pub content: String,
    pub description: String,
    pub last_modified: String,
}

impl PartialEq for Dataset {
    fn eq(&self, other: &Dataset) -> bool {
        self.alias == other.alias
    }
}
impl Eq for Dataset {}

impl Default for Dataset {
    fn default() -> Self {
        Dataset {
            url: String::from_str("").unwrap(),
            html: String::from_str("<html></html>").unwrap(),
            alias: String::from_str("Default").unwrap(),
            content: String::from_str("").unwrap(),
            description: String::from_str("Default Dataset").unwrap(),
            last_modified: chrono::Utc::now().to_string(),
        }
    }
}

impl Dataset {
    pub async fn from_url(url: url::Url) -> Dataset {
        println!("{url}");
        let html = get_url_html(url.clone()).await;
        let package = sxd_html::parse_html(html.as_str());
        let document = package.as_document();

        let alias = url.path_segments().unwrap().last().unwrap().to_string();

        let description =
            sxd_xpath::evaluate_xpath(&document, "//div[@class='documentDescription']//text()")
                .expect("Unable to find h1 descriptor")
                .string();

        // let content = String::new();
        let content = match sxd_xpath::evaluate_xpath(&document, "//div[@id='content']").unwrap() {
            sxd_xpath::Value::Nodeset(ns) => {
                let node = ns.document_order_first().unwrap();
                node.string_value()
            }
            _ => panic!("Unable to get content"),
        };

        // let content = content.nodes.iter.next();

        let modified = chrono::Utc::now();

        let url = url.to_string();
        let modified = modified.to_string();

        Dataset {
            url,
            html,
            alias,
            content,
            description,
            last_modified: modified,
        }
    }

    pub fn persist(&self, fp: std::path::PathBuf) {
        let dataset_file_path = fp.join("ds.bin");
        let dataset_encode_result = bincode::encode_to_vec(self, bincode::config::standard());
        match dataset_encode_result {
            Ok(dataset_encoded) => {
                let dataset_write_result =
                    std::fs::write(dataset_file_path.clone(), dataset_encoded);
                match dataset_write_result {
                    Ok(()) => {
                        println!("{} was written.", dataset_file_path.to_str().unwrap())
                    }
                    Err(e) => {
                        println!(
                            "Unable to write {}: {}.",
                            dataset_file_path.to_str().unwrap(),
                            e
                        )
                    }
                }
            }
            Err(e) => {
                println!(
                    "Unable to persist {}: {}.",
                    dataset_file_path.to_str().unwrap(),
                    e
                );
            }
        }
    }

    pub fn from_filepath(fp: std::path::PathBuf) -> Self {
        let dataset_fp = fp.join("ds.bin");
        let dataset_fc_result = std::fs::read(dataset_fp.clone());
        match dataset_fc_result {
            Ok(dataset_fc) => {
                let dataset_decode_result =
                    bincode::decode_from_slice(&dataset_fc[..], bincode::config::standard());
                match dataset_decode_result {
                    Ok((dataset, _len)) => dataset,
                    Err(e) => {
                        println!("Unable to decode {}: {}", dataset_fp.to_str().unwrap(), e);
                        Dataset::default()
                    }
                }
            }
            Err(e) => {
                println!("Unable to read {}: {}", dataset_fp.to_str().unwrap(), e);
                Dataset::default()
            }
        }
    }
    pub async fn get_crcns_file(
        &self,
        collection_alias: &str,
        filepath: &str,
        _sr: Arc<Mutex<SrPair>>,
    ) {
        let remote_filepath = format!("{}/{}", self.alias, filepath);
        println!("{remote_filepath}");
        let remote_response = get_crcns_file(remote_filepath.as_str()).await;

        // Construct the file path
        let local_filepath = global::get_state()
            .working_directory
            .join("data")
            .join(collection_alias.as_str())
            .join(remote_filepath);

        let mut local_file = get_file(local_filepath).await.unwrap();

        write_response(local_file.borrow_mut(), remote_response).await;
        // write_response_with_sender(local_file.borrow_mut(), remote_response, sr).await;

        // Create directories if they don't exist
    }
}
