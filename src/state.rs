use crate::block::{ Blokka, MainBlock, Library, Queue, SelectableList, Playlists, Search, Sort, Playbar };
use crate::chunk::Chunks;
use crate::event::Key;
use crate::theme::Theme;
use crate::client::StrofaClient;
use crate::key::KeyBindings;

use tui::backend::Backend;
use anyhow::Result;
use std::collections::{ VecDeque, HashMap };
use tui::layout::Rect;
use mpd_client::Client;

pub struct State<B> {
    pub chunks: Chunks<B>,
    pub blocks: Blocks,
    pub size: Rect,
    pub theme: Theme,
    pub keys: KeyBindings,
    pub client: Client
}

impl<B: 'static + Backend> State<B> {
    pub async fn new(client: Client) -> Result<Self> {
        Ok(Self {
            chunks: Chunks::<B>::new().await?,
            blocks: Blocks::new(client.clone()).await?,
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
            Some(Blokka::Search) => Search::active_key_event(self, key).await,
            Some(Blokka::Sort) => Sort::active_key_event(self, key).await,
            Some(Blokka::Library) => Library::active_key_event(self, key).await,
            Some(Blokka::Playlists) => Playlists::active_key_event(self, key).await,

            Some(Blokka::Main) => { 
                match key {
                    Key::Up => self.blocks.main.index().dec(),
                    Key::Down => self.blocks.main.index().inc(),
                    _ => {}
                }

                match &self.blocks.main {
                    MainBlock::Tracks(x) => {
                        match key {
                            Key::Enter => x.play(&self.client, x.index.inner).await,
                            // Key::Char('A') => self.client.add_song_to_playlist(x.songs.get(x.index.inner).unwrap()).await
                            _ => {}
                        }
                    },

                    MainBlock::Queue(x) => {
                        // x.active_key_event(self, key);
                        // match key {
                        //     Key::Enter => x.play(self.client.clone(), x.index.inner).await,
                        //     Key::Char('c') => self.client.clear_queue().await.unwrap(),
                        //     // Key::Char('p') => self.client.proritise_song_in_queue(x.index.inner)
                        //     // Key::Char('w') => self.client.move_song_up_in_queue(x.songs.get(x.index.inner).unwrap()).await
                        //     // Key::Char('s') => self.client.move_song_down_in_queue(x.songs.get(x.index.inner).unwrap()).await
                        //     // Key::Char('A') => self.client.add_song_to_playlist(x.songs.get(x.index.inner).unwrap()).await
                        //     // Key::Char('o') => x.jump_to_current_song().await
                        //     _ => {}
                        // }
                    },

                    _ => {}
                }
            },

            _ => {}
        }
    }

    pub async fn hovered_event(&mut self, key: Key) {
        match self.blocks.hovered {
            Blokka::Search => Search::hovered_key_event(self, key).await,
            Blokka::Sort => Sort::hovered_key_event(self, key).await,
            Blokka::Library => Library::hovered_key_event(self, key).await,
            Blokka::Playlists => Playlists::hovered_key_event(self, key).await,

            Blokka::Main => {
                match key {
                    Key::Up => self.blocks.set_hover(&Blokka::Search),
                    Key::Left => {
                        for previous in self.blocks.hover_history.clone().into_iter() {
                            if previous==Blokka::Library || previous==Blokka::Playlists {
                                self.blocks.set_hover(&previous);
                                return;
                            }
                        }

                        self.blocks.set_hover(&Blokka::Library)
                    },

                    Key::Right => self.blocks.set_hover(&Blokka::Sort),
                    Key::Down => {
                        self.blocks.set_active(Blokka::Main);
                        self.blocks.main.index().inc();
                    },

                    _ => {},
                }
            },

            _ => {}   
        }

        // common behaviour
        match key {
            Key::Enter => self.blocks.set_active(self.blocks.hovered),
            _ => {}
        }
    }      
}

pub struct Blocks {    
    pub search: Search,
    pub sort: Sort,
    pub library: Library,
    pub playlists: Playlists,
    pub playbar: Playbar,
    pub main: MainBlock,
    pub active: Option<Blokka>,
    pub hovered: Blokka,
    pub hover_history: VecDeque<Blokka>,
}

impl Blocks {
    pub async fn new(client: Client) -> Result<Self> {
        Ok(Self {
            search: Search::default(),
            sort: Sort::new().await,
            library: Library::new().await,
            playlists: Playlists::new(&client).await?,
            playbar: Playbar::new(&client).await,
            main: MainBlock::Queue(Queue::new(&client).await?),
            active: None,
            hovered: Blokka::Library,
            hover_history: VecDeque::new() 
        })
    }

    pub fn is_hovered(&self, blk: Blokka) -> bool {
        if self.hovered==blk { return true; } false
    }

    pub fn is_active(&self, blk: Blokka) -> bool {
        if self.active==Some(blk) { return true; } false
    } 

    pub fn set_main(&mut self, blk: MainBlock) {
        self.main = blk;
        self.set_active(Blokka::Main);
    }


    pub fn set_active(&mut self, blk: Blokka) {
        self.active = Some(blk);
        self.hovered = blk;
    }

    pub fn set_hover(&mut self, blk: &Blokka) {
        self.hover_history.truncate(5);
        self.hover_history.push_front(self.hovered.clone());
        self.hovered = blk.clone();
    }  
}