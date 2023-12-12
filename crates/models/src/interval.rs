#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Interval<T: PartialOrd> {
    start: T,
    end: T,
}
