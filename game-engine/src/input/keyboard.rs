use macroquad::prelude::*;
use std::collections::HashSet;

/// returns whether the given scancode is pressed
pub fn is_pressed(keycode : KeyCode) -> bool {

    macroquad::input::is_key_down(keycode)
    //self.event_pump.keyboard_state().is_scancode_pressed(scancode)
}

/// returns whether the given scancode is released (not pressed)
pub fn is_released(keycode : KeyCode) -> bool {

    !is_pressed(keycode)
}

/// returns whether the given scancode was pressed this frame
/// (will not function if update is not called every frame)
pub fn pressed_this_frame(keycode : KeyCode) -> bool {

    //self.new_state.contains(&scancode) && !self.old_state.contains(&scancode)
    macroquad::input::is_key_pressed(keycode)
}

/// return whether the given scancode was released this frame
/// (will not function if update is not called every frame)
pub fn released_this_frame(keycode : KeyCode) -> bool {

    //!self.new_state.contains(&scancode) && self.old_state.contains(&scancode)
    macroquad::input::is_key_released(keycode)
}

/// gets the current state of the keyboard as a hashset of scancodes
pub fn get_keyboard_state() -> HashSet<KeyCode> {

    //self.event_pump.keyboard_state().pressed_scancodes().collect()

    macroquad::input::get_keys_down()
}