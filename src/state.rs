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
    pub async fn new(client: Client) -> Result<Self> {
        Ok(Self {
            blocks: Blocks::new(client.clone()).await?,
            size: Rect::default(),
            theme: Theme::default(),
            keys: KeyBindings::default(),
            client,
        })
    }

    pub async fn handle_keybind(&mut self, cmd: &str) -> Result<()> {
        match cmd {
            "to_queue" => self.blocks.set_main(MainBlock::Tracks(Tracks::new(TrackKind::Queue, self.client.clone()).await?)),
            "to_playlists" => self.blocks.set_active(Blokka::Playlists),
            "search" => self.blocks.set_active(Blokka::Search),
            
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
                            "Queue" => MainBlock::Tracks(Tracks::new(TrackKind::Queue, self.client.clone()).await.unwrap()),
                            "Tracks" => MainBlock::Tracks(Tracks::new(TrackKind::All, self.client.clone()).await.unwrap()),
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
                        self.blocks.set_main(MainBlock::Tracks(Tracks::new(TrackKind::Playlist(name), self.client.clone()).await.unwrap()));
                    },

                    _ => {}
                }  
            },

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
            playlists: Playlists::new(client.clone()).await?,
            playbar: Playbar::new(client.clone()).await,
            main: MainBlock::Tracks(Tracks::new(TrackKind::Queue, client).await?),
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

    pub fn hover_previous(&mut self, idx: usize) -> &Blokka {
        self.hover_history.get(idx).unwrap_or(&Blokka::Search)
    }  
}

pub struct KeyBindings(pub HashMap<Key, String>);
impl Default for KeyBindings {
    fn default() -> Self {
        let mut map: HashMap<Key, String> = HashMap::new();

        map.insert(Key::Backspace, "back".to_string());
        map.insert(Key::Char('q'), "to_queue".to_string());
        map.insert(Key::Char('e'), "to_playlists".to_string());

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

use async_trait::async_trait;
use anyhow::Result;
use mpd_client::{ CommandError, commands, commands::responses::{ PlayState  }};
use std::time::Duration;

#[async_trait]
pub trait Fnx {
    async fn toggle_playback(&self) -> Result<(), CommandError>;
    async fn set_volume(&self, o: i8) -> Result<(), CommandError>;
    async fn next_track(&self) -> Result<(), CommandError>;
    async fn previous_track(&self) -> Result<(), CommandError>;
    async fn seek_forwards(&self, o: u64) -> Result<(), CommandError>;
    async fn seek_backwards(&self, o: u64) -> Result<(), CommandError>;
    async fn toggle_shuffle(&self) -> Result<(), CommandError>;
    async fn toggle_repeat(&self) -> Result<(), CommandError>;
}

#[async_trait]
impl Fnx for Client {
    async fn toggle_playback(&self) -> Result<(), CommandError> {
        let status = self.command(commands::Status).await?;
        match status.state {
            PlayState::Stopped => self.command(commands::SetPause(true)).await,
            PlayState::Playing => self.command(commands::SetPause(true)).await,
            PlayState::Paused => self.command(commands::Play::current()).await,
        }
    }

    async fn set_volume(&self, o: i8) -> Result<(), CommandError> {
        let current_volume = self.command(commands::Status).await?.volume;
        let new_volume = (current_volume as i8)+o;
        
        let vol = if new_volume < 0 {
            0
        } else if new_volume > 100 {
            100
        } else {
            new_volume
        };

        self.command(commands::SetVolume(vol.try_into().unwrap())).await
    }

    async fn next_track(&self) -> Result<(), CommandError> {
        self.command(commands::Next).await
    }

    async fn previous_track(&self) -> Result<(), CommandError> {
        self.command(commands::Previous).await
    }

    async fn seek_forwards(&self, o: u64) -> Result<(), CommandError> {
        self.command(commands::Seek(commands::SeekMode::Forward(Duration::from_secs(o)))).await
    }

    async fn seek_backwards(&self, o: u64) -> Result<(), CommandError> {
        self.command(commands::Seek(commands::SeekMode::Backward(Duration::from_secs(o)))).await
    }

    async fn toggle_shuffle(&self) -> Result<(), CommandError> {
        let current_shuffle = self.command(commands::Status).await?.random;
        self.command(commands::SetRandom(!current_shuffle)).await
    }

    async fn toggle_repeat(&self) -> Result<(), CommandError> {
        let current_repeat = self.command(commands::Status).await?.repeat;
        self.command(commands::SetRepeat(!current_repeat)).await
    }

    // async fn jump_to_start(&self, client: Client) {
    //     if let Some(current_song) = &self.song {
    //         let current_id = current_song.id;
    //         let pos = commands::SongPosition(0);
    //         client.command(commands::Move::id(current_id).to_position(pos)).await.unwrap();
    //     }
    // }
}