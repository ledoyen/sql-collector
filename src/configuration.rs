#[derive(Debug, Deserialize)]
pub struct Configuration {
    pub sources: Vec<Source>
}

#[derive(Debug, Deserialize)]
pub struct Source {
    pub name: String,
    #[serde(rename="type")]
    pub source_type: SourceType,
    pub url: String,
    pub query: String
}

#[derive(Debug, Deserialize)]
pub enum SourceType {
    MYSQL
}

use std::fs::File;
use std::io::BufReader;
use serde_yaml;

impl Configuration {
    pub fn new(args: &[String]) -> Result<Configuration, String> {
        if args.len() < 2 {
            return Err("not enough arguments".to_owned());
        }

        let filename = &args[1];
        println!("Loading: {}", filename);

        let file = File::open(filename).expect(&("unable to open file".to_owned() + filename)[..]);
        let buf_reader = BufReader::new(file);

        serde_yaml::from_reader(buf_reader).map_err(|err| err.to_string())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn configuration_is_loaded_from_test_file() {
        Configuration::new(&[String::from("typically path to executable"), String::from("test/configuration.yml")]).expect("Unable to parse configuration");
    }
}
