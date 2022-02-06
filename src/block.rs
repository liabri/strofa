use crate::state::State;
use crate::theme::get_color;
use crate::event::Key;

use anyhow::Result;
use mpd_client::{ Client };

use tui::{
  backend::Backend,
  layout::{ Constraint, Direction, Layout, Rect },
  style::{ Modifier, Style },
  text::{ Span, Text },
  widgets::{ Block, Borders, BorderType, List, ListItem, ListState, Paragraph },
  Frame,
};

pub struct Blocks {
    pub search: Search,
    pub sort: Sort,
    pub library: Library,
    pub playlists: Playlists,
    pub main: Box<dyn Main>,
}

impl Default for Blocks {
    fn default() -> Self {
        Self {
            main: Box::new(Tracks::new(&TrackKind::Queue)),
            search: Search::default(),
            sort: Sort::default(),
            library: Library::default(),
            playlists: Playlists::default(),
        }
    }
}

pub trait BlockTrait {
    fn render<B>(&self, f: &mut Frame<B>, state: &State, layout_chunk: Rect) where B: Backend;
}

#[derive(Clone, PartialEq, Debug)]
pub enum StrofaBlock {
    Search,
    Sort,
    Library,
    Playlists,
    Error, //todo popup
    Empty,
    MainBlock(MainBlock)
}

#[derive(Clone, PartialEq, Debug)]
pub enum MainBlock {
    SearchResults(String),
    Artists,
    Albums(AlbumKind),
    Tracks(TrackKind),
    Podcasts
}

#[derive(Clone, PartialEq, Debug)]
pub enum AlbumKind {
    Artist(String),
    All,
}

#[derive(Clone, PartialEq, Debug)]
pub enum TrackKind {
    Album(String),
    Artist(String),
    Playlist(String),
    Queue,
    All,
}

impl std::fmt::Display for TrackKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TrackKind::Album(s) => write!(f, " Album {} ", s),
            TrackKind::Artist(s) => write!(f, " Artist {} ", s),
            TrackKind::Playlist(s) => write!(f, " Playlist {} ", s),
            TrackKind::Queue => write!(f, " Queue "),
            TrackKind::All => write!(f, " Tracks "),

        }
    }
}

pub fn top<B>(f: &mut Frame<B>, state: &State, layout_chunk: Rect) where B: Backend {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(90), Constraint::Percentage(10)].as_ref())
        .split(layout_chunk);

    state.blocks.search.render(f, state, chunks[0]);
    state.blocks.sort.render(f, state, chunks[1]);
}

pub fn left<B>(f: &mut Frame<B>, state: &State, layout_chunk: Rect) where B: Backend {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(30), 
            Constraint::Percentage(70)
        ].as_ref())
        .split(layout_chunk);

    state.blocks.library.render(f, state, chunks[0]);
    state.blocks.playlists.render(f, state, chunks[1]);
}

pub fn centre<B>(f: &mut Frame<B>, state: &State, layout_chunk: Rect) where B: Backend {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
        .split(layout_chunk);

    left(f, state, chunks[0]);

    match &state.main_block {
        MainBlock::SearchResults(query) => SearchResults::new(query.to_string()).render(f, state, chunks[1]),
        MainBlock::Tracks(kind) => Tracks::new(kind).render(f, state, chunks[1]),
        MainBlock::Albums(kind) => Albums::new(kind).render(f, state, chunks[1]),
        MainBlock::Artists => Artists::new().render(f, state, chunks[1]),
        MainBlock::Podcasts => Podcasts::new().render(f, state, chunks[1]),
    }
}

pub fn bottom<B>(f: &mut Frame<B>, state: &State, layout_chunk: Rect) where B: Backend {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(layout_chunk);

    Playbar::new(&state.client).unwrap().render(f, state, chunks[0]);
}



pub struct Library {
   pub entries: [&'static str; 5],
   pub index: Index 
}

impl Default for Library {
    fn default() -> Self {
        Self {
            entries: [
                "Queue",
                "Tracks",
                "Albums",
                "Artists",
                "Podcasts"
            ],
            index: Index::new(5),
        }        
    }
}

impl BlockTrait for Library {
    fn render<B>(&self, f: &mut Frame<B>, state: &State, layout_chunk: Rect) where B: Backend {
        let highlight_state = (
            state.active_block == StrofaBlock::Library,
            state.hovered_block == StrofaBlock::Library,
        );

        let items: Vec<ListItem> = self.entries
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

pub struct Playlists {
   pub entries: Vec<String>,
   pub index: Index 
}

impl Default for Playlists {
    fn default() -> Self {
        Self {
            entries: Vec::new(), //use mpd_client to get em
            index: Index::new(50),
        }        
    }
}

impl BlockTrait for Playlists {
    fn render<B>(&self, f: &mut Frame<B>, state: &State, layout_chunk: Rect) where B: Backend {
        let highlight_state = (
            state.active_block == StrofaBlock::Playlists,
            state.hovered_block == StrofaBlock::Playlists,
        );

        let items: Vec<ListItem> = self.entries
            .iter()
            .map(|i| ListItem::new(Span::from(i.as_str())))
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

#[derive(Default)]
pub struct Search {
    pub index: usize,
    pub cursor_position: u16,
    pub query: String,
}

impl BlockTrait for Search {
    fn render<B>(&self, f: &mut Frame<B>, state: &State, layout_chunk: Rect) where B: Backend {
        let highlight_state = (
            state.active_block == StrofaBlock::Search,
            state.hovered_block == StrofaBlock::Search,
        );

        let lines = Text::from((&self.query).as_str());
        let search = Paragraph::new(lines).block(
            Block::default()
                .borders(Borders::ALL)
                .title(Span::styled(
                    " Search ",
                    get_color(highlight_state, state.theme),
                )).border_style(get_color(highlight_state, state.theme))
                .border_type(BorderType::Rounded),
        );

        f.render_widget(search, layout_chunk);
    }    
}



pub struct Sort {
    pub entries: [&'static str; 2],
    pub asc: bool,
    pub index: Index 
}

impl Default for Sort {
    fn default() -> Self {
        Self {
            entries: [
                "Date of Release",
                "Language",
            ],
            asc: true,
            index: Index::new(2),
        }        
    }
}

impl BlockTrait for Sort {
    fn render<B>(&self, f: &mut Frame<B>, state: &State, layout_chunk: Rect) where B: Backend {
        let highlight_state = (
            state.active_block == StrofaBlock::Sort,
            state.hovered_block == StrofaBlock::Sort,
        );

        let block = Block::default()
            .title(Span::styled(" Sort By ", Style::default().fg(state.theme.text)))
            .borders(Borders::ALL)
            .border_style(get_color(highlight_state, state.theme))
            .border_type(BorderType::Rounded);

        let lines = Text::from(self.entries[0]);
        let sort = Paragraph::new(lines)
            .block(block)
            .style(get_color(highlight_state, state.theme));

        f.render_widget(sort, layout_chunk);
    }
}



pub struct Playbar {
    // pub song: Song,
}

impl Playbar {
    pub fn new(client: &Client) -> Result<Self> {
        Ok(Self {})
        // Ok(async {
        //     let song = client.command(commands::CurrentSong).await?.unwrap().song;
        //     return Self {
        //         song,
        //     };
        // })
    }
}

impl BlockTrait for Playbar {
    fn render<B>(&self, f: &mut Frame<B>, state: &State, layout_chunk: Rect) where B: Backend {
        let playback = Block::default()
            .title(Span::styled(/*self.song.title().unwrap()*/" David Bowie - Life on Mars ", Style::default().fg(state.theme.text)))
            .borders(Borders::NONE);

        f.render_widget(playback, layout_chunk);
    }
}




pub trait Main {
    fn index(&mut self) -> &mut Index;
    // fn next_page(&mut self);
    // fn prev_page(&mut self);
}



pub struct Tracks {
    pub index: Index,
    pub kind: String,
    // pub songs: Vec<Song>,
}

impl Main for Tracks {
    fn index(&mut self) -> &mut Index {
        &mut self.index
    }
}

impl Tracks {
    fn new(kind: &TrackKind) -> Self {
    //use kind to populate tracks
        Self {
            kind: kind.to_string(),
            index: Index::new(50),
        }
    }
}

impl BlockTrait for Tracks {
    fn render<B>(&self, f: &mut Frame<B>, state: &State, layout_chunk: Rect) where B: Backend {
        let highlight_state = (
            if let StrofaBlock::MainBlock(_) = state.active_block { true } else { false },
            if let StrofaBlock::MainBlock(_) = state.hovered_block { true } else { false },
        );

        let items: Vec<ListItem> = Vec::new(); 
        // items
        //     .iter()
        //     .map(|i| ListItem::new(Span::raw(i.as_ref())))
        //     .collect();

        selectable_list(
            f,
            state,
            layout_chunk,
            &self.kind,
            items,
            highlight_state,
            Some(self.index.inner)
        );
    }
}





pub struct Albums {
    pub index: Index,
    // pub songs: Vec<Song>,
}

impl Main for Albums {
    fn index(&mut self) -> &mut Index {
        &mut self.index
    }
}

impl Albums {
    fn new(kind: &AlbumKind) -> Self {
        //use kind to populate tracks

        Self {
            index: Index::new(50),
        }
    }

    fn render<B>(&self, f: &mut Frame<B>, state: &State, layout_chunk: Rect) where B: Backend {
        let highlight_state = (
            if let StrofaBlock::MainBlock(_) = state.active_block { true } else { false },
            if let StrofaBlock::MainBlock(_) = state.hovered_block { true } else { false },
        );

        let items: Vec<ListItem> = Vec::new(); 
        // items
        //     .iter()
        //     .map(|i| ListItem::new(Span::raw(i.as_ref())))
        //     .collect();

        selectable_list(
            f,
            state,
            layout_chunk,
            " Albums ",
            items,
            highlight_state,
            Some(self.index.inner)
        );
    }
}




pub struct Artists {
    pub index: Index,
    // pub songs: Vec<Song>,
}

impl Main for Artists {
    fn index(&mut self) -> &mut Index {
        &mut self.index
    }
}

impl Artists {
    fn new() -> Self {
        Self {
            index: Index::new(50),
        }
    }

    fn render<B>(&self, f: &mut Frame<B>, state: &State, layout_chunk: Rect) where B: Backend {
        let highlight_state = (
            if let StrofaBlock::MainBlock(_) = state.active_block { true } else { false },
            if let StrofaBlock::MainBlock(_) = state.hovered_block { true } else { false },
        );

        let items: Vec<ListItem> = Vec::new(); 
        // items
        //     .iter()
        //     .map(|i| ListItem::new(Span::raw(i.as_ref())))
        //     .collect();

        selectable_list(
            f,
            state,
            layout_chunk,
            " Artists ",
            items,
            highlight_state,
            Some(self.index.inner)
        );
    }
}



pub struct Podcasts {
    pub index: Index,
}

impl Main for Podcasts {
    fn index(&mut self) -> &mut Index {
        &mut self.index
    }
}

impl Podcasts {
    fn new() -> Self {
        Self {
            index: Index::new(50),
        }
    }

    fn render<B>(&self, f: &mut Frame<B>, state: &State, layout_chunk: Rect) where B: Backend {
        let highlight_state = (
            if let StrofaBlock::MainBlock(_) = state.active_block { true } else { false },
            if let StrofaBlock::MainBlock(_) = state.hovered_block { true } else { false },
        );

        let items: Vec<ListItem> = Vec::new(); 
        // items
        //     .iter()
        //     .map(|i| ListItem::new(Span::raw(i.as_ref())))
        //     .collect();

        selectable_list(
            f,
            state,
            layout_chunk,
            " Podcasts ",
            items,
            highlight_state,
            Some(self.index.inner)
        );
    }
}


pub struct SearchResults {
    pub index: Index,
    pub query: String,
}

impl Main for SearchResults {
    fn index(&mut self) -> &mut Index {
        &mut self.index
    }
}

impl SearchResults {
    fn new(query: String) -> Self {
        Self {
            query,
            index: Index::new(50),
        }
    }

    fn render<B>(&self, f: &mut Frame<B>, state: &State, layout_chunk: Rect) where B: Backend {
        let highlight_state = (
            if let StrofaBlock::MainBlock(_) = state.active_block { true } else { false },
            if let StrofaBlock::MainBlock(_) = state.hovered_block { true } else { false },
        );

        let items: Vec<ListItem> = Vec::new(); 
        // items
        //     .iter()
        //     .map(|i| ListItem::new(Span::raw(i.as_ref())))
        //     .collect();

        selectable_list(
            f,
            state,
            layout_chunk,
            " Search Results ",
            items,
            highlight_state,
            Some(self.index.inner)
        );
    }
}

// generics

pub struct Index {
    inner: usize,
    max: usize,
}

impl Index {
    pub fn new(max: usize) -> Self {
        Index {
            inner: 0,
            max: max-1,
        }
    }

    pub fn dec(&mut self) {
        if self.inner > 0 {
            self.inner-=1;
        }
    }

    pub fn inc(&mut self) {
        if self.inner < self.max {
            self.inner+=1;
        }
    }
}

fn selectable_list<B>(f: &mut Frame<B>, state: &State, layout_chunk: Rect, title: &str, items: Vec<ListItem>, highlight_state: (bool, bool), selected_index: Option<usize>) 
where B: Backend {
    let mut list_state = ListState::default();
    list_state.select(selected_index);

    let colour = get_color(highlight_state, state.theme);
    let list = List::new(items).block(Block::default()
        .title(Span::styled(title, colour)).borders(Borders::ALL).border_style(colour).border_type(BorderType::Rounded)
    ).style(Style::default().fg(state.theme.text)).highlight_style(colour.add_modifier(Modifier::BOLD));

    f.render_stateful_widget(list, layout_chunk, &mut list_state);
}

impl StrofaBlock {
    pub fn active_event(&self, key: Key, state: &mut State) {
        match self {
            StrofaBlock::Search => {
                match key {
                    Key::Enter => { 
                        let query = state.blocks.search.query.clone();
                        state.set_active(StrofaBlock::MainBlock(MainBlock::SearchResults(query)));
                    },

                    _ => {}
                }
            },

            StrofaBlock::Sort => {},
            StrofaBlock::Library => {
                match key {
                    Key::Up => state.blocks.library.index.dec(),
                    Key::Down => state.blocks.library.index.inc(),
                    Key::Enter => {
                        let index = state.blocks.library.index.inner;
                        let main_block = match state.blocks.library.entries[index] {
                            "Queue" => MainBlock::Tracks(TrackKind::Queue),
                            "Tracks" => MainBlock::Tracks(TrackKind::All),
                            "Albums" => MainBlock::Albums(AlbumKind::All),
                            "Artists" => MainBlock::Artists,
                            "Podcasts" => MainBlock::Podcasts,
                            _ => panic!("view not found"),
                        };

                        state.set_hover(&StrofaBlock::Library);
                        state.main_block = main_block.clone();
                        state.active_block = StrofaBlock::MainBlock(main_block);
                        state.set_hover(&state.active_block.clone());
                    }
                    _ => {},
                }
            },

            StrofaBlock::Playlists => {},
            StrofaBlock::Error => {},
            StrofaBlock::Empty => {},
            StrofaBlock::MainBlock(blk) => { 
                match key {
                    Key::Up => state.blocks.main.index().dec(),
                    Key::Down => state.blocks.main.index().inc(),
                    Key::Enter => {
                        match blk {
                            _ => {} //todo
                        }
                    }
                    _ => {}
                }
            },
        }
    }

    pub fn hovered_event(&self, key: Key, state: &mut State) {
        match self {
            StrofaBlock::Search => {
                match key {
                    Key::Down => {
                        for previous in state.hover_history.clone().into_iter() {
                            if previous == StrofaBlock::Library {
                                state.set_hover(&previous);
                                return;  
                            }

                            if let StrofaBlock::MainBlock(_) = previous {
                                state.set_hover(&previous);
                                return;   
                            }
                        }

                        state.set_hover(&StrofaBlock::Library)
                    },

                    Key::Right => state.set_hover(&StrofaBlock::Sort),
                    _ => {},
                }
            },

            StrofaBlock::Sort => {
                match key {
                    Key::Left => state.set_hover(&StrofaBlock::Search),
                    Key::Down => state.set_hover(&StrofaBlock::MainBlock(state.main_block.clone())),
                    _ => {},
                }
            },

            StrofaBlock::Library => {
                match key {
                    Key::Up => state.set_hover(&StrofaBlock::Search),
                    Key::Down => state.set_hover(&StrofaBlock::Playlists),
                    Key::Right => state.set_hover(&StrofaBlock::MainBlock(state.main_block.clone())),
                    _ => {},
                }
            },

            StrofaBlock::Playlists => {
                match key {
                    Key::Up => state.set_hover(&StrofaBlock::Library),
                    Key::Right => state.set_hover(&StrofaBlock::MainBlock(state.main_block.clone())),
                    _ => {},
                }
            },

            StrofaBlock::MainBlock(_) => {
                match key {
                    Key::Up => state.set_hover(&StrofaBlock::Search),
                    Key::Left => {
                        for previous in state.hover_history.clone().into_iter() {
                            if previous==StrofaBlock::Library || previous==StrofaBlock::Playlists {
                                state.set_hover(&previous);
                                return;
                            }
                        }

                        state.set_hover(&StrofaBlock::Library)
                    },

                    Key::Right => state.set_hover(&StrofaBlock::Sort),
                    _ => {},
                }
            },

            _ => {}   
        }

        // common behaviour
        match key {
            Key::Enter => state.active_block=state.hovered_block.clone(),
            _ => {}
        }
    }
}
