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
use crate::block::{ BlockTrait, IndexedBlock };
use crate::chunk::BlockKind;
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
impl BlockTrait for IndexedBlock<Library> {
    async fn active_event(state: &mut State, key: Key) {
        match key {
            Key::Up => state.chunks.centre.inner.left_chunk.inner.top.index.dec(),
            Key::Down => state.chunks.centre.inner.left_chunk.inner.top.index.inc(),
            // Key::Enter => {
            //     let index = state.chunks.library.index.inner;
            //     let main_block = match state.chunks.library.entries[index] {
            //         "Queue" => MainBlock::Queue(Queue::new(&state.client).await.unwrap()),
            //         "Tracks" => MainBlock::Tracks(Tracks::new(TrackKind::All, &state.client).await.unwrap()),
            //         "Albums" => MainBlock::Albums(Albums::new(AlbumKind::All).await),
            //         "Artists" => MainBlock::Artists(Artists::new().await),
            //         "Podcasts" => MainBlock::Podcasts(Podcasts::new().await),
            //         _ => panic!("view not found"),
            //     };

            //     state.chunks.set_main(main_block);
            // }
            _ => {},
        }
    }

    async fn hovered_event(state: &mut State, key: Key) {}
}

impl<B: Backend + Send> Render<B> for IndexedBlock<Library> {
    fn render(&self, f: &mut Frame<B>, state: &State, layout_chunk: Rect) {
        let highlight_state = (
            state.chunks.is_active(BlockKind::LeftTop),
            state.chunks.is_hovered(BlockKind::LeftTop)
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