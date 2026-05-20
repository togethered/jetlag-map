use std::{error::Error, fs::File, path::Path};

use reqwest::{blocking::Client, header::USER_AGENT};
use serde::Serialize;

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
    pub fn ensure(&self, file: &str, query: &str) -> Result<(), Box<dyn Error>> {
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
        Ok(())
    }
}
