use crate::state::State;
use crate::theme::get_color;
use crate::event::Key;

use tui::{
  backend::Backend,
  layout::{ Constraint, Direction, Layout, Rect },
  style::{ Modifier, Style },
  text::{ Span, Text },
  widgets::{ Block, Borders, List, ListItem, ListState, Paragraph },
  Frame,
};

#[derive(Default)]
pub struct Blocks {
    search: Search,
    sort: Sort,
    library: Library,
    playlists: Playlists,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum StrofaBlock {
    Search, // top
    Sort, // top
    Library, // home
    Playlists, // home
    Playbar, // bottom
    Error, // popup 
    Empty, // misc

    MainBlock(MainBlock)
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum MainBlock {
    SearchResults,
    Queue,
    Albums,
    Artists,
    Podcasts,
    Tracks,
}





pub fn top<B>(f: &mut Frame<B>, state: &State, layout_chunk: Rect) where B: Backend {
    let highlight_state = (
        state.active_block == StrofaBlock::Search,
        state.hovered_block == StrofaBlock::Search,
    );

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


    if let blk = state.main_block {
        match blk {
            // MainBlock::SearchResults => search_results(f, state, chunks[1]),
            MainBlock::Queue => queue(f, state, chunks[1]),
            // MainBlock::Albums => albums(f, state, chunks[1]),
            // MainBlock::Artists => artists(f, state, chunks[1]),
            // MainBlock::Tracks => tracks(f, state, chunks[1]),
            // MainBlock::Podcasts => podcasts(f, state, chunks[1])
            _ => {},
        }
    }
}





pub struct Library {
   pub entries: [&'static str; 5],
   pub index: usize 
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
            index: 0,
        }        
    }
}

impl Library {
    pub fn render<B>(&self, f: &mut Frame<B>, state: &State, layout_chunk: Rect) where B: Backend {
        let highlight_state = (
            state.active_block == StrofaBlock::Library,
            state.hovered_block == StrofaBlock::Library,
        );

        selectable_list(
            f,
            state,
            layout_chunk,
            "Library",
            &self.entries,
            highlight_state,
            Some(self.index)
        );
    }    
}



pub struct Playlists {
   pub entries: Vec<String>,
   pub index: usize 
}

impl Default for Playlists {
    fn default() -> Self {
        Self {
            entries: Vec::new(), //use mpd_client to get em
            index: 0,
        }        
    }
}

impl Playlists {
    pub fn render<B>(&self, f: &mut Frame<B>, state: &State, layout_chunk: Rect) where B: Backend {
        let highlight_state = (
            state.active_block == StrofaBlock::Playlists,
            state.hovered_block == StrofaBlock::Playlists,
        );

        selectable_list(
            f,
            state,
            layout_chunk,
            "Playlists",
            &self.entries.as_slice(),
            highlight_state,
            Some(self.index)
        );
    }    
}



#[derive(Default)]
pub struct Search {
   pub query: String,
}

impl Search {
    pub fn render<B>(&self, f: &mut Frame<B>, state: &State, layout_chunk: Rect) where B: Backend {
        let highlight_state = (
            state.active_block == StrofaBlock::Search,
            state.hovered_block == StrofaBlock::Search,
        );

        let lines = Text::from((&self.query).as_str());
        let search = Paragraph::new(lines).block(
            Block::default()
                .borders(Borders::ALL)
                .title(Span::styled(
                    "Search",
                    get_color(highlight_state, state.theme),
                )).border_style(get_color(highlight_state, state.theme)),
        );

        f.render_widget(search, layout_chunk);
    }    
}



pub struct Sort {
   pub entries: [&'static str; 2],
   pub asc: bool,
   pub index: usize 
}

impl Default for Sort {
    fn default() -> Self {
        Self {
            entries: [
                "Date of Release",
                "Language",
            ],
            asc: true,
            index: 0,
        }        
    }
}

impl Sort {
    pub fn render<B>(&self, f: &mut Frame<B>, state: &State, layout_chunk: Rect) where B: Backend {
        let highlight_state = (
            state.active_block == StrofaBlock::Sort,
            state.hovered_block == StrofaBlock::Sort,
        );

        let block = Block::default()
            .title(Span::styled("Sort By", Style::default().fg(state.theme.text)))
            .borders(Borders::ALL)
            .border_style(get_color(highlight_state, state.theme));

        let lines = Text::from(self.entries[0]);
        let sort = Paragraph::new(lines)
            .block(block)
            .style(get_color(highlight_state, state.theme));

        f.render_widget(sort, layout_chunk);
    }
}



pub fn playbar<B>(f: &mut Frame<B>, state: &State, layout_chunk: Rect) where B: Backend {
    let playback = Block::default()
        .title(Span::styled("Playback", Style::default().fg(state.theme.text)))
        .borders(Borders::ALL)
        .border_style(get_color((false, false), state.theme));

    f.render_widget(playback, layout_chunk);
}



pub fn queue<B>(f: &mut Frame<B>, state: &State, layout_chunk: Rect) where B: Backend {
    let highlight_state = (
        state.active_block == StrofaBlock::MainBlock(state.main_block),
        state.hovered_block == StrofaBlock::MainBlock(state.main_block),
    );

    selectable_list(
        f,
        state,
        layout_chunk,
        "Queue",
        &["pooop"],
        highlight_state,
        Some(0)// Some(app.library.selected_index),
    );
}

// generics

fn selectable_list<B, S>(f: &mut Frame<B>, state: &State, layout_chunk: Rect, title: &str, items: &[S], highlight_state: (bool, bool), selected_index: Option<usize>) 
where B: Backend, S: std::convert::AsRef<str> {
    let mut list_state = ListState::default();
    list_state.select(selected_index);

    let lst_items: Vec<ListItem> = items
        .iter()
        .map(|i| ListItem::new(Span::raw(i.as_ref())))
        .collect();

    let list = List::new(lst_items)
        .block(
            Block::default()
            .title(Span::styled(
                title,
                get_color(highlight_state, state.theme),
            )).borders(Borders::ALL).border_style(get_color(highlight_state, state.theme)),
        ).style(Style::default().fg(state.theme.text))
        .highlight_style(get_color(highlight_state, state.theme).add_modifier(Modifier::BOLD));

    f.render_stateful_widget(list, layout_chunk, &mut list_state);
}

impl StrofaBlock {
    pub fn active_event(&self, key: Key, state: &mut State) {
        match self {
            StrofaBlock::Search => {},
            StrofaBlock::Sort => {},
            StrofaBlock::Library => {
                match key {
                    Key::Up => state.blocks.library.index-=1,
                    Key::Down => state.blocks.library.index+=1,
                    Key::Enter => {
                        let index = state.blocks.library.index;
                        let main_block = match state.blocks.library.entries[index] {
                            "Queue" => MainBlock::Queue,
                            "Tracks" => MainBlock::Tracks,
                            "Albums" => MainBlock::Albums,
                            "Artists" => MainBlock::Artists,
                            "Podcasts" => MainBlock::Podcasts,
                            _ => MainBlock::Queue,
                        };

                        state.main_block = main_block;
                        state.active_block = StrofaBlock::MainBlock(main_block);
                        state.hovered_block = StrofaBlock::Empty;
                    }
                    _ => {},
                }
            },
            StrofaBlock::Playlists => {},
            StrofaBlock::Playbar => {},
            StrofaBlock::Error => {},
            StrofaBlock::Empty => {},
            StrofaBlock::MainBlock(SearchResults) => {},
            StrofaBlock::MainBlock(Queue) => {},
            StrofaBlock::MainBlock(Albums) => {},
            StrofaBlock::MainBlock(Artists) => {},
            StrofaBlock::MainBlock(Podcasts) => {},
            StrofaBlock::MainBlock(Tracks) => {}
        }
    }

    pub fn hovered_event(&self, key: Key, state: &mut State) {
        match self {
            StrofaBlock::Search => {
                match key {
                    Key::Down => state.hovered_block=StrofaBlock::Library, //make it so if came from Main, go back to main,
                    Key::Right => state.hovered_block=StrofaBlock::Sort,
                    _ => {},
                }
            },

            StrofaBlock::Sort => {
                match key {
                    Key::Left => state.hovered_block=StrofaBlock::Search,
                    Key::Down => state.hovered_block=StrofaBlock::MainBlock(state.main_block),
                    _ => {},
                }
            },

            StrofaBlock::Library => {
                println!("POPO");
                match key {
                    Key::Up => state.hovered_block=StrofaBlock::Search,
                    Key::Down => state.hovered_block=StrofaBlock::Playlists,
                    Key::Right => state.hovered_block=StrofaBlock::MainBlock(state.main_block),
                    _ => {},
                }
            },

            StrofaBlock::Playlists => {
                match key {
                    Key::Up => state.hovered_block=StrofaBlock::Library,
                    Key::Right => state.hovered_block=StrofaBlock::MainBlock(state.main_block),
                    _ => {},
                }
            },

            StrofaBlock::MainBlock(_) => {
                match key {
                    Key::Up => state.hovered_block=StrofaBlock::Search,
                    Key::Left => state.hovered_block=StrofaBlock::Library,
                    Key::Right => state.hovered_block=StrofaBlock::Playlists,
                    _ => {},
                }
            },

            _ => {}   
        }

        // common behaviour
        match key {
            Key::Enter => { 
                state.active_block=state.hovered_block;
                state.hovered_block=StrofaBlock::Empty;
            },
            _ => {}
        }
    }
}
