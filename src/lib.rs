extern crate reqwest;

use std::collections::HashMap;

#[derive(Debug)]
pub struct ElasticInfos {
    alias : String,
    index: String
}

impl ElasticInfos {
    pub fn new(a: &str, i: &str) -> ElasticInfos {
        ElasticInfos {alias: a.to_string(), index:i.to_string()}
    }
}


pub fn get_indexes() -> Result<Vec<ElasticInfos>,  Box<std::error::Error>> {
    let resp = reqwest::get("http://localhost:9200/_cat/aliases")?
            .text()?;
    
    let mut ret : Vec<ElasticInfos> = Vec::new();

    for line in resp.lines() {
        let mut vec: Vec<&str> = line.split(" ").collect();
        vec.retain(|&v| v != "");
        
        ret.push(ElasticInfos::new(vec[0], vec[1]));
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