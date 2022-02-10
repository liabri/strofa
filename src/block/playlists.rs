use super::{ BlockKind, State, Render, Index, get_color, selectable_list };
use mpd_client::{ Client, commands, commands::responses::Playlist };
use anyhow::Result;
use tui::{ 
    Frame,
    backend::Backend, 
    layout::Rect, 
    text::Span, 
    widgets::ListItem
};

pub struct Playlists {
   pub entries: Vec<Playlist>,
   // pub index: Index 
}

// impl Playlists {
//     pub async fn new(client: &Client) -> Result<Self> {
//         Ok(Self {
//             entries: client.command(commands::GetPlaylists).await?,
//             index: Index::new(50),
//         })      
//     }
// }

// impl<B: Backend> Render<B> for Playlists {
//     fn render(&self, f: &mut Frame<B>, state: &State<B>, layout_chunk: Rect) {
//         let highlight_state = (
//             state.blocks.is_active(BlockKind::Playlists),
//             state.blocks.is_hovered(BlockKind::Playlists)
//         );

//         let items: Vec<ListItem> = self.entries
//             .iter()
//             .map(|i| ListItem::new(Span::from(i.name.as_str())))
//             .collect();

//         selectable_list(
//             f,
//             state,
//             layout_chunk,
//             " Playlists ",
//             items,
//             highlight_state,
//             Some(self.index.inner)
//         );
//     }    
// }

// use crate::block::{ Tracks, TrackKind, MainBlock };
// use crate::event::Key;
// impl Playlists {
//     pub async fn active_key_event<B>(state: &mut State<B>, key: Key) where B: Backend {
//         match key {
//             Key::Up => state.blocks.playlists.index.dec(),
//             Key::Down => state.blocks.playlists.index.inc(),   
//             Key::Enter => {
//                 let index = state.blocks.playlists.index.inner;  
//                 let name = state.blocks.playlists.entries.get(0).unwrap().name.to_string();
//                 state.blocks.set_main(MainBlock::Tracks(Tracks::new(TrackKind::Playlist(name), &state.client).await.unwrap()));
//             },

//             _ => {}
//         }  
//     }

//     pub async fn hovered_key_event<B>(state: &mut State<B>, key: Key) where B: Backend {
//         match key {
//             Key::Up => state.blocks.set_hover(&BlockKind::Library),
//             Key::Right => state.blocks.set_hover(&BlockKind::Main),
//             _ => {},
//         }
//     }
// }



//---------------------------
use crate::event::Key;
use crate::block::IndexedBlock;
use crate::block::BlockTrait;
use async_trait::async_trait;

impl IndexedBlock<Playlists> {
    pub async fn new(client: &Client) -> Result<Self> {
        Ok(Self {
            index: Index::new(50),
            inner: Playlists { entries: client.command(commands::GetPlaylists).await? }  
        })
    }
}

#[async_trait]
impl<B: 'static + Send + Backend> BlockTrait<B> for IndexedBlock<Playlists> {
    async fn active_event(state: &mut State<B>, key: Key) {
        match key {
            // Key::Up => state.blocks.playlists.index.dec(),
            // Key::Down => state.blocks.playlists.index.inc(),   
            // Key::Enter => {
            //     let index = state.blocks.playlists.index.inner;  
            //     let name = state.blocks.playlists.entries.get(0).unwrap().name.to_string();
            //     state.blocks.set_main(MainBlock::Tracks(Tracks::new(TrackKind::Playlist(name), &state.client).await.unwrap()));
            // },

            _ => {}
        }  
    }

    async fn hovered_event(state: &mut State<B>, key: Key) {
        match key {
            Key::Up => state.blocks.set_hover(BlockKind::Library),
            // Key::Right => state.blocks.set_hover(&BlockKind::Main),
            _ => {}
        }
    }
}

impl<B: Backend> Render<B> for IndexedBlock<Playlists> {
    fn render(&self, f: &mut Frame<B>, state: &State<B>, layout_chunk: Rect) {
        let highlight_state = (
            state.blocks.is_active(BlockKind::Playlists),
            state.blocks.is_hovered(BlockKind::Playlists)
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