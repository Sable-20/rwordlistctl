use color_eyre::{
    eyre::{eyre, WrapErr},
    Result,
};
use log::debug;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Repo {
    wordlists: Vec<HashMap<String, Wordlist>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Wordlist {
    url: String,
    size: f64,
    unit: String,
    group: String,
}

impl Wordlist {
    pub fn get_size(&self) -> f64 {
        self.size
    }

    pub fn get_unit(&self) -> &str {
        &self.unit
    }

    pub fn get_url(&self) -> &str {
        &self.url
    }

    pub fn get_group(&self) -> &str {
        &self.group
    }
}

fn load_repo() -> Result<Repo> {
    let repo = toml::from_str::<Repo>(include_str!("../config/repo.toml"))
        .wrap_err_with(|| eyre!("Failed to read repository from repo.toml"))?;
    Ok(repo)
}

pub fn get_wordlist_by_group(group: String) -> Result<Vec<Wordlist>> {
    let repo: Repo = load_repo()?;
    let wordlists = repo
        .wordlists
        .into_iter()
        .filter(|wordlist| {
            wordlist
                .values()
                .next()
                .expect("Filter of grouips failed")
                .group
                == group
        })
        .map(|wordlist| {
            wordlist
                .values()
                .next().cloned()
                .expect("Map of groups failed")
        })
        .collect::<Vec<Wordlist>>(); // Collect the wordlists into a vector

    Ok(wordlists)
}

pub fn get_wordlist_by_name_regex(name: &str) -> Result<Vec<Wordlist>> {
    let repo: Repo = load_repo()?;

    let re = regex::Regex::new(name).unwrap();
    let results = repo.wordlists[0]
        .keys()
        .filter(|key| re.is_match(key))
        .collect::<Vec<&String>>();

    let ret = results
        .iter()
        .map(|key| repo.wordlists[0].get(*key).unwrap().clone())
        .collect::<Vec<Wordlist>>();

    Ok(ret)
}

pub fn get_wordlist_by_name(name: &str) -> Result<Wordlist> {
    Ok(load_repo()?.wordlists[0].get_key_value(name).unwrap().1.clone())
}
