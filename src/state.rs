use crate::block::{ Blocks, StrofaBlock, MainBlock, TrackKind };
use crate::event::Key;
use crate::theme::Theme;

use anyhow::{ anyhow, Result };
use std::collections::{ VecDeque, HashMap };
use tui::layout::Rect;

pub struct State {
    pub blocks: Blocks,
    pub active_block: StrofaBlock,
    pub hovered_block: StrofaBlock,
    pub hover_history: VecDeque<StrofaBlock>, // Helps make tui controls more fluid by having memory
    pub main_block: MainBlock,
    pub size: Rect,
    pub theme: Theme,
    pub keys: KeyBindings,
    pub client: mpd_client::Client,
}

impl State {
    pub fn new(client: mpd_client::Client) -> Self {
        Self {
            client,
            blocks: Blocks::default(),
            active_block: StrofaBlock::Empty,
            hovered_block: StrofaBlock::Library,
            hover_history: VecDeque::with_capacity(5),
            main_block: MainBlock::Tracks(TrackKind::Queue),
            size: Rect::default(),
            theme: Theme::default(),
            keys: KeyBindings::default(),
        }
    }

    pub fn set_hover(&mut self, blk: &StrofaBlock) {
        self.hover_history.truncate(5);
        self.hover_history.push_front(self.hovered_block.clone());
        self.hovered_block = blk.clone();
    }

    pub fn set_active(&mut self, blk: StrofaBlock) { 
        self.hovered_block = blk.clone();
        self.active_block = blk.clone();
    }
}

// impl KeyBindings {
//     pub fn get(&self, k: &str) -> Result<&Key> {
//         self.0.get(k).ok_or(anyhow!("key `{}` not found", k))
//     }
// }

pub struct KeyBindings(pub HashMap<Key, String>);
impl Default for KeyBindings {
    fn default() -> Self {
        let mut map: HashMap<Key, String> = HashMap::new();

        map.insert(Key::Backspace, "back".to_string());
        map.insert(Key::Char('q'), "to_queue".to_string());
        map.insert(Key::Char('e'), "to_playlists".to_string());

        map.insert(Key::Char('d'), "next_page".to_string());
        map.insert(Key::Char('a'), "previous_page".to_string());

        // map.insert("jump_to_start".to_string(), Key::Ctrl('f'));
        // map.insert("jump_to_end".to_string(), Key::Ctrl('g'));
        // map.insert("jump_to_album".to_string(), Key::Char('t'));
        // map.insert("jump_to_artist_album".to_string(), Key::Char('y'));
        // map.insert("jump_to_context".to_string(), Key::Char('o'));

        // map.insert("decrease_volume".to_string(), Key::Char('-'));
        // map.insert("increase_volume".to_string(), Key::Char('+'));
        // map.insert("toggle_playback".to_string(), Key::Char(' '));
        // map.insert("seek_backwards".to_string(), Key::Char('<'));
        // map.insert("seek_forwards".to_string(), Key::Char('>'));
        // map.insert("next_track".to_string(), Key::Char(']'));
        // map.insert("previous_track".to_string(), Key::Char('['));
        // map.insert("shuffle".to_string(), Key::Ctrl('s'));
        // map.insert("repeat".to_string(), Key::Ctrl('r'));
        map.insert(Key::Char('/'), "search".to_string());
        // map.insert("submit".to_string(), Key::Enter);

        // map.insert("copy_song_name".to_string(), Key::Char('c'));
        // map.insert("copy_album_name".to_string(), Key::Char('C'));
        // map.insert("basic_view".to_string(), Key::Char('B'));
        // map.insert("add_item_to_queue".to_string(), Key::Char('x'));

        Self(map)
    }
}