use std::sync::{LazyLock, Mutex, MutexGuard};
use crate::utils::network_manager::WifiNetwork;

static NETWORK_CACHE: LazyLock<Mutex<Vec<WifiNetwork>>> = LazyLock::new(|| Mutex::new(Vec::new()));
pub fn network_cache() -> MutexGuard<'static, Vec<WifiNetwork>> { NETWORK_CACHE.lock().unwrap() }

pub const NMCLI_ERROR: &str = "Failed to run nmcli, consider installing it in your package manager of choice";
pub const ABOUT: &str = "--- SimpleWifi-rs ---
Quick rust CLI tool designed to be a simpler and prettier alternative to nmtui.
The point is not to make a powerful complex network manager, but simply to give a simple convenient alternative \
to nmcli/nmtui for daily use.
This binary is essentially a rust wrapper with a simple tui over nmcli, to make its functions easier to \
use as a daily wifi manager.

Refer to https://github.com/catboylei/simpleWifi-rs for more info :3
--- SimpleWifi-rs ---";
pub const LABEL: &str = "    SSID                           BARS  SIGNAL    RATE      SECURITY";