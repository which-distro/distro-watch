#![doc = include_str!("../README.md")]

mod result;

use getset::Getters;
use reqwest::{Client, StatusCode};
use serde_json::Value;
use std::str::FromStr;

pub use result::{Error, Result};

/// A distro's status.
///
/// For more information, see
/// [faq: distro status](https://distrowatch.com/dwres.php?resource=faq#distrostatus).
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Status {
    /// This distro is under active development.
    Active,
    /// This distro has not put out a new release in two or more years, or it
    /// no longer plans to put out future releases, ie planned inactivity,
    /// while older releases are maintained.
    Dormant,
    /// This distro is no longer being developed or have websites.
    Discontinued,
}

impl FromStr for Status {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        if s.starts_with("Active") {
            Ok(Self::Active)
        } else if s.starts_with("Dormant") {
            Ok(Self::Dormant)
        } else if s.starts_with("Discontinued") {
            Ok(Self::Discontinued)
        } else {
            Err(())
        }
    }
}

/// Distro's Metadata.
#[derive(Getters, Debug, Clone, PartialEq)]
pub struct DistroMetadata {
    /// Distro's name.
    #[getset(get = "pub")]
    name: String,
    /// Which distros is this distro based on.
    #[getset(get = "pub")]
    based_on: Vec<String>,
    /// Origin of this distro.
    #[getset(get = "pub")]
    origin: String,
    /// CPU architectures supported by this distro.
    #[getset(get = "pub")]
    architecture: Vec<String>,
    /// Desktop Environments supported by this distro.
    #[getset(get = "pub")]
    desktop: Vec<String>,
    /// What categories does this distro belong to.
    #[getset(get = "pub")]
    category: Vec<String>,
    /// Distro's status.
    #[getset(get = "pub")]
    status: Status,
    /// Popularity level of this distro.
    #[getset(get = "pub")]
    popularity: String,
}

/// Fetch a distro's metadata from DistroWatch.
pub async fn stat<S: AsRef<str>>(distro: S) -> Result<Option<DistroMetadata>> {
    let client = Client::new();
    let url = format!(
        "https://diwa.demo-web-fahmi.my.id/api/v2/distributions/{}",
        distro.as_ref(),
    );
    let response = client.get(url.as_str()).send().await?;

    if response.status() == StatusCode::NOT_FOUND {
        return Ok(None);
    }

    let response_json = response.json::<Value>().await?;

    let based_on = response_json
        .get("based_ons")
        .ok_or(Error::DistroWatchApiChanged)?
        .as_array()
        .unwrap()
        .iter()
        .map(|val| val.as_str().map(|str| str.to_string()))
        .collect::<Option<Vec<String>>>()
        .ok_or(Error::DistroWatchApiChanged)?;

    let origin = response_json
        .get("origin")
        .ok_or(Error::DistroWatchApiChanged)?
        .as_str()
        .map(|str| str.to_string())
        .ok_or(Error::DistroWatchApiChanged)?;

    let architecture = response_json
        .get("architectures")
        .ok_or(Error::DistroWatchApiChanged)?
        .as_array()
        .unwrap()
        .iter()
        .map(|val| val.as_str().map(|str| str.to_string()))
        .collect::<Option<Vec<String>>>()
        .ok_or(Error::DistroWatchApiChanged)?;

    let desktop = response_json
        .get("desktop_environments")
        .ok_or(Error::DistroWatchApiChanged)?
        .as_array()
        .unwrap()
        .iter()
        .map(|val| val.as_str().map(|str| str.to_string()))
        .collect::<Option<Vec<String>>>()
        .ok_or(Error::DistroWatchApiChanged)?;

    let category = response_json
        .get("categories")
        .ok_or(Error::DistroWatchApiChanged)?
        .as_array()
        .unwrap()
        .iter()
        .map(|val| val.as_str().map(|str| str.to_string()))
        .collect::<Option<Vec<String>>>()
        .ok_or(Error::DistroWatchApiChanged)?;

    let status = response_json
        .get("status")
        .ok_or(Error::DistroWatchApiChanged)?
        .as_str()
        .ok_or(Error::DistroWatchApiChanged)?
        .parse::<Status>()
        .map_err(|_| Error::DistroWatchApiChanged)?;

    let popularity = response_json
        .get("popularity")
        .ok_or(Error::DistroWatchApiChanged)?
        .as_str()
        .map(|str| str.to_string())
        .ok_or(Error::DistroWatchApiChanged)?;

    Ok(Some(DistroMetadata {
        name: distro.as_ref().to_string(),
        based_on,
        origin,
        architecture,
        desktop,
        category,
        status,
        popularity,
    }))
}
