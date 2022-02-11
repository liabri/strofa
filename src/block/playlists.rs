use super::{ IndexedBlock, BlockTrait, State, Render, Index, get_color, selectable_list };
use mpd_client::{ Client, commands, commands::responses::Playlist };
use crate::event::Key;
use async_trait::async_trait;
use anyhow::Result;
use crate::chunk::BlockKind;
use tui::{ 
    Frame,
    backend::Backend, 
    layout::Rect, 
    text::Span, 
    widgets::ListItem
};

pub struct Playlists {
   pub entries: Vec<Playlist>
}

impl IndexedBlock<Playlists> {
    pub async fn new(client: &Client) -> Result<Self> {
        Ok(Self {
            index: Index::new(50),
            inner: Playlists { entries: client.command(commands::GetPlaylists).await? }  
        })
    }
}

#[async_trait]
impl BlockTrait for IndexedBlock<Playlists> {
    async fn active_event(state: &mut State, key: Key) {
        match key {
            // Key::Up => state.chunks.playlists.index.dec(),
            // Key::Down => state.chunks.playlists.index.inc(),   
            // Key::Enter => {
            //     let index = state.chunks.playlists.index.inner;  
            //     let name = state.chunks.playlists.entries.get(0).unwrap().name.to_string();
            //     state.chunks.set_main(MainBlock::Tracks(Tracks::new(TrackKind::Playlist(name), &state.client).await.unwrap()));
            // },

            _ => {}
        }  
    }

    async fn hovered_event(state: &mut State, key: Key) {
        match key {
            Key::Up => state.chunks.set_hover(BlockKind::LeftTop),
            // Key::Right => state.chunks.set_hover(&BlockKind::Main),
            _ => {}
        }
    }
}

impl<B: Backend + Send> Render<B> for IndexedBlock<Playlists> {
    fn render(&self, f: &mut Frame<B>, state: &State, layout_chunk: Rect) {
        let highlight_state = (
            state.chunks.is_active(BlockKind::LeftBottom),
            state.chunks.is_hovered(BlockKind::LeftBottom)
        );

        let items: Vec<ListItem> = self.inner.entries
            .iter()
            .map(|i| ListItem::new(Span::from(i.name.as_str())))
            .collect();

        selectable_list(
            f,
            state,
            layout_chunk,
            " Playlists ",
            items,
            highlight_state,
            Some(self.index.inner)
        );
    }    
}