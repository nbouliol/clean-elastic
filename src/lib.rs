extern crate reqwest;
extern crate termion;

use termion::{color, style};
use reqwest::Url;
use std::fmt;

#[derive(Debug)]
pub enum Health {
    Green,
    Yellow,
    Red
}

impl fmt::Display for Health {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let display = match &self {
          Health::Green => format!("{}Green{}", color::Fg(color::Green), style::Reset),
          Health::Red => format!("{}Red{}", color::Fg(color::Red), style::Reset),
          Health::Yellow => format!("{}Yellow{}", color::Fg(color::Yellow), style::Reset),
        };
        write!(f, "{}",display)
    }
}

#[derive(Debug)]
pub struct ElasticInfos {
    pub alias : String,
    pub index: String,
    pub doc_count: i64,
    pub health: Health
}

impl ElasticInfos {
    pub fn new(a: &str, i: &str, docs: i64, health: &str) -> ElasticInfos {
        ElasticInfos {alias:"".to_string(), index:i.to_string(), doc_count: docs, health: Health::Yellow}
    }
}

pub fn get_max_length(vec: &Vec<ElasticInfos>) -> Result<usize, Box<std::error::Error>> {
    let mut max : usize = 0;
    for v in vec {
        let value = v.index.len();
        if value > max {
            max = value;
        }
    }
    Ok(max)
}

pub fn get_indexes(el_url: &str) -> Result<Vec<ElasticInfos>,  Box<std::error::Error>> {
	// "http://localhost:9200"
    // let mut url = el_url.to_owned() + "/_cat/indices";
    let url = Url::parse(el_url).unwrap();
    let url = url.join("/_cat/indices").unwrap();
    
    let resp = reqwest::get(url)?
            .text()?;

    let mut ret : Vec<ElasticInfos> = Vec::new();

    for line in resp.lines() {
        let mut vec: Vec<&str> = line.split(" ").collect();
        vec.retain(|&v| v != "");

        ret.push(ElasticInfos::new(vec[0], vec[2], vec[4].parse::<i64>().unwrap(), vec[0]));
    }

    Ok(ret)
}

pub fn clean_indexes(infos: &Vec<ElasticInfos>) -> Result<(), Box<std::error::Error>> {
    let client = reqwest::Client::new();

    for i in infos {
        let del_alias_url = format!("http://localhost:9200/{}/_alias/{}", i.index, i.alias);
		let del_index_url = format!("http://localhost:9200/{}", i.index);

        client.delete(&del_alias_url).send()?;
        client.delete(&del_index_url).send()?;

        println!("Deleted index {} and alias {}", i.index, i.alias);
    }

    Ok(())
}
