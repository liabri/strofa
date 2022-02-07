use crate::block::{ Blokka, MainBlock, Library, Podcasts, Artists, Albums, SearchResults, Main, Playlists, Search, Sort, Playbar, Tracks, TrackKind, AlbumKind };
use crate::event::Key;
use crate::theme::Theme;

use std::collections::{ VecDeque, HashMap };
use tui::layout::Rect;
use mpd_client::Client;

pub struct State {
    pub blocks: Blocks,
    pub size: Rect,
    pub theme: Theme,
    pub keys: KeyBindings,
    pub client: Client
}

impl State {
    pub async fn new(client: Client) -> Self {
        Self {
            blocks: Blocks::new(client.clone()).await,
            size: Rect::default(),
            theme: Theme::default(),
            keys: KeyBindings::default(),
            client,
        }
    }

    pub async fn handle_keybind(&mut self, cmd: &str) {
        match cmd {
            "to_queue" => self.blocks.set_main(MainBlock::Tracks(Tracks::new(TrackKind::Queue, self.client.clone()).await)),
            "search" => self.blocks.set_active(Blokka::Search),
            "toggle_playback" => self.blocks.playbar.toggle(self.client.clone()).await,
            _ => {},
        } 
    }

    // new blocks are only made here !!
    pub async fn active_event(&mut self, key: Key) {
        match self.blocks.active {
            Some(Blokka::Search) => {
                match key {
                    Key::Enter => { 
                        let query = self.blocks.search.query.clone();
                        self.blocks.main = MainBlock::SearchResults(SearchResults::new(query).await);
                        self.blocks.set_active(Blokka::Main);
                        self.blocks.hovered = Blokka::Main;
                    },

                    Key::Char(c) => {
                        self.blocks.search.query.push(c);
                        self.blocks.search.cursor_position+=1;
                    },

                    Key::Backspace => {
                        self.blocks.search.query.pop();
                        self.blocks.search.cursor_position-=1;
                    }

                    _ => {}
                }
            },

            Some(Blokka::Sort) => {},
            Some(Blokka::Library) => {
                match key {
                    Key::Up => self.blocks.library.index.dec(),
                    Key::Down => self.blocks.library.index.inc(),
                    Key::Enter => {
                        let index = self.blocks.library.index.inner;
                        let main_block = match self.blocks.library.entries[index] {
                            "Queue" => MainBlock::Tracks(Tracks::new(TrackKind::Queue, self.client.clone()).await),
                            "Tracks" => MainBlock::Tracks(Tracks::new(TrackKind::All, self.client.clone()).await),
                            "Albums" => MainBlock::Albums(Albums::new(AlbumKind::All).await),
                            "Artists" => MainBlock::Artists(Artists::new().await),
                            "Podcasts" => MainBlock::Podcasts(Podcasts::new().await),
                            _ => panic!("view not found"),
                        };

                        self.blocks.set_main(main_block);
                    }
                    _ => {},
                }
            },

            Some(Blokka::Playlists) => {},
            Some(Blokka::Error) => {},
            Some(Blokka::Main) => { 
                match key {
                    Key::Up => self.blocks.main.index().dec(),
                    Key::Down => self.blocks.main.index().inc(),
                    Key::Enter => {
                        match &self.blocks.main {
                            MainBlock::Tracks(x) => x.play(self.client.clone(), x.index.inner).await,
                            _ => {} //todo
                        }
                    }
                    _ => {}
                }
            },

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
    hover_history: VecDeque<Blokka>,
}

impl Blocks {
    pub async fn new(client: Client) -> Self {
        Self {
            search: Search::default(),
            sort: Sort::new().await,
            library: Library::new().await,
            playlists: Playlists::new().await,
            playbar: Playbar::new().await,
            main: MainBlock::Tracks(Tracks::new(TrackKind::Queue, client).await),
            active: None,
            hovered: Blokka::Library,
            hover_history: VecDeque::new() 
        }
    }

    pub fn is_hovered(&self, blk: Blokka) -> bool {
        if self.hovered==blk { return true; }
        false
    }

    pub fn is_active(&self, blk: Blokka) -> bool {
        if self.active==Some(blk) { return true; }
        false
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

    pub fn hover_previous(&mut self, idx: usize) -> &Blokka {
        self.hover_history.get(idx).unwrap_or(&Blokka::Search)
    }

    pub fn hovered_event(&mut self, key: Key) {
        match self.hovered {
            Blokka::Search => {
                match key {
                    Key::Down => {
                        for previous in self.hover_history.clone().into_iter() {
                            if previous == Blokka::Library || previous == Blokka::Main {
                                self.set_hover(&previous);
                                return;  
                            }
                        }

                        self.set_hover(&Blokka::Library)
                    },

                    Key::Right => self.set_hover(&Blokka::Sort),
                    _ => {},
                }
            },

            Blokka::Sort => {
                match key {
                    Key::Left => self.set_hover(&Blokka::Search),
                    Key::Down => self.set_hover(&Blokka::Main),
                    _ => {},
                }
            },

            Blokka::Library => {
                match key {
                    Key::Up => self.set_hover(&Blokka::Search),
                    Key::Down => self.set_hover(&Blokka::Playlists),
                    Key::Right => self.set_hover(&Blokka::Main),
                    _ => {},
                }
            },

            Blokka::Playlists => {
                match key {
                    Key::Up => self.set_hover(&Blokka::Library),
                    Key::Right => self.set_hover(&Blokka::Main),
                    _ => {},
                }
            },

            Blokka::Main => {
                match key {
                    Key::Up => self.set_hover(&Blokka::Search),
                    Key::Left => {
                        for previous in self.hover_history.clone().into_iter() {
                            if previous==Blokka::Library || previous==Blokka::Playlists {
                                self.set_hover(&previous);
                                return;
                            }
                        }

                        self.set_hover(&Blokka::Library)
                    },

                    Key::Right => self.set_hover(&Blokka::Sort),
                    _ => {},
                }
            },

            _ => {}   
        }

        // common behaviour
        match key {
            Key::Enter => self.set_active(self.hovered),
            _ => {}
        }
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