use crate::block::StrofaBlock;
use crate::event::Key;
use crate::theme::Theme;

use anyhow::{ anyhow, Result };
use std::collections::HashMap;
use tui::layout::Rect;

pub struct State {
    pub active_block: StrofaBlock,
    pub hovered_block: StrofaBlock,
    pub size: Rect,
    pub theme: Theme,
    pub keys: KeyBindings,
}

impl Default for State {
    fn default() -> Self {
        Self {
            active_block: StrofaBlock::Empty,
            hovered_block: StrofaBlock::Library,
            size: Rect::default(),
            theme: Theme::default(),
            keys: KeyBindings::default()
        }
    }
}

impl KeyBindings {
    pub fn get(&self, k: &str) -> Result<&Key> {
        self.0.get(k).ok_or(anyhow!("key `{}` not found", k))
    }
}

pub struct KeyBindings(HashMap<String, Key>);
impl Default for KeyBindings {
    fn default() -> Self {
        let mut map: HashMap<String, Key> = HashMap::new();

        map.insert("back".to_string(), Key::Char('q'));
        map.insert("next_page".to_string(), Key::Ctrl('d'));
        map.insert("previous_page".to_string(), Key::Ctrl('u'));
        map.insert("jump_to_start".to_string(), Key::Ctrl('a'));
        map.insert("jump_to_end".to_string(), Key::Ctrl('e'));
        map.insert("jump_to_album".to_string(), Key::Char('a'));
        map.insert("jump_to_artist_album".to_string(), Key::Char('A'));
        map.insert("jump_to_context".to_string(), Key::Char('o'));
        map.insert("manage_devices".to_string(), Key::Char('d'));
        map.insert("decrease_volume".to_string(), Key::Char('-'));
        map.insert("increase_volume".to_string(), Key::Char('+'));
        map.insert("toggle_playback".to_string(), Key::Char(' '));
        map.insert("seek_backwards".to_string(), Key::Char('<'));
        map.insert("seek_forwards".to_string(), Key::Char('>'));
        map.insert("next_track".to_string(), Key::Char('n'));
        map.insert("previous_track".to_string(), Key::Char('p'));
        map.insert("help".to_string(), Key::Char('?'));
        map.insert("shuffle".to_string(), Key::Ctrl('s'));
        map.insert("repeat".to_string(), Key::Ctrl('r'));
        map.insert("search".to_string(), Key::Char('/'));
        map.insert("submit".to_string(), Key::Enter);
        map.insert("copy_song_url".to_string(), Key::Char('c'));
        map.insert("copy_album_url".to_string(), Key::Char('C'));
        map.insert("audio_analysis".to_string(), Key::Char('v'));
        map.insert("basic_view".to_string(), Key::Char('B'));
        map.insert("add_item_to_queue".to_string(), Key::Char('z'));

        Self(map)
    }
}