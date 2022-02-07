use crate::state::State;
use crate::theme::get_color;

use mpd_client::commands::responses::{ Song, SongInQueue, Playlist, PlayState };
use mpd_client::{ Client, commands };

use tui::{
    backend::Backend,
    layout::{ Constraint, Direction, Layout, Rect },
    style::{ Modifier, Style },
    text::{ Span, Text },
    widgets::{ Block, Borders, BorderType, List, ListItem, ListState, Paragraph },
    Frame,
};

//maybe move to state ?
#[derive(Copy, Clone, PartialEq)]
pub enum Blokka {
    Search,
    Sort,
    Library,
    Playlists,
    Playbar,
    Error, //todo popup
    // Help, //todo popup, will contains all shortcuts
    Main//(MainBlock)
}

pub trait Render<B: Backend> {
    fn render(&self, f: &mut Frame<B>, state: &State, layout_chunk: Rect);
}

pub fn top<B>(f: &mut Frame<B>, state: &State, layout_chunk: Rect) where B: Backend {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(90), Constraint::Percentage(10)].as_ref())
        .split(layout_chunk);

    state.blocks.search.render(f, state, chunks[0]);
    state.blocks.sort.render(f, state, chunks[1]);
}

pub fn centre<B>(f: &mut Frame<B>, state: &State, layout_chunk: Rect) where B: Backend {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
        .split(layout_chunk);

    left(f, state, chunks[0]);
    state.blocks.main.render(f, state, chunks[1]);
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

pub fn bottom<B>(f: &mut Frame<B>, state: &State, layout_chunk: Rect) where B: Backend {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(layout_chunk);

    state.blocks.playbar.render(f, state, chunks[0]);
}


pub struct Library {
   pub entries: [&'static str; 5],
   pub index: Index 
}

impl Library {
    pub async fn new() -> Self {
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

impl<B: Backend> Render<B> for Library {
    fn render(&self, f: &mut Frame<B>, state: &State, layout_chunk: Rect) {
        let highlight_state = (
            state.blocks.is_active(Blokka::Library),
            state.blocks.is_hovered(Blokka::Library)
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
   pub entries: Vec<Playlist>,
   pub index: Index 
}

impl Playlists {
    pub async fn new() -> Self {
        Self {
            entries: Vec::new(),
            index: Index::new(50),
        }        
    }
}

impl<B: Backend> Render<B> for Playlists {
    fn render(&self, f: &mut Frame<B>, state: &State, layout_chunk: Rect) {
        let highlight_state = (
            state.blocks.is_active(Blokka::Playlists),
            state.blocks.is_hovered(Blokka::Playlists)
        );

        let items: Vec<ListItem> = self.entries
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

#[derive(Default)]
pub struct Search {
    pub index: usize,
    pub cursor_position: u16,
    pub query: String,
}

impl<B: Backend> Render<B> for Search {
    fn render(&self, f: &mut Frame<B>, state: &State, layout_chunk: Rect) {
        let highlight_state = (
            state.blocks.is_active(Blokka::Search),
            state.blocks.is_hovered(Blokka::Search)
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

impl Sort {
    pub async fn new() -> Self {
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

impl<B: Backend> Render<B> for Sort {
    fn render(&self, f: &mut Frame<B>, state: &State, layout_chunk: Rect) {
        let highlight_state = (
            state.blocks.is_active(Blokka::Sort),
            state.blocks.is_hovered(Blokka::Sort)
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
    pub song: Option<SongInQueue>,
}

impl Playbar {
    pub async fn new() -> Self {
        Self { 
            song: None,
        }
    }

    pub async fn toggle(&self, client: Client) {
        let status = client.command(commands::Status).await.unwrap();
        match status.state {
            PlayState::Stopped => client.command(commands::SetPause(true)).await.unwrap(),
            PlayState::Playing => client.command(commands::SetPause(true)).await.unwrap(),
            PlayState::Paused => client.command(commands::Play::current()).await.unwrap(),
        }
    }
}

impl<B: Backend> Render<B> for Playbar {
    fn render(&self, f: &mut Frame<B>, state: &State, layout_chunk: Rect) {
        let playbar = Block::default()
            .title(Span::styled(/*self.current_song.as_ref().unwrap().song.title().unwrap_or("Empty")*/ "pla", Style::default().fg(state.theme.text)))
            .borders(Borders::NONE);

        f.render_widget(playbar, layout_chunk);
    }
}




pub enum MainBlock {
    SearchResults(SearchResults),
    Artists(Artists),
    Albums(Albums),
    Tracks(Tracks),
    Podcasts(Podcasts)
}


impl<B: Backend> Render<B> for MainBlock {
    fn render(&self, f: &mut Frame<B>, state: &State, layout_chunk: Rect) {
        match self {
            MainBlock::SearchResults(x) => x.render(f, state, layout_chunk),
            MainBlock::Artists(x) => x.render(f, state, layout_chunk),
            MainBlock::Albums(x) => x.render(f, state, layout_chunk),
            MainBlock::Tracks(x) => x.render(f, state, layout_chunk),
            MainBlock::Podcasts(x) => x.render(f, state, layout_chunk),

        }
    }
}

impl Main for MainBlock {
    fn index(&mut self) -> &mut Index {
        match self {
            MainBlock::SearchResults(x) => x.index(), 
            MainBlock::Artists(x) => x.index(), 
            MainBlock::Albums(x) => x.index(), 
            MainBlock::Tracks(x) => x.index(), 
            MainBlock::Podcasts(x) => x.index(), 
        }
    }
}


pub trait Main {
    fn index(&mut self) -> &mut Index;
}

pub enum AlbumKind {
    Artist(String),
    All,
}

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
            TrackKind::All => write!(f, " Tracks ")
        }
    }
}

pub struct Tracks {
    pub index: Index,
    pub kind: String,
    pub tracks: Vec<SongInQueue>,
}

impl Main for Tracks {
    fn index(&mut self) -> &mut Index {
        &mut self.index
    }
}

// eventually access mpd directly from here, need to async it and pass in `client`
impl Tracks {
    pub async fn new(kind: TrackKind, client: Client) -> Self {
        let tracks: Vec<SongInQueue> = match kind {
            TrackKind::Queue => client.command(commands::Queue).await.unwrap(),
             _ => Vec::new(),
        };

        Self {
            kind: kind.to_string(),
            index: Index::new(50),
            tracks,
        }
    }

    pub async fn play(&self, client: Client, index: usize) {
        let song = self.tracks.get(index).unwrap().id;
        client.command(commands::Play::song(song)).await.unwrap();
    }
}

impl<B: Backend> Render<B> for Tracks {
    fn render(&self, f: &mut Frame<B>, state: &State, layout_chunk: Rect) {
        let highlight_state = (
            state.blocks.is_active(Blokka::Main),
            state.blocks.is_hovered(Blokka::Main)
        );

        let items: Vec<ListItem> = self.tracks
            .iter()
            .map(|x| ListItem::new(Span::raw(x.song.title().unwrap_or("Ger"))))
            .collect();

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
    pub async fn new(kind: AlbumKind) -> Self {
        //use kind to populate tracks

        Self {
            index: Index::new(50),
        }
    }
}

impl<B: Backend> Render<B> for Albums {
    fn render(&self, f: &mut Frame<B>, state: &State, layout_chunk: Rect) {
        let highlight_state = (
            state.blocks.is_active(Blokka::Main),
            state.blocks.is_hovered(Blokka::Main)
        );

        let items: Vec<ListItem> = Vec::new(); 

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
    // pub artists: Vec<Artist>,
}

impl Main for Artists {
    fn index(&mut self) -> &mut Index {
        &mut self.index
    }
}

impl Artists {
    pub async fn new() -> Self {
        Self {
            index: Index::new(50),
        }
    }
}

impl<B: Backend> Render<B> for Artists {
    fn render(&self, f: &mut Frame<B>, state: &State, layout_chunk: Rect) {
        let highlight_state = (
            state.blocks.is_active(Blokka::Main),
            state.blocks.is_hovered(Blokka::Main)
        );

        let items: Vec<ListItem> = Vec::new(); 

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
    pub async fn new() -> Self {
        Self {
            index: Index::new(50),
        }
    }
}

impl<B: Backend> Render<B> for Podcasts {
    fn render(&self, f: &mut Frame<B>, state: &State, layout_chunk: Rect) {
        let highlight_state = (
            state.blocks.is_active(Blokka::Main),
            state.blocks.is_hovered(Blokka::Main)
        );

        let items: Vec<ListItem> = Vec::new(); 

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
    pub async fn new(query: String) -> Self {
        Self {
            query,
            index: Index::new(50),
        }
    }
}

impl<B: Backend> Render<B> for SearchResults {
    fn render(&self, f: &mut Frame<B>, state: &State, layout_chunk: Rect) {
        let highlight_state = (
            state.blocks.is_active(Blokka::Main),
            state.blocks.is_hovered(Blokka::Main)
        );

        let items: Vec<ListItem> = Vec::new(); 

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
    pub inner: usize,
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

fn selectable_list<B>(f: &mut Frame<B>, state: &State, layout_chunk: Rect, title: &str, items: Vec<ListItem>, highlight_state: (bool, bool), selected_index: Option<usize>) where B: Backend {
    let mut list_state = ListState::default();
    list_state.select(selected_index);

    let colour = get_color(highlight_state, state.theme);
    let list = List::new(items).block(Block::default()
        .title(Span::styled(title, colour)).borders(Borders::ALL).border_style(colour).border_type(BorderType::Rounded)
    ).style(Style::default().fg(state.theme.text)).highlight_style(colour.add_modifier(Modifier::BOLD));

    f.render_stateful_widget(list, layout_chunk, &mut list_state);
}