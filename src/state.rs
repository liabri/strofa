// use crate::block::{ Blocks, BlockKind, MainBlock, Library, Queue, SelectableList, Playlists, Search, Sort, Playbar };
use crate::block::{ Blocks, BlockKind, BlockTrait, IndexedBlock, Playlists, Library };
use crate::chunk::Chunks;
use crate::event::Key;
use crate::theme::Theme;
use crate::client::StrofaClient;
use crate::key::KeyBindings;

use tui::backend::Backend;
use anyhow::Result;
use tui::layout::Rect;
use mpd_client::Client;

pub struct State<B> {
    pub chunks: Chunks<B>,
    pub blocks: Blocks<B>,
    pub size: Rect,
    pub theme: Theme,
    pub keys: KeyBindings,
    pub client: Client
}

impl<B: 'static + Send + Backend> State<B> {
    pub async fn new(client: Client) -> Result<Self> {
        Ok(Self {
            chunks: Chunks::<B>::new().await?,
            blocks: Blocks::new(&client).await?,
            size: Rect::default(),
            theme: Theme::default(),
            keys: KeyBindings::default(),
            client,
        })
    }

    // new blocks are only made here !!
    pub async fn active_event(&mut self, key: Key) {
        // ideal: self.blocks.active.active_key_event();

        match self.blocks.active {
            // Some(BlockKind::Search) => Search::active_key_event(self, key).await,
            // Some(BlockKind::Sort) => Sort::active_key_event(self, key).await,
            Some(BlockKind::Library) => IndexedBlock::<Library>::active_event(self, key).await,
            Some(BlockKind::Playlists) => IndexedBlock::<Playlists>::active_event(self, key).await,

        //     Some(BlockKind::Main) => { 
        //         match key {
        //             Key::Up => self.blocks.main.index().dec(),
        //             Key::Down => self.blocks.main.index().inc(),
        //             _ => {}
        //         }

        //         match &self.blocks.main {
        //             MainBlock::Tracks(x) => {
        //                 match key {
        //                     Key::Enter => x.play(&self.client, x.index.inner).await,
        //                     // Key::Char('A') => self.client.add_song_to_playlist(x.songs.get(x.index.inner).unwrap()).await
        //                     _ => {}
        //                 }
        //             },

        //             MainBlock::Queue(x) => {
        //                 // x.active_key_event(self, key);
        //                 // match key {
        //                 //     Key::Enter => x.play(self.client.clone(), x.index.inner).await,
        //                 //     Key::Char('c') => self.client.clear_queue().await.unwrap(),
        //                 //     // Key::Char('p') => self.client.proritise_song_in_queue(x.index.inner)
        //                 //     // Key::Char('w') => self.client.move_song_up_in_queue(x.songs.get(x.index.inner).unwrap()).await
        //                 //     // Key::Char('s') => self.client.move_song_down_in_queue(x.songs.get(x.index.inner).unwrap()).await
        //                 //     // Key::Char('A') => self.client.add_song_to_playlist(x.songs.get(x.index.inner).unwrap()).await
        //                 //     // Key::Char('o') => x.jump_to_current_song().await
        //                 //     _ => {}
        //                 // }
        //             },

        //             _ => {}
        //         }
        //     },

            _ => {}
        }
    }

    pub async fn hovered_event(&mut self, key: Key) {
        match self.blocks.hovered {
            // BlockKind::Search => Search::hovered_key_event(self, key).await,
            // BlockKind::Sort => Sort::hovered_key_event(self, key).await,
            BlockKind::Library => IndexedBlock::<Library>::hovered_event(self, key).await,
            BlockKind::Playlists => IndexedBlock::<Playlists>::hovered_event(self, key).await,

            // BlockKind::Main => {
            //     match key {
            //         Key::Up => self.blocks.set_hover(&BlockKind::Search),
            //         Key::Left => {
            //             for previous in self.blocks.hover_history.clone().into_iter() {
            //                 if previous==BlockKind::Library || previous==BlockKind::Playlists {
            //                     self.blocks.set_hover(&previous);
            //                     return;
            //                 }
            //             }

            //             self.blocks.set_hover(&BlockKind::Library)
            //         },

            //         Key::Right => self.blocks.set_hover(&BlockKind::Sort),
            //         Key::Down => {
            //             self.blocks.set_active(BlockKind::Main);
            //             self.blocks.main.index().inc();
            //         },

            //         _ => {},
            //     }
            // },

            _ => {}   
        }

        // common behaviour
        match key {
            Key::Enter => self.blocks.set_active(self.blocks.hovered),
            _ => {}
        }
    }      
}