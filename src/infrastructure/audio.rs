use crate::domain::audio::{KeyDefine, KeyDefineType, MechvibesConfig};
use anyhow::{Context, Result};
use rodio::{Decoder, Source};
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::result::Result::Ok;
use std::sync::Arc;
use std::time::Duration;

#[derive(Clone, Debug)]
pub struct AudioBuffer {
    pub samples: Arc<[i16]>,
    pub sample_rate: u32,
    pub channels: u16,
}

impl AudioBuffer {
    pub fn to_source(&self) -> AudioBufferSource {
        AudioBufferSource {
            buffer: self.samples.clone(),
            pos: 0,
            sample_rate: self.sample_rate,
            channels: self.channels,
            end_pos: self.samples.len(),
        }
    }

    pub fn to_source_slice(&self, start_ms: u64, duration_ms: u64) -> AudioBufferSource {
        let start_pos =
            (start_ms * self.sample_rate as u64 / 1000) as usize * self.channels as usize;
        let duration_pos =
            (duration_ms * self.sample_rate as u64 / 1000) as usize * self.channels as usize;
        let end_pos = (start_pos + duration_pos).min(self.samples.len());

        AudioBufferSource {
            buffer: self.samples.clone(),
            pos: start_pos,
            sample_rate: self.sample_rate,
            channels: self.channels,
            end_pos,
        }
    }
}

pub struct AudioBufferSource {
    buffer: Arc<[i16]>,
    pos: usize,
    sample_rate: u32,
    channels: u16,
    end_pos: usize,
}

impl Iterator for AudioBufferSource {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos < self.end_pos {
            let res = self.buffer[self.pos] as f32 / 32767.0;
            self.pos += 1;
            Some(res)
        } else {
            None
        }
    }
}

impl Source for AudioBufferSource {
    fn current_span_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        self.channels
    }

    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn total_duration(&self) -> Option<Duration> {
        Some(Duration::from_secs_f64(
            (self.end_pos - self.pos) as f64 / self.sample_rate as f64 / self.channels as f64,
        ))
    }
}

#[derive(Debug)]
pub struct LoadedSoundPack {
    pub config: MechvibesConfig,
    pub buffers: HashMap<String, AudioBuffer>,
}

pub struct SoundPackLoader;

const ALLOWED_SOUND_BASES: &[&str] = &[
    "src/assets/sounds",
    "assets/sounds",
    "/usr/share/hibiki/sounds",
    "/usr/local/share/hibiki/sounds",
];

impl SoundPackLoader {
    fn safe_join(base: &Path, sub: &str) -> Result<PathBuf> {
        let sub_path = Path::new(sub);
        if sub_path.is_absolute()
            || sub_path
                .components()
                .any(|c| matches!(c, std::path::Component::ParentDir))
        {
            anyhow::bail!("Forbidden path component: {}", sub);
        }

        let joined = base.join(sub_path);

        if !joined.exists() {
            return Ok(joined);
        }

        let canon_joined = joined
            .canonicalize()
            .with_context(|| format!("Failed to canonicalize path: {:?}", joined))?;
        let canon_base = base
            .canonicalize()
            .with_context(|| format!("Failed to canonicalize base path: {:?}", base))?;

        if !canon_joined.starts_with(canon_base) {
            anyhow::bail!("Path traversal detected via symlink: {}", sub);
        }

        Ok(canon_joined)
    }

    pub fn get_sound_pack_dir() -> PathBuf {
        if let Ok(env_path) = std::env::var("HIBIKI_SOUNDS_DIR") {
            let path = PathBuf::from(env_path);
            if path.exists() && path.is_dir() {
                return path;
            }
        }

        for base in ALLOWED_SOUND_BASES {
            let path = PathBuf::from(base);
            if path.exists() && path.is_dir() {
                return path;
            }
        }

        PathBuf::from("assets/sounds")
    }

    pub fn list_available_packs() -> Vec<(String, String)> {
        let mut packs = Vec::new();
        let path = Self::get_sound_pack_dir();

        if let Ok(entries) = fs::read_dir(&path) {
            for entry in entries.flatten() {
                if let Ok(file_type) = entry.file_type() {
                    if file_type.is_dir() {
                        if let Ok(dir_name) = entry.file_name().into_string() {
                            let config_path = entry.path().join("config.json");
                            let display_name = if let Ok(file) = File::open(&config_path) {
                                let reader = BufReader::new(file);
                                if let Ok(json) =
                                    serde_json::from_reader::<_, serde_json::Value>(reader)
                                {
                                    json.get("name")
                                        .and_then(|v| v.as_str())
                                        .map(|s| s.to_string())
                                        .unwrap_or_else(|| dir_name.clone())
                                } else {
                                    dir_name.clone()
                                }
                            } else {
                                dir_name.clone()
                            };
                            packs.push((dir_name, display_name));
                        }
                    }
                }
            }
        }
        packs.sort_by(|a, b| a.1.cmp(&b.1));
        packs
    }

    pub fn load_from_directory(path: &Path) -> Result<LoadedSoundPack> {
        if path
            .components()
            .any(|c| matches!(c, std::path::Component::ParentDir))
        {
            anyhow::bail!("Path traversal detected: {:?}", path);
        }

        let path = path
            .canonicalize()
            .with_context(|| format!("Failed to canonicalize pack path: {:?}", path))?;

        let config_path = path.join("config.json");
        let config_file = File::open(&config_path)
            .with_context(|| format!("Failed to open config.json at {:?}", config_path))?;
        let reader = BufReader::new(config_file);
        let config: MechvibesConfig = serde_json::from_reader(reader)
            .with_context(|| format!("Failed to parse config.json at {:?}", config_path))?;

        let mut buffers = HashMap::new();

        match config.key_define_type {
            KeyDefineType::Single => {
                let full_path = Self::safe_join(&path, &config.sound)?;
                let file = File::open(&full_path)
                    .with_context(|| format!("Failed to open sound file: {:?}", full_path))?;
                let reader = BufReader::new(file);
                let decoder = Decoder::new(reader)
                    .with_context(|| format!("Failed to decode sound: {:?}", full_path))?;

                let sample_rate = decoder.sample_rate();
                let channels = decoder.channels();
                let samples: Vec<i16> = decoder.map(|s| (s * 32767.0) as i16).collect();

                buffers.insert(
                    "main".to_string(),
                    AudioBuffer {
                        samples: Arc::from(samples),
                        sample_rate,
                        channels,
                    },
                );
            }
            KeyDefineType::Multi => {
                for define in config.defines.values() {
                    if let KeyDefine::Multi(Some(rel_path)) = define {
                        let full_path = Self::safe_join(&path, rel_path)?;
                        if full_path.exists() {
                            let file = File::open(&full_path).with_context(|| {
                                format!("Failed to open sound file: {:?}", full_path)
                            })?;
                            let reader = BufReader::new(file);
                            if let Ok(decoder) = Decoder::new(reader) {
                                let sample_rate = decoder.sample_rate();
                                let channels = decoder.channels();
                                let samples: Vec<i16> =
                                    decoder.map(|s| (s * 32767.0) as i16).collect();
                                buffers.insert(
                                    rel_path.clone(),
                                    AudioBuffer {
                                        samples: Arc::from(samples),
                                        sample_rate,
                                        channels,
                                    },
                                );
                            }
                        }
                    }
                }
            }
        }

        Ok(LoadedSoundPack { config, buffers })
    }
}
