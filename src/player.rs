use crate::pieces::Color;

#[derive(Clone)]
pub struct Player {
    // The player's username.
    pub username: String,

    // Is the player a bot?
    pub is_bot: bool,

    // The player's color.
    pub color: Color,

    // The player's timer. Represented in seconds.
    pub timer: u16
}

impl Player {
    pub fn new(username: Option<String>, color: Color, timer_minutes: u16) -> Self {
        let name = username.clone().unwrap_or("Bot".to_string());

        Player {
            username: name,
            is_bot: username.is_none(),
            color,
            timer: timer_minutes * 60
        }
    }
}