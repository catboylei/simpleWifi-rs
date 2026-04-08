use std::process::Command;
use std::fmt;

#[derive(Debug)]
pub struct WifiNetwork {
    pub ssid: String,
    pub signal: u8,
    pub bars: String,
    pub rate: String,
    pub security: String,
    pub active: bool
}

impl fmt::Display for WifiNetwork {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {:<30} {:<4} {:>4}%   {:<12} {:<10}",
            if self.active { "✓" } else { " " },
            self.ssid,
            self.bars,
            self.signal,
            self.rate,
            self.security,
        )
    }
}

// nmcli d w l -r yes
pub fn scan_wifi() -> String {
    let meow = Command::new("nmcli")
        .args(["-t", "-f", "SSID,SIGNAL,BARS,RATE,SECURITY,ACTIVE", "d", "w", "l", "-r", "yes"])
        .output()
        .expect("failed to execute process")
        .stdout;
    String::from_utf8_lossy(&meow).to_string()
}

pub fn wifi_as_vec() -> Vec<WifiNetwork> {
    scan_wifi().lines()
        .filter(|l| !l.is_empty())
        .filter_map(|line| {
            let mut parts = line.splitn(6, ':');
            Some(WifiNetwork {
                ssid:     parts.next()?.to_string(),
                signal:   parts.next()?.parse::<u8>().ok()?,
                bars:     parts.next()?.to_string(),
                rate:     parts.next()?.to_string(),
                security: parts.next()?.to_string(),
                active: if parts.next()?.to_string().contains("yes") { true } else  { false }
            })
        })
        .collect()
}