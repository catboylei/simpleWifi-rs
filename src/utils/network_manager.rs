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

pub fn rescan_wifi() {
    Command::new("nmcli")
        .args(["d", "w", "l", "-r", "yes"])
        .output()
        .expect("failed to execute process");
}

pub fn scan_wifi() -> String {
    let meow = Command::new("nmcli")
        .args(["-t", "-f", "SSID,SIGNAL,BARS,RATE,SECURITY,ACTIVE", "d", "w", "l"])
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
                active: if parts.next()?.to_string().contains("yes") { true } else { false }
            })
        })
        .collect()
}

// todo handle passwords somehow
pub fn handle_wifi_selection(network: String) {
    let meow: Vec<&str> = network
        .splitn(2, ":")
        .collect();

    let ssid = meow.get(0).unwrap();
    let cmd = if meow.get(1).unwrap().contains("false") { "up" } else { "down" };

    println!("{}", network);
    println!("applying {} to {}", cmd, ssid);

    Command::new("nmcli")
        .args(["c", cmd, ssid, "--ask"])
        .output()
        .expect("failed to execute process");
}