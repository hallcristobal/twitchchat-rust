use super::error::Error;
use std::str::FromStr;

#[derive(Eq, PartialEq, Debug)]
pub enum Badge {
    Admin(usize),
    Bits(usize),
    Broadcaster(usize),
    GlobalMod(usize),
    Moderator(usize),
    Subscriber(usize),
    Staff(usize),
    Turbo(usize),
    Partner(usize),
}

impl FromStr for Badge {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use self::Badge::*;

        let mut split = s.split("/");
        let badge = split.next().unwrap();
        let version = split.next().unwrap().parse()?;
        match badge {
            "admin" => Ok(Admin(version)),
            "bits" => Ok(Bits(version)),
            "broadcaster" => Ok(Broadcaster(version)),
            "global_mod" => Ok(GlobalMod(version)),
            "moderator" => Ok(Moderator(version)),
            "subscriber" => Ok(Subscriber(version)),
            "staff" => Ok(Staff(version)),
            "turbo" => Ok(Turbo(version)),
            "partner" => Ok(Partner(version)),
            _ => Err(format!("Unknown input for Badge: {}", s).into()),
        }
    }
}
