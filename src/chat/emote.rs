use super::error::Error;
use std::str::FromStr;

#[derive(Eq, PartialEq, Debug)]
pub struct Emote {
    emote_id: usize,
    ranges: Vec<(usize, usize)>,
}

impl FromStr for Emote {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(":");
        let emote_id = split.next().unwrap().parse()?;
        let range = split.next().unwrap();

        let mut ranges = Vec::new();

        for range in range.split(",") {
            let mut split = range.split("-");
            ranges.push((
                split.next().unwrap().parse()?,
                split.next().unwrap().parse()?,
            ));
        }

        Ok(Emote { emote_id, ranges })
    }
}
