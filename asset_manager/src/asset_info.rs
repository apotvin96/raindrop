pub enum AssetStatus {
    Unloaded,
    Loaded,
    Uploaded,
    Optimized,
}

pub struct AssetInfo {
    pub id: String,
    pub status: AssetStatus,
}