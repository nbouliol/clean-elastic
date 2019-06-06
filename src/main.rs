extern crate reqwest;
extern crate clap;

use elastic;
use clap::{Arg, App};
use std::env;

fn main() {
	 let matches = App::new("Elastic")
        .version("0.1")
        .author("nbo")
        .about("Elsatic management 4 the noobs")
        .arg(Arg::with_name("list")
            .short("l")
            .help("list indexes and aliases"))
        .get_matches();

	let elastic_url: String; // = env::var("ELASTIC_URL").unwrap();
	match env::var("ELASTIC_URL") {
        Ok(url) => elastic_url = url,
        Err(_e) => elastic_url = "http://localhost:9200/_cat/aliases".to_string(),
    };

	println!("Elastic url : {}", elastic_url);

	let elastic_infos = elastic::get_indexes(elastic_url);

	if matches.is_present("list") {

	}

    let elastic_infos = elastic::get_indexes();
}
