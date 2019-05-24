extern crate reqwest;

use elastic;

fn main() -> Result<(), Box<std::error::Error>> {
    let elastic_infos = elastic::get_indexes();
    println!("{:?}", elastic_infos);
    elastic::clean_indexes(&elastic_infos?).expect("Something wrong happened");
    Ok(())
}
