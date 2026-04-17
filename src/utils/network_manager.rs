use std::process::Command;
use std::{fmt, io};
use std::io::Write;
use crate::constants::NMCLI_ERROR;
use crate::utils::utils::split_escaped;

// todo make stuff that can use bssid use it
#[derive(Debug)]
pub struct WifiNetwork {
    pub ssid: String,
    pub signal: u8,
    pub bars: String,
    pub rate: String,
    pub security: String,
    pub active: bool,
    pub bssid: String
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
        .expect(NMCLI_ERROR);
}

pub fn scan_wifi() -> String {
    let meow = Command::new("nmcli")
        .args(["-t", "-f", "SSID,SIGNAL,BARS,RATE,SECURITY,ACTIVE,BSSID", "d", "w", "l"])
        .output()
        .expect(NMCLI_ERROR)
        .stdout;
    String::from_utf8_lossy(&meow).to_string()
}

pub fn wifi_as_vec() -> Vec<WifiNetwork> {
    scan_wifi().lines()
        .filter(|l| !l.is_empty())
        .filter_map(|line| {
            let parts = split_escaped(line);
            Some(WifiNetwork {
                ssid:   parts.get(0)?.to_string(),
                signal:  parts.get(1)?.parse::<u8>().ok()?,
                bars:    parts.get(2)?.to_string(),
                rate:    parts.get(3)?.to_string(),
                security:parts.get(4)?.to_string(),
                active:  parts.get(5)? == "yes",
                bssid: parts.get(6)?.to_string(),
            })
        })
        .collect()
}

// todo merge these checks to a single nmcli query
pub fn connection_exists(ssid: &str) -> bool {
    let output = Command::new("nmcli")
        .args(["-t", "-f", "connection.id", "c", "show", ssid])
        .output()
        .expect(NMCLI_ERROR);
    output.status.success()
}

pub fn has_saved_password(ssid: &str) -> bool {
    let output = Command::new("nmcli")
        .args(["-t", "-s", "-f", "802-11-wireless-security.psk", "c", "show", ssid])
        .output()
        .expect(NMCLI_ERROR);

    let stdout = String::from_utf8_lossy(&output.stdout);
    stdout.lines().any(|line| {
        let value = line.split(':').nth(1).unwrap_or("").trim();
        !value.is_empty()
    })
}

pub fn is_open_network(bssid: &str) -> bool {
    //println!("testing for {bssid}");
    let output = Command::new("nmcli")
        .args(["-t", "-f", "SECURITY", "d", "w", "l", "bssid", bssid])
        .output()
        .expect(NMCLI_ERROR);

    let stdout = String::from_utf8_lossy(&output.stdout);
    stdout.to_string().trim().is_empty()
}

fn connection_up(ssid: &str) {
    if !connection_exists(ssid) { return; }
    println!("Attempting to connect to {ssid}...");
    Command::new("nmcli")
        .args(["c", "up", ssid])
        .output()
        .expect(NMCLI_ERROR);
}

fn connection_down(ssid: &str) {
    println!("Disconnecting from {ssid}...");
    Command::new("nmcli")
        .args(["c", "down", ssid])
        .output()
        .expect(NMCLI_ERROR);
}

fn add_connection_password(ssid: &str) -> bool { // returns correct password bool
    print!("Enter password for {} or cancel: ", ssid);
    io::stdout().flush().unwrap();
    let mut password = String::new();
    io::stdin().read_line(&mut password).unwrap();
    let password = password.trim();
    if password == "cancel" { return true }
    Command::new("nmcli")
        .args(["c", "add", "type", "wifi", "con-name", ssid, "ssid", ssid, "wifi-sec.key-mgmt", "wpa-psk", "wifi-sec.psk", password])
        .spawn().expect(NMCLI_ERROR)
        .wait().unwrap()
        .to_string()
        .contains("802-11-wireless-security.psk: property is invalid") // return true if correct to stop looping
}

fn add_connection_open(ssid: &str) {
    Command::new("nmcli")
        .args(["c", "add", "type", "wifi", "con-name", ssid, "ssid", ssid])
        .output()
        .expect(NMCLI_ERROR);
}

// parses from a network string formatted like <ssid>:<status>:<bssid>
pub fn handle_wifi_selection(network: String) -> bool {
    if network.contains("simplewifi-exit-select") { return true } // return true to exit selection
    if network.contains("simplewifi-refresh-select") { return false } // return false to reopen

    let meow: Vec<&str> = network
        .splitn(3, ":")
        .collect();

    let ssid = *meow.get(0).unwrap();
    let is_connected = meow.get(1).unwrap().contains("true");
    let bssid = *meow.get(2).unwrap();
    let open = is_open_network(bssid);

    if is_connected {
        connection_down(ssid)
    } else if connection_exists(ssid) && (has_saved_password(ssid) || open) {
        connection_up(ssid);
    } else if open {
        add_connection_open(ssid);
        connection_up(ssid)
    } else {
        loop { if add_connection_password(ssid) { break } } // loop until correct password or exit
        connection_up(ssid);
    }
    false // returns false to reopen selection
}