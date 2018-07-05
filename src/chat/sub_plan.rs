#[derive(Eq, PartialEq, Debug)]
pub enum SubPlan {
    Prime,
    Tier1,
    Tier2,
    Tier3,
    Other(String),
}
