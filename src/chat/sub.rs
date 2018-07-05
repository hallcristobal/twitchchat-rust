use super::SubPlan;

#[derive(Eq, PartialEq, Debug)]
pub struct Sub {
    months: usize,
    sub_plan: SubPlan,
    sub_plan_name: String,
}
