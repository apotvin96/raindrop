#[derive(Debug, PartialEq)]
pub enum AssetStatus {
    Invalid,
    Unloaded,
    Loaded,
    Uploaded,
    Optimized,
}

pub struct AssetInfo {
    pub id: String,
    pub status: AssetStatus,
}
