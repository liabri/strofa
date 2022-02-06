use crate::block::{ Blokka, MainBlock, Library, SearchResults, Main, Playlists, Search, Sort, Playbar, Tracks, TrackKind };
use crate::event::Key;
use crate::theme::Theme;

use std::collections::{ VecDeque, HashMap };
use tui::layout::Rect;

pub struct State {
    pub blocks: Blocks,
    pub size: Rect,
    pub theme: Theme,
    pub keys: KeyBindings,
}

//are there any benefits to keeping Blokka if im using Strings identify them anyway ?
//i think i should separate the enum and the "block" objects, using the enum to solely keep track
//of whats going on with the ui

impl State {
    pub fn new() -> Self {
        Self {
            blocks: Blocks::new(),
            size: Rect::default(),
            theme: Theme::default(),
            keys: KeyBindings::default(),
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
    pub fn new() -> Self {
        Self {
            search: Search::default(),
            sort: Sort::default(),
            library: Library::default(),
            playlists: Playlists::default(),
            playbar: Playbar::default(),
            main: MainBlock::Tracks(Tracks::new(&TrackKind::Queue)),
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

    // new blocks are only made here !!
    pub fn active_event(&mut self, key: Key) {
        match self.active {
            Some(Blokka::Search) => {
                match key {
                    Key::Enter => { 
                        let query = self.search.query.clone();
                        self.main = MainBlock::SearchResults(SearchResults::new(query));
                        self.set_active(Blokka::Main);
                        self.hovered = Blokka::Main;
                    },

                    Key::Char(c) => {
                        self.search.query.push(c);
                        self.search.cursor_position+=1;
                    },

                    Key::Backspace => {
                        self.search.query.pop();
                        self.search.cursor_position-=1;
                    }

                    _ => {}
                }
            },

            Some(Blokka::Sort) => {},
            Some(Blokka::Library) => {
                match key {
                    Key::Up => self.library.index.dec(),
                    Key::Down => self.library.index.inc(),
                    // Key::Enter => {
                    //     let index = self.library.index.inner;
                    //     let main_block = match self.library.entries[index] {
                    //         "Queue" => MainBlock::Tracks(TrackKind::Queue),
                    //         "Tracks" => MainBlock::Tracks(TrackKind::All),
                    //         "Albums" => MainBlock::Albums(AlbumKind::All),
                    //         "Artists" => MainBlock::Artists,
                    //         "Podcasts" => MainBlock::Podcasts,
                    //         _ => panic!("view not found"),
                    //     };

                    //     state.set_hover(&Blokka::Library);
                    //     state.main_block = main_block.clone();
                    //     state.active_block = Blokka::Main(main_block);
                    //     state.set_hover(&state.active_block.clone());
                    // }
                    _ => {},
                }
            },

            Some(Blokka::Playlists) => {},
            Some(Blokka::Error) => {},
            Some(Blokka::Main) => { 
                // match key {
                //     Key::Up => self.main.index().dec(),
                //     Key::Down => self.main.index().inc(),
                //     Key::Enter => {
                //         match blk {
                //             _ => {} //todo
                //         }
                //     }
                //     _ => {}
                // }
            },

            _ => {}
        }
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