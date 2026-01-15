use macroquad::audio::{self, Sound};
use quad_snd::PlaySoundParams;

use super::constants::VOLUME;

pub struct Sounds {
    pub click: Sound,
    pub win: Sound,
    pub loss: Sound,
    pub button: Sound,
    pub mode: Sound,
}

impl Sounds {
    pub fn play(sound: &Sound) {
        audio::play_sound(
            sound,
            PlaySoundParams {
                looped: false,
                volume: VOLUME,
            },
        );
    }
}
