use std::{fs::File, io::BufReader};

use log::warn;
use rodio::Decoder;

use crate::asset_info::{AssetInfo, AssetStatus};

pub struct Sound {
    pub asset_info: AssetInfo,
    pub source: Option<Decoder<BufReader<File>>>,
}

impl Sound {
    pub fn load(&mut self) {
        let file = File::open(&self.asset_info.id);

        if file.is_err() {
            self.asset_info.status = AssetStatus::Invalid;

            warn!("Failed to load sound file: {}", self.asset_info.id);
            return;
        }

        let buffer = BufReader::new(file.unwrap());

        let source = Decoder::new(buffer);

        if source.is_err() {
            self.asset_info.status = AssetStatus::Invalid;

            warn!("Failed to decode sound file: {}", self.asset_info.id);
            return;
        }

        self.source = Some(source.unwrap());
        self.asset_info.status = AssetStatus::Loaded;

        warn!("Loaded sound file: {}", self.asset_info.id);
    }
}
