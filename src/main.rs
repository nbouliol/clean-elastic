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
        .arg(Arg::with_name("list-index")
            .short("i")
            .help("list indexes"))
        .arg(Arg::with_name("list-alias")
            .short("a")
            .help("list alias"))
        .get_matches();

	let elastic_url: String; // = env::var("ELASTIC_URL").unwrap();
	match env::var("ELASTIC_URL") {
        Ok(url) => elastic_url = url,
        Err(_e) => elastic_url = "http://localhost:9200".to_string(),
    };

	println!("Elastic url : {}", elastic_url);

	let elastic_infos = elastic::get_indexes(&elastic_url).unwrap();

	if matches.is_present("list-index") {
        let space = elastic::get_max_length(&elastic_infos).expect("Error getting max shit");
        for e in elastic_infos {
            println!("{0:width$} {1: >30} {2}", e.index, e.health, e.doc_count, width = space + 5 );
        }
	}

}
