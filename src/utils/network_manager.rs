use std::process::Command;
use std::{fmt, io};
use std::io::Write;

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

// todo merge these checks to a single nmcli query
pub fn connection_exists(ssid: &str) -> bool {
    let output = Command::new("nmcli")
        .args(["-t", "-f", "connection.id", "c", "show", ssid])
        .output()
        .expect("failed to run nmcli");
    output.status.success()
}

pub fn has_saved_password(ssid: &str) -> bool {
    let output = Command::new("nmcli")
        .args(["-t", "-s", "-f", "802-11-wireless-security.psk", "c", "show", ssid])
        .output()
        .expect("failed to run nmcli");

    let stdout = String::from_utf8_lossy(&output.stdout);
    stdout.lines().any(|line| {
        let value = line.split(':').nth(1).unwrap_or("").trim();
        !value.is_empty()
    })
}

// todo fix this this is broken
pub fn is_open_network(ssid: &str) -> bool {
    let output = Command::new("nmcli")
        .args(["-t", "-f", "802-11-wireless-security.key-mgmt", "c", "show", ssid])
        .output()
        .expect("failed to run nmcli");

    let stdout = String::from_utf8_lossy(&output.stdout);
    stdout.lines().any(|line| {
        let value = line.split(':').nth(1).unwrap_or("").trim();
        value.is_empty()
    })
}

fn connection_up(ssid: &str) {
    if !connection_exists(ssid) { return; }
    println!("Attempting to connect to {ssid}...");
    Command::new("nmcli")
        .args(["c", "up", ssid])
        .output()
        .expect("meow");
}

fn connection_down(ssid: &str) {
    println!("Disconnecting from {ssid}...");
    Command::new("nmcli")
        .args(["c", "down", ssid])
        .output()
        .expect("failed to execute process");
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
        .spawn().expect("meow")
        .wait().unwrap()
        .to_string()
        .contains("802-11-wireless-security.psk: property is invalid") // return true if correct to stop looping
}

pub fn handle_wifi_selection(network: String) -> bool {
    if network.contains("simplewifi-exit-select") { return true } // return true to exit selection
    if network.contains("simplewifi-refresh-select") { return false } // return false to reopen

    let meow: Vec<&str> = network
        .splitn(2, ":")
        .collect();

    let ssid = *meow.get(0).unwrap();
    let is_connected = meow.get(1).unwrap().contains("true");

    if is_connected {
        connection_down(ssid)
    } else if connection_exists(ssid) && (has_saved_password(ssid) || is_open_network(ssid)) {
        connection_up(ssid);
    } else {
        loop { if add_connection_password(ssid) { break } } // loop until correct password or exit
        connection_up(ssid);
    }
    false // returns false to reopen selection
}