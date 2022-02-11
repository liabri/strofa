use super::{ Index, State, Render, get_color, selectable_list };
use mpd_client::{ Client, commands, commands::responses::Playlist };
use tui::{ 
    Frame,
    backend::Backend, 
    layout::{ Rect }, 
    text::Span, 
    widgets::ListItem
};

pub struct Library {
   pub entries: [&'static str; 5]
}

use crate::event::Key;
use crate::block::{ BlockTrait, BlockKind, IndexedBlock };
use async_trait::async_trait;
use anyhow::Result;

impl IndexedBlock<Library> {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            index: Index::new(5),
            inner: Library { 
                entries: [
                    "Queue",
                    "Tracks",
                    "Albums",
                    "Artists",
                    "Podcasts"
                ],
            }  
        })
    }
}

#[async_trait]
impl<B: Send + Backend> BlockTrait<B> for IndexedBlock<Library> {
    async fn active_event(state: &mut State<B>, key: Key) {
        match key {
            Key::Up => state.chunks.centre.inner.left_chunk.inner.library.index.dec(),
            Key::Down => state.chunks.centre.inner.left_chunk.inner.library.index.inc(),
            // Key::Enter => {
            //     let index = state.blocks.library.index.inner;
            //     let main_block = match state.blocks.library.entries[index] {
            //         "Queue" => MainBlock::Queue(Queue::new(&state.client).await.unwrap()),
            //         "Tracks" => MainBlock::Tracks(Tracks::new(TrackKind::All, &state.client).await.unwrap()),
            //         "Albums" => MainBlock::Albums(Albums::new(AlbumKind::All).await),
            //         "Artists" => MainBlock::Artists(Artists::new().await),
            //         "Podcasts" => MainBlock::Podcasts(Podcasts::new().await),
            //         _ => panic!("view not found"),
            //     };

            //     state.blocks.set_main(main_block);
            // }
            _ => {},
        }
    }

    async fn hovered_event(state: &mut State<B>, key: Key) {
        match key {
            // Key::Up => state.blocks.set_hover(&BlockKind::Search),
            Key::Down => state.blocks.set_hover(BlockKind::Playlists),
            // Key::Right => state.blocks.set_hover(&BlockKind::Main),
            _ => {},
        }
    }
}

impl<B: Backend + Send> Render<B> for IndexedBlock<Library> {
    fn render(&self, f: &mut Frame<B>, state: &State<B>, layout_chunk: Rect) {
        let highlight_state = (
            state.blocks.is_active(BlockKind::Library),
            state.blocks.is_hovered(BlockKind::Library)
        );

        let items: Vec<ListItem> = self.inner.entries
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