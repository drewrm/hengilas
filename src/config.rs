use dirs;
use serde::{ Deserialize, Serialize };


#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub font_family: Option<String>,
    pub font_size: Option<u8>,
    #[serde(default)]
    pub header: HeaderConfig,
    #[serde(default)]
    pub subtitle: SubtitleConfig,
    #[serde(default)]
    pub overlay: OverlayConfig,
    #[serde(default)]
    pub focus: FocusConfig,
    #[serde(default)]
    pub button: ButtonConfig,
    #[serde(default)]
    pub background: BackgroundConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            font_family: Some("Sans".to_string()),
            font_size: Some(14),
            header: HeaderConfig::default(),
            subtitle: SubtitleConfig::default(),
            overlay: OverlayConfig::default(),
            focus: FocusConfig::default(),
            button: ButtonConfig::default(),
            background: BackgroundConfig::default(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HeaderConfig {
    pub font_family: Option<String>,
    pub font_size: Option<u8>,
    pub text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubtitleConfig {
    pub font_family: Option<String>,
    pub font_size: Option<u8>,
    pub text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OverlayConfig {
    pub color: Option<String>,
    pub opacity: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FocusConfig {
    pub color: Option<String>,
    pub width: Option<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ButtonConfig {
    pub background: Option<String>,
    pub foreground: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BackgroundConfig {
    pub image: Option<String>,
}

impl Default for HeaderConfig {
    fn default() -> Self {
        Self {
            font_family: None,
            font_size: None,
            text: None,
        }
    }
}

impl Default for SubtitleConfig {
    fn default() -> Self {
        Self {
            font_family: None,
            font_size: None,
            text: None,
        }
    }
}

impl Default for OverlayConfig {
    fn default() -> Self {
        Self {
            color: Some("#000000".to_string()),
            opacity: Some(0.65),
        }
    }
}

impl Default for FocusConfig {
    fn default() -> Self {
        Self {
            color: None,
            width: None,
        }
    }
}

impl Default for ButtonConfig {
    fn default() -> Self {
        Self {
            background: None,
            foreground: None,
        }
    }
}

impl Default for BackgroundConfig {
    fn default() -> Self {
        Self {
            image: None,
        }
    }
}

impl Config {
    pub fn load() -> Config {
        let config_path = dirs::config_dir()
            .expect("Failed to get config directory")
            .join("hengilas")
            .join("config.toml");

        if config_path.exists() {
            let config_str = std::fs::read_to_string(config_path).expect("Failed to read config file");
            toml::from_str(&config_str).expect("Failed to parse config")
        } else {
            log::warn!("Config file not found at {:?}, using default config", config_path);
            Config::default()
        }
    }
}
