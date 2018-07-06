use super::error::Error;
use std::str::FromStr;

#[derive(Eq, PartialEq, Debug)]
pub enum UserType {
    Mod,
    GlobalMod,
    Admin,
    Staff,
    None,
}

impl FromStr for UserType {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "mod" => Ok(UserType::Mod),
            "global_mod" => Ok(UserType::GlobalMod),
            "admin" => Ok(UserType::Admin),
            "staff" => Ok(UserType::Staff),
            "" => Ok(UserType::None),
            _ => Err(String::from("Unknown Input").into()),
        }
    }
}
