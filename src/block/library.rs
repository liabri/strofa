use super::{ Blokka, State, Render, Index, get_color, selectable_list };
use mpd_client::{ Client, commands, commands::responses::Playlist };
use tui::{ 
    Frame,
    backend::Backend, 
    layout::{ Rect }, 
    text::Span, 
    widgets::ListItem
};

pub struct Library {
   pub entries: [&'static str; 5],
   pub index: Index 
}

impl Library {
    pub async fn new() -> Self {
        Self {
            entries: [
                "Queue",
                "Tracks",
                "Albums",
                "Artists",
                "Podcasts"
            ],
            index: Index::new(5),
        }        
    }
}

impl<B: Backend> Render<B> for Library {
    fn render(&self, f: &mut Frame<B>, state: &State<B>, layout_chunk: Rect) {
        let highlight_state = (
            state.blocks.is_active(Blokka::Library),
            state.blocks.is_hovered(Blokka::Library)
        );

        let items: Vec<ListItem> = self.entries
            .into_iter()
            .map(|i| ListItem::new(Span::raw(i)))
            .collect();

        selectable_list(
            f,
            state,
            layout_chunk,
            " Library ",
            items,
            highlight_state,
            Some(self.index.inner)
        );
    }    
}

use crate::block::{ MainBlock, Podcasts, Artists, Albums, AlbumKind, Tracks, TrackKind, Queue };
use crate::event::Key;
impl Library {
    pub async fn active_key_event<B>(state: &mut State<B>, key: Key) where B: Backend {
        match key {
            Key::Up => state.blocks.library.index.dec(),
            Key::Down => state.blocks.library.index.inc(),
            Key::Enter => {
                let index = state.blocks.library.index.inner;
                let main_block = match state.blocks.library.entries[index] {
                    "Queue" => MainBlock::Queue(Queue::new(&state.client).await.unwrap()),
                    "Tracks" => MainBlock::Tracks(Tracks::new(TrackKind::All, &state.client).await.unwrap()),
                    "Albums" => MainBlock::Albums(Albums::new(AlbumKind::All).await),
                    "Artists" => MainBlock::Artists(Artists::new().await),
                    "Podcasts" => MainBlock::Podcasts(Podcasts::new().await),
                    _ => panic!("view not found"),
                };

                state.blocks.set_main(main_block);
            }
            _ => {},
        }
    }

    pub async fn hovered_key_event<B>(state: &mut State<B>, key: Key) where B: Backend {
        match key {
            Key::Up => state.blocks.set_hover(&Blokka::Search),
            Key::Down => state.blocks.set_hover(&Blokka::Playlists),
            Key::Right => state.blocks.set_hover(&Blokka::Main),
            _ => {},
        }
    }
}