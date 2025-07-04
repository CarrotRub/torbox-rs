use std::collections::HashMap;

use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct TorrentStatus {
    pub id: u64,
    pub hash: String,
    pub created_at: Option<DateTime<FixedOffset>>,
    pub updated_at: Option<DateTime<FixedOffset>>,
    pub magnet: Option<String>,
    pub size: f64,
    pub active: bool,
    pub auth_id: String,
    pub download_state: TorrentDownloadState,
    pub seeds: u64,
    pub peers: u64,
    pub ratio: f64,
    pub progress: f64,
    pub download_speed: f64,
    pub upload_speed: f64,
    pub name: String,
    pub eta: f64,
    pub server: u64,
    pub torrent_file: bool,
    pub expires_at: Option<DateTime<FixedOffset>>,
    pub download_present: bool,
    pub download_finished: bool,
    pub files: Vec<TorrentFile>,
    pub inactive_check: Option<u64>,
    pub availability: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct TorrentMeta {
    pub name: String,
    pub hash: String,
    pub size: u64,
    pub trackers: Vec<String>,
    pub seeds: u64,
    pub peers: u64,
    pub files: Vec<TorrentFile>,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct TorrentData {
    pub name: String,
    pub size: u64,
    pub hash: String,
    pub files: Vec<TorrentFile>,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct TorrentFile {
    pub name: String,
    pub size: f64,
    pub hash: Option<String>,
}

pub type TorrentMap = HashMap<String, TorrentFile>;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub enum TorrentDownloadState {
    Downloading,
    #[serde(rename = "uploading (no peers)")]
    Uploading,
    #[serde(rename = "stalled (no seeds)")]
    Stalled,
    Paused,
    Completed,
    Cached,
    #[serde(rename = "metaDL")]
    MetaDl,
    #[serde(rename = "checkingResumeData")]
    CheckingResumeData,
}
