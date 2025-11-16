use chrono::Utc;
use dirs::home_dir;
use serde::{Serialize, Deserialize};
use std::{fs, path::PathBuf};

#[derive(Serialize, Deserialize, Debug)]
pub struct Memory {
    pub data: Vec<String>,
    pub last_updated: String,
}

impl Memory {
    pub fn path() -> PathBuf {
        let mut p = home_dir().unwrap();
        p.push(".coffee_break_memory.json");
        p
    }

    pub fn load() -> Self {
        let path = Self::path();
        if path.exists() {
            let raw = fs::read_to_string(path).ok();
            if let Some(json) = raw {
                serde_json::from_str(&json).unwrap_or_else(|_| Self::empty())
            } else {
                Self::empty()
            }
        } else {
            Self::empty()
        }
    }

    pub fn empty() -> Self {
        Self {
            data: vec![],
            last_updated: Utc::now().to_rfc3339(),
        }
    }

    pub fn save(&mut self) {
        self.last_updated = Utc::now().to_rfc3339();
        let raw = serde_json::to_string_pretty(self).unwrap();
        let _ = fs::write(Self::path(), raw);
    }
}


