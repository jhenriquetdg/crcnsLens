use crate::net::get_url_html::get_url_html;
use crate::types::collection::Collection;
use crate::types::dataset::Dataset;

use std::path::PathBuf;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use url as urllib;

pub struct CRCNS;

impl CRCNS {
    pub async fn get(collections: Arc<Mutex<Vec<Collection>>>) {
        let crcns_sitemap_url = urllib::Url::from_str("https://crcns.org/sitemap").unwrap();
        let crcns_sitemap_html = get_url_html(crcns_sitemap_url).await;
        let crcns_sitemap_package = sxd_html::parse_html(crcns_sitemap_html.as_str());
        let crcns_sitemap_document = crcns_sitemap_package.as_document();

        let links = sxd_xpath::evaluate_xpath(&crcns_sitemap_document, "//a/@href").expect("Panic");

        let mut urls: Vec<urllib::Url> = match links {
            sxd_xpath::Value::Nodeset(d) => d
                .into_iter()
                .map(|n| n.string_value())
                .filter(|s| s.contains("/data-sets/"))
                .map(|u| urllib::Url::parse(u.as_str()))
                .filter_map(|r| r.ok())
                .collect(),
            _ => panic!("Cannot find anchor tags in CRCNS sitemap."),
        };

        urls.sort();
        let nof_segments = |a: &urllib::Url| a.path_segments().map(|c| c.collect::<Vec<_>>().len());

        let collections_url: Vec<urllib::Url> = urls
            .clone()
            .into_iter()
            .filter(|u| nof_segments(u).unwrap() == 2)
            .collect();

        for (count_c, url) in collections_url.into_iter().enumerate() {
            println!("{}", url.as_str());
            let c = collections.clone();
            let collection_urls: Vec<urllib::Url> = urls
                .clone()
                .into_iter()
                .filter(|u| {
                    u.as_str().contains(url.clone().as_str()) && nof_segments(u).unwrap() == 3
                })
                .collect();

            tokio::spawn(async move {
                let collection = Collection::from_url(url).await;
                for (count_d, ds_url) in collection_urls.into_iter().enumerate() {
                    let d = collection.datasets.clone();
                    tokio::spawn(async move {
                        let dataset = Dataset::from_url(ds_url).await;
                        d.lock().unwrap().push(dataset);
                    });
                    if count_d > 1 {
                        // break;
                    }
                }
                c.lock().unwrap().push(collection);
            });
            if count_c > 1 {
                // break;
            }
        }
        println!("utcny4792d87my3u984ymu2,09m8u59g82yumj9f8tm39 yrmd9uh");
    }

    pub async fn persist(
        collections_arc: Arc<Mutex<Vec<Collection>>>,
        data_directory_path: PathBuf,
    ) {
        'cloop: loop {
            let collections_lock_result = collections_arc.lock();
            match collections_lock_result {
                Ok(collections_mutex) => {
                    for collection in collections_mutex.clone().iter_mut() {
                        let collection_directory_path = data_directory_path.clone();
                        let collection_directory_path =
                            collection_directory_path.join(collection.alias.clone());

                        if !collection_directory_path.exists() {
                            let _ = std::fs::create_dir(collection_directory_path.clone());
                        }

                        let collection_file_path = collection_directory_path.join("c.bin");

                        if !collection_file_path.exists() {
                            collection.persist(collection_directory_path);
                        } else {
                            let persisted_collection =
                                Collection::from_filepath(collection_directory_path.clone());
                            if persisted_collection.last_modified < collection.last_modified {
                                collection.persist(collection_directory_path);
                            } else {
                                *collection = persisted_collection;
                            }
                        }
                        'dloop: loop {
                            let datasets_lock_result = collection.datasets.lock();
                            match datasets_lock_result {
                                Ok(mut datasets_mutex) => {
                                    for dataset in datasets_mutex.iter_mut() {
                                        let mut dataset_directory_path =
                                            data_directory_path.clone();
                                        for f in urllib::Url::parse(dataset.url.as_str())
                                            .unwrap()
                                            .clone()
                                            .path_segments()
                                            .unwrap()
                                            .skip(1)
                                        {
                                            dataset_directory_path.push(f);
                                        }

                                        if !dataset_directory_path.exists() {
                                            let _ =
                                                std::fs::create_dir(dataset_directory_path.clone());
                                        }

                                        let dataset_file_path =
                                            dataset_directory_path.join("ds.bin");

                                        if !dataset_file_path.exists() {
                                            dataset.persist(dataset_directory_path)
                                        } else {
                                            let persisted_dataset = Dataset::from_filepath(
                                                dataset_directory_path.clone(),
                                            );
                                            if persisted_dataset.last_modified
                                                < dataset.last_modified
                                            {
                                                dataset.persist(dataset_directory_path)
                                            } else {
                                                *dataset = persisted_dataset;
                                            }
                                        }
                                    }
                                    break 'dloop;
                                }
                                Err(e) => {
                                    println!("Could not persist CRCNS data: {}", e);
                                    continue 'dloop;
                                }
                            };
                        }
                    }
                    break 'cloop;
                }
                Err(e) => {
                    println!("Could not persist CRCNS data: {}", e);
                    continue 'cloop;
                }
            };
        }
    }
}
