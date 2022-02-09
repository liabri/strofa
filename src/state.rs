use crate::block::{ Blokka, MainBlock, Library, Podcasts, Artists, Queue, Albums, SearchResults, SelectableList, Playlists, Search, Sort, Playbar, Tracks, TrackKind, AlbumKind };
use crate::chunk::Chunks;
use crate::event::Key;
use crate::theme::Theme;
use crate::client::StrofaClient;

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

    pub async fn global_keybinds(&mut self, cmd: &str) -> Result<()> {
        match cmd {
            // binds manipulating ui
            "to_queue" => self.blocks.set_main(MainBlock::Queue(Queue::new(&self.client).await?)),
            // "toggle_top" => self.blocks
            "to_playlists" => self.blocks.set_active(Blokka::Playlists),
            "search" => self.blocks.set_active(Blokka::Search),
          
            // binds manipulating mpd  
            "toggle_playback" => self.client.toggle_playback().await?,
            "decrease_volume" => self.client.set_volume(-5).await?,
            "increase_volume" => self.client.set_volume(5).await?,
            "decrease_volume_big" => self.client.set_volume(-10).await?,
            "increase_volume_big" => self.client.set_volume(10).await?,
            "next_track" => self.client.next_track().await?,
            "previous_track" => self.client.previous_track().await?,
            "seek_forwards" => self.client.seek_forwards(10).await?,
            "seek_backwards" => self.client.seek_backwards(10).await?,
            "shuffle" => self.client.toggle_shuffle().await?,
            "repeat" => self.client.toggle_repeat().await?,
            // "jump_to_start" => self.blocks.playbar.jump_to_start(self.client.clone()).await,

            _ => {},
        }

        Ok(()) 
    }

    // new blocks are only made here !!
    pub async fn active_event(&mut self, key: Key) {
        match self.blocks.active {
            Some(Blokka::Search) => {
                match key {
                    Key::Enter => { 
                        let query = self.blocks.search.query.clone();
                        self.blocks.main = MainBlock::SearchResults(SearchResults::new(self.client.clone(), query).await.unwrap());
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
                            "Queue" => MainBlock::Queue(Queue::new(&self.client).await.unwrap()),
                            "Tracks" => MainBlock::Tracks(Tracks::new(TrackKind::All, &self.client).await.unwrap()),
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

            Some(Blokka::Playlists) => {
                match key {
                    Key::Up => self.blocks.playlists.index.dec(),
                    Key::Down => self.blocks.playlists.index.inc(),   
                    Key::Enter => {
                        let index = self.blocks.playlists.index.inner;  
                        let name = self.blocks.playlists.entries.get(0).unwrap().name.to_string();
                        self.blocks.set_main(MainBlock::Tracks(Tracks::new(TrackKind::Playlist(name), &self.client).await.unwrap()));
                    },

                    _ => {}
                }  
            },

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

    pub fn hovered_event(&mut self, key: Key) {
        match self.blocks.hovered {
            Blokka::Search => {
                match key {
                    Key::Down => {
                        for previous in self.blocks.hover_history.clone().into_iter() {
                            if previous == Blokka::Library || previous == Blokka::Main {
                                self.blocks.set_hover(&previous);
                                return;  
                            }
                        }

                        self.blocks.set_hover(&Blokka::Library)
                    },

                    Key::Right => self.blocks.set_hover(&Blokka::Sort),
                    _ => {},
                }
            },

            Blokka::Sort => {
                match key {
                    Key::Left => self.blocks.set_hover(&Blokka::Search),
                    Key::Down => self.blocks.set_hover(&Blokka::Main),
                    _ => {},
                }
            },

            Blokka::Library => {
                match key {
                    Key::Up => self.blocks.set_hover(&Blokka::Search),
                    Key::Down => self.blocks.set_hover(&Blokka::Playlists),
                    Key::Right => self.blocks.set_hover(&Blokka::Main),
                    _ => {},
                }
            },

            Blokka::Playlists => {
                match key {
                    Key::Up => self.blocks.set_hover(&Blokka::Library),
                    Key::Right => self.blocks.set_hover(&Blokka::Main),
                    _ => {},
                }
            },

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
    hover_history: VecDeque<Blokka>,
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
}


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