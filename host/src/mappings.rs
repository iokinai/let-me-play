use crate::error::LmpError;
use enigo::Enigo;

#[cfg(target_os = "linux")]
pub fn char_to_keycode(c: char) -> Option<u16> {
    let evdev_code = match c.to_ascii_lowercase() {
        '1' => 2,
        '2' => 3,
        '3' => 4,
        '4' => 5,
        '5' => 6,
        '6' => 7,
        '7' => 8,
        '8' => 9,
        '9' => 10,
        '0' => 11,
        'q' => 16,
        'w' => 17,
        'e' => 18,
        'r' => 19,
        't' => 20,
        'y' => 21,
        'u' => 22,
        'i' => 23,
        'o' => 24,
        'p' => 25,
        'a' => 30,
        's' => 31,
        'd' => 32,
        'f' => 33,
        'g' => 34,
        'h' => 35,
        'j' => 36,
        'k' => 37,
        'l' => 38,
        'z' => 44,
        'x' => 45,
        'c' => 46,
        'v' => 47,
        'b' => 48,
        'n' => 49,
        'm' => 50,
        ' ' => 57,
        '\n' => 28,
        _ => return None,
    };

    Some(evdev_code + 8)
}

#[cfg(target_os = "linux")]
pub fn send_input(enigo: &mut Enigo, input: char) -> Result<(), LmpError> {
    use enigo::{Direction, Keyboard};

    use crate::error::{HandlerError, LmpError};

    let code =
        char_to_keycode(input).ok_or(LmpError::HandlerError(HandlerError::KeyIsNotSupported))?;

    enigo
        .raw(code, Direction::Click)
        .map_err(|_| LmpError::HandlerError(HandlerError::CouldNotSendInput))?;

    Ok(())
}

#[cfg(target_os = "windows")]
pub fn send_input(enigo: &mut Enigo, input: char) -> Result<(), LmpError> {
    use enigo::{Direction, Key, Keyboard};

    use crate::error::{HandlerError, LmpError};

    enigo
        .key(Key::Unicode(input), Direction::Click)
        .map_err(|_| LmpError::HandlerError(HandlerError::CouldNotSendInput))?;

    Ok(())
}
