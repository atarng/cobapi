use std::ffi::c_void;

use engage::{
    script::EventScript,
    menu::config::ConfigBasicMenuItem,
};

pub type EventScriptRegistrationCallback = extern "C" fn(&EventScript);
pub type GameSettingRegistrationCallback = extern "C" fn() -> &'static mut ConfigBasicMenuItem;

extern "C" {
    fn cobapi_register_configmenuitem_cb(callback: *const c_void);
    fn cobapi_register_eventscript_cb(callback: EventScriptRegistrationCallback);
}

/// Install a new setting in the "Plugin Settings" sub-menu during gameplay.
/// 
/// Expects a function returning an instance of ConfigBasicMenuItem to be appended to the list of settings.
pub fn install_game_setting(callback: GameSettingRegistrationCallback) {
    unsafe { cobapi_register_configmenuitem_cb(callback as _)}
}

/// Install a new command registerer for lua scripts.
/// 
/// The callback will be provided with the EventScript used by the game to add handlers.
/// Note that both the game and Cobalt's commands are already installed by the time your callback is called.
pub fn install_lua_command_registerer(callback: EventScriptRegistrationCallback) {
    unsafe { cobapi_register_eventscript_cb(callback) }
}