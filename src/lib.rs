use anyhow::Result;
use getset::{CopyGetters, Getters};
use voyager::{Crawler, Response, Scraper};

/// Scraper for DistroWatch
#[derive(Debug, Copy, Clone)]
struct DistroWatchScraper;

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

#[derive(Getters, CopyGetters, Debug, Clone, PartialEq)]
pub struct DistroInfo {
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
    #[getset(get_copy = "pub")]
    popularity: u64,
}

impl Scraper for DistroWatchScraper {
    type Output = DistroInfo;
    type State = ();

    fn scrape(
        &mut self,
        response: Response<Self::State>,
        crawler: &mut Crawler<Self>,
    ) -> Result<Option<Self::Output>> {
        todo!()
    }
}

/// Fetch a distro's info from DistroWatch.
pub async fn stat<S: AsRef<str>>(distro: S) -> Result<Option<DistroWatchScraper>> {
    todo!()
}
