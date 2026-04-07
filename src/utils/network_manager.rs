use std::process::Command;
// nmcli d w l --r yes | head -50
pub fn scan_wifi() {
    let meow = Command::new("nmcli")
        .args(["-f", "IN-USE,SSID,SIGNAL,BARS", "d", "w", "l", "-r", "yes"])
        .output()
        .expect("failed to execute process")
        .stdout;
    println!("{}", String::from_utf8_lossy(&meow))
}

pub fn list_wifi() -> Vec<String> {
    let meow = Command::new("nmcli")
        .args(["-f", "IN-USE,SSID,SIGNAL,BARS", "d", "w", "l"])
        .output()
        .expect("failed to execute process")
        .stdout;
    String::from_utf8_lossy(&meow)
        .into_owned()
        .split("\n")
        .map(|e| { e.to_owned() })
        .collect()
}