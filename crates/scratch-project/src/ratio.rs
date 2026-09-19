use serde::{Deserialize, Serialize};

#[cfg(target_os = "windows")]
#[link(name = "user32")]
extern "system" {
    fn GetSystemMetrics(n_index: i32) -> i32;
}

/// Detects the physical display resolution of the primary monitor.
pub fn get_device_screen_resolution() -> (u32, u32) {
    #[cfg(target_os = "windows")]
    unsafe {
        let w = GetSystemMetrics(0); // SM_CXSCREEN = 0
        let h = GetSystemMetrics(1); // SM_CYSCREEN = 1
        if w > 0 && h > 0 {
            return (w as u32, h as u32);
        }
    }
    (1920, 1080)
}

fn gcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

/// Computes human-readable aspect ratio string (e.g. "16:9", "16:10", "4:3", "21:9").
pub fn format_aspect_ratio(w: u32, h: u32) -> String {
    if w == 0 || h == 0 {
        return "16:9".to_string();
    }
    let r = w as f32 / h as f32;
    if (r - 16.0 / 9.0).abs() < 0.03 {
        "16:9".to_string()
    } else if (r - 16.0 / 10.0).abs() < 0.03 || (r - 8.0 / 5.0).abs() < 0.03 {
        "16:10".to_string()
    } else if (r - 4.0 / 3.0).abs() < 0.03 {
        "4:3".to_string()
    } else if (r - 21.0 / 9.0).abs() < 0.05 {
        "21:9".to_string()
    } else if (r - 3.0 / 2.0).abs() < 0.03 {
        "3:2".to_string()
    } else if (r - 1.0).abs() < 0.02 {
        "1:1".to_string()
    } else {
        let g = gcd(w, h);
        let sw = w / g;
        let sh = h / g;
        if sw < 50 && sh < 50 {
            format!("{}:{}", sw, sh)
        } else {
            format!("{:.2}:1", r)
        }
    }
}

/// Standard game ratio picker options.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AspectRatioPreset {
    Ratio16x9,
    Ratio4x3,
    Device,
}

impl AspectRatioPreset {
    pub fn parse_str(s: &str) -> Option<Self> {
        let clean = s.trim().to_lowercase();
        if clean.contains("16:9") || clean.contains("16/9") || clean.contains("16x9") || clean == "widescreen" || clean == "wide" {
            Some(Self::Ratio16x9)
        } else if clean.contains("4:3") || clean.contains("4/3") || clean.contains("4x3") || clean == "classic" || clean == "retro" {
            Some(Self::Ratio4x3)
        } else if clean.contains("device") || clean == "screen" || clean == "native" || clean == "display" || clean == "current" {
            Some(Self::Device)
        } else {
            let (dw, dh) = get_device_screen_resolution();
            let dev_ratio = format_aspect_ratio(dw, dh).to_lowercase();
            if clean == dev_ratio {
                Some(Self::Device)
            } else {
                None
            }
        }
    }

    pub fn to_resolution(&self) -> (u32, u32) {
        match self {
            Self::Ratio16x9 => (1280, 720),
            Self::Ratio4x3 => (960, 720),
            Self::Device => {
                let (dw, dh) = get_device_screen_resolution();
                // Normalize to standard game viewport scale while preserving device ratio
                if dw >= dh {
                    let h = 720;
                    let w = ((720.0 * (dw as f32 / dh as f32)).round() as u32) & !1; // ensure even
                    (w, h)
                } else {
                    let w = 720;
                    let h = ((720.0 * (dh as f32 / dw as f32)).round() as u32) & !1;
                    (w, h)
                }
            }
        }
    }

    pub fn to_aspect(&self) -> f32 {
        let (w, h) = self.to_resolution();
        w as f32 / h.max(1) as f32
    }

    pub fn label(&self) -> String {
        match self {
            Self::Ratio16x9 => "16:9 (1280x720)".to_string(),
            Self::Ratio4x3 => "4:3 (960x720)".to_string(),
            Self::Device => {
                let (dw, dh) = get_device_screen_resolution();
                let ratio_str = format_aspect_ratio(dw, dh);
                format!("Device: {} ({}x{})", ratio_str, dw, dh)
            }
        }
    }

    pub fn short_label(&self) -> String {
        match self {
            Self::Ratio16x9 => "16:9".to_string(),
            Self::Ratio4x3 => "4:3".to_string(),
            Self::Device => {
                let (dw, dh) = get_device_screen_resolution();
                format!("Device ({})", format_aspect_ratio(dw, dh))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_aspect_ratio() {
        assert_eq!(format_aspect_ratio(1920, 1080), "16:9");
        assert_eq!(format_aspect_ratio(1280, 720), "16:9");
        assert_eq!(format_aspect_ratio(2560, 1440), "16:9");
        assert_eq!(format_aspect_ratio(1280, 800), "16:10");
        assert_eq!(format_aspect_ratio(1920, 1200), "16:10");
        assert_eq!(format_aspect_ratio(800, 600), "4:3");
        assert_eq!(format_aspect_ratio(1024, 768), "4:3");
    }

    #[test]
    fn test_preset_resolution() {
        let (w, h) = AspectRatioPreset::Ratio16x9.to_resolution();
        assert_eq!(w, 1280);
        assert_eq!(h, 720);

        let (w, h) = AspectRatioPreset::Ratio4x3.to_resolution();
        assert_eq!(w, 960);
        assert_eq!(h, 720);

        let (dw, dh) = AspectRatioPreset::Device.to_resolution();
        assert!(dw > 0 && dh > 0);
    }

    #[test]
    fn test_parse_str() {
        assert_eq!(AspectRatioPreset::parse_str("16:9"), Some(AspectRatioPreset::Ratio16x9));
        assert_eq!(AspectRatioPreset::parse_str("16/9"), Some(AspectRatioPreset::Ratio16x9));
        assert_eq!(AspectRatioPreset::parse_str("4:3"), Some(AspectRatioPreset::Ratio4x3));
        assert_eq!(AspectRatioPreset::parse_str("retro"), Some(AspectRatioPreset::Ratio4x3));
        assert_eq!(AspectRatioPreset::parse_str("device"), Some(AspectRatioPreset::Device));
        assert_eq!(AspectRatioPreset::parse_str("unknown_ratio"), None);
    }
}
