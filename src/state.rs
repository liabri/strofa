use crate::block::{ Blocks, StrofaBlock, MainBlock, TrackKind };
use crate::event::Key;
use crate::theme::Theme;

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
}

impl State {
    pub fn new() -> Self {
        Self {
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

pub struct KeyBindings(pub HashMap<Key, String>);
impl Default for KeyBindings {
    fn default() -> Self {
        let mut map: HashMap<Key, String> = HashMap::new();

        map.insert(Key::Backspace, "back".to_string());
        map.insert(Key::Char('q'), "to_queue".to_string());
        map.insert(Key::Char('e'), "to_playlists".to_string());

        map.insert(Key::Char('d'), "next_page".to_string());
        map.insert(Key::Char('a'), "previous_page".to_string());

        map.insert(Key::Char('v'), "jump_to_start".to_string());
        map.insert(Key::Char('z'), "jump_to_end".to_string());
        map.insert(Key::Char('f'), "jump_to_album".to_string());
        map.insert(Key::Char('c'), "jump_to_artist".to_string());

        map.insert(Key::Char('-'), "decrease_volume".to_string());
        map.insert(Key::Char('+'), "increase_volume".to_string());
        map.insert(Key::Char(' '), "toggle_playback".to_string());
        map.insert(Key::Char('<'), "seek_backwards".to_string());
        map.insert(Key::Char('>'), "seek_forwards".to_string());
        map.insert(Key::Char(']'), "next_track".to_string());
        map.insert(Key::Char('['), "previous_track".to_string());
        map.insert(Key::Char('s'), "shuffle".to_string());
        map.insert(Key::Char('r'), "repeat".to_string());
        map.insert(Key::Char('/'), "search".to_string());
        // map.insert(Key::Enter, "submit".to_string());

        // map.insert("copy_song_name".to_string(), Key::Char('c'));
        // map.insert("copy_album_name".to_string(), Key::Char('C'));
        map.insert(Key::Char('x'), "add_item_to_queue".to_string());

        Self(map)
    }
}