pub mod dbus;
pub mod gamepad;
pub mod keyboard;
pub mod mouse;
pub mod touchscreen;

use zbus::fdo;
use zbus_macros::interface;

/// The [TargetInterface] provides a DBus interface that can be exposed for managing
/// a target input device.
pub struct TargetInterface {
    dev_name: String,
    device_type: String,
}

impl TargetInterface {
    pub fn new(dev_name: String, device_type: String) -> TargetInterface {
        TargetInterface {
            dev_name,
            device_type,
        }
    }
}

impl Default for TargetInterface {
    fn default() -> Self {
        Self::new("Gamepad".to_string(), "gamepad".to_string())
    }
}

#[interface(name = "org.shadowblip.Input.Target")]
impl TargetInterface {
    /// Name of the DBus device
    #[zbus(property)]
    async fn name(&self) -> fdo::Result<String> {
        Ok(self.dev_name.clone())
    }

    #[zbus(property)]
    async fn device_type(&self) -> fdo::Result<String> {
        Ok(self.device_type.clone())
    }
}
