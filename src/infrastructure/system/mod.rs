#[cfg(feature = "system-integration")]
pub mod keyboard_monitor;
#[cfg(feature = "system-integration")]
pub mod clipboard_manager;
#[cfg(feature = "system-integration")]
pub mod input_simulator;

#[cfg(not(feature = "system-integration"))]
pub mod stub;

#[cfg(feature = "system-integration")]
pub use keyboard_monitor::*;
#[cfg(feature = "system-integration")]
pub use clipboard_manager::*;
#[cfg(feature = "system-integration")]
pub use input_simulator::*;

#[cfg(not(feature = "system-integration"))]
pub use stub::*;