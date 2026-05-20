use std::{error::Error, fs::File, io::BufReader, path::Path};

use reqwest::{blocking::Client, header::USER_AGENT};
use serde::{Deserialize, Serialize};
use serde_json::from_reader;

use crate::overpass::models::OverpassResult;

static OVERPASS_API_URL: &'static str = "https://overpass-api.de/api/interpreter";

pub struct Ensurer {
    client: Client,
}

#[derive(Debug, Serialize)]
struct Query<'a> {
    data: &'a str,
}

impl Ensurer {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    /// Writes the results of `query` to `file`. Skips if `file` exists.
    pub fn ensure<'a>(
        &self,
        file: &'a str,
        query: &str,
    ) -> Result<EnsuredFile<'a>, Box<dyn Error>> {
        if Path::new(file).exists() {
            eprintln!("Using cached result in {file} (delete to regenerate)");
        } else {
            self.client
                .post(OVERPASS_API_URL)
                .header(USER_AGENT, "https://github.com/togethered/jetlag-map")
                .form(&Query { data: query })
                .send()?
                .copy_to(&mut File::create(file)?)?;
            eprintln!("Cached result to {file}");
        }
        Ok(EnsuredFile { path: file })
    }
}

pub struct EnsuredFile<'a> {
    path: &'a str,
}

impl EnsuredFile<'_> {
    pub fn read<T: for<'a> Deserialize<'a>>(&self) -> Result<OverpassResult<T>, Box<dyn Error>> {
        Ok(from_reader(BufReader::new(File::open(self.path)?))?)
    }
}
