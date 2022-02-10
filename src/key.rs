use crate::block::{ BlockKind, Playlists };//, MainBlock, Library, Queue, SelectableList, Playlists, Search, Sort, Playbar };
use crate::chunk::Chunks;
use crate::event::Key;
use crate::theme::Theme;
use crate::client::StrofaClient;
use crate::state::State;

use tui::backend::Backend;
use anyhow::Result;
use std::collections::{ VecDeque, HashMap };
use tui::layout::Rect;
use mpd_client::Client;


//maybe some gentlemens rule where Ctrl(X) = in the current active window, Char(X) = in the whole app
//or the other way 'round
pub struct KeyBindings(pub HashMap<Key, String>);
impl Default for KeyBindings {
    fn default() -> Self {
        let mut map: HashMap<Key, String> = HashMap::new();

        map.insert(Key::Backspace, "back".to_string());
        map.insert(Key::Char('q'), "to_queue".to_string());
        map.insert(Key::Char('e'), "to_playlists".to_string());
        map.insert(Key::Ctrl('b'), "toggle_top".to_string());

        map.insert(Key::Char('v'), "jump_to_start".to_string());
        map.insert(Key::Char('z'), "jump_to_end".to_string());
        map.insert(Key::Char('f'), "jump_to_album".to_string());
        map.insert(Key::Char('c'), "jump_to_artist".to_string());

        map.insert(Key::Char('-'), "decrease_volume".to_string());
        map.insert(Key::Char('+'), "increase_volume".to_string());
        // map.insert(Key::Shift('-'), "decrease_volume_big".to_string());
        // map.insert(Key::Shift('+'), "increase_volume_big".to_string());

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

impl KeyBindings {
    pub async fn execute<B>(state: &mut State<B>, cmd: &str) -> Result<()> where B: 'static + Backend {
        match cmd {
            // binds manipulating ui
            // "to_queue" => state.blocks.set_main(MainBlock::Queue(Queue::new(&state.client).await?)),
            // "toggle_top" => self.blocks
            "to_playlists" => state.blocks.set_active(BlockKind::Playlists),
            "search" => state.blocks.set_active(BlockKind::Search),
          
            // binds manipulating mpd  
            "toggle_playback" => state.client.toggle_playback().await?,
            "decrease_volume" => state.client.set_volume(-5).await?,
            "increase_volume" => state.client.set_volume(5).await?,
            "decrease_volume_big" => state.client.set_volume(-10).await?,
            "increase_volume_big" => state.client.set_volume(10).await?,
            "next_track" => state.client.next_track().await?,
            "previous_track" => state.client.previous_track().await?,
            "seek_forwards" => state.client.seek_forwards(10).await?,
            "seek_backwards" => state.client.seek_backwards(10).await?,
            "shuffle" => state.client.toggle_shuffle().await?,
            "repeat" => state.client.toggle_repeat().await?,
            // "jump_to_start" => self.blocks.playbar.jump_to_start(self.client.clone()).await,

            _ => {},
        }

        Ok(()) 
    }
}