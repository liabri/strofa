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

// specific blocks

pub fn home<B>(f: &mut Frame<B>, state: &State, layout_chunk: Rect) where B: Backend {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(30), 
            Constraint::Percentage(70)
        ].as_ref())
        .split(layout_chunk);

    library(f, state, chunks[0]);
    playlists(f, state, chunks[1]);
}

pub fn centre<B>(f: &mut Frame<B>, state: &State, layout_chunk: Rect) where B: Backend {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
        .split(layout_chunk);

    home(f, state, chunks[0]);


    queue(f, state, chunks[1]); //temp

    //instead of passing everything through `state` we can pass it through StrofaBlock(X) enum fields
    match state.active_block {
        // StrofaBlock::SearchResults => search_results(f, state, chunks[1]),
        // StrofaBlock::Queue => queue(f, state, chunks[1]),
        // StrofaBlock::Albums => albums(f, state, chunks[1]),
        // StrofaBlock::Artists => artists(f, state, chunks[1]),
        // StrofaBlock::Tracks => tracks(f, state, chunks[1]),
        // StrofaBlock::Podcasts => podcasts(f, state, chunks[1]),
        _ => {}
    }
}

pub const LIBRARY_ENTRIES: [&str; 5] = [
    "Queue",
    "Tracks",
    "Albums",
    "Artists",
    "Podcasts"
];

pub fn library<B>(f: &mut Frame<B>, state: &State, layout_chunk: Rect) where B: Backend {
    let highlight_state = (
        state.active_block == StrofaBlock::Library,
        state.hovered_block == StrofaBlock::Library,
    );

    selectable_list(
        f,
        state,
        layout_chunk,
        "Library",
        &LIBRARY_ENTRIES,
        highlight_state,
        Some(0)// Some(app.library.selected_index),
    );
}

pub fn playlists<B>(f: &mut Frame<B>, state: &State, layout_chunk: Rect) where B: Backend {
    let highlight_state = (
        state.active_block == StrofaBlock::Playlists,
        state.hovered_block == StrofaBlock::Playlists,
    );

    selectable_list(
        f,
        state,
        layout_chunk,
        "Playlists",
        &["pop"],
        highlight_state,
        Some(0)// Some(app.library.selected_index),
    );
}

pub fn search<B>(f: &mut Frame<B>, state: &State, layout_chunk: Rect) where B: Backend {
    let highlight_state = (
        state.active_block == StrofaBlock::Search,
        state.hovered_block == StrofaBlock::Search,
    );

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(90), Constraint::Percentage(10)].as_ref())
        .split(layout_chunk);


    let input_string: String = String::new();//app.input.iter().collect();
    let lines = Text::from((&input_string).as_str());
    let input = Paragraph::new(lines).block(
        Block::default()
            .borders(Borders::ALL)
            .title(Span::styled(
                "Search",
                get_color(highlight_state, state.theme),
            )).border_style(get_color(highlight_state, state.theme)),
    );

    f.render_widget(input, chunks[0]);


    let block = Block::default()
        .title(Span::styled("Sort By", Style::default().fg(state.theme.text)))
        .borders(Borders::ALL)
        .border_style(get_color(highlight_state, state.theme));

    let lines = Text::from("Language");
    let sort = Paragraph::new(lines)
        .block(block)
        .style(get_color(highlight_state, state.theme));

    f.render_widget(sort, chunks[1]);
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
        &LIBRARY_ENTRIES,
        highlight_state,
        Some(0)// Some(app.library.selected_index),
    );
}

pub fn draw_playbar<B>(f: &mut Frame<B>, state: &State, layout_chunk: Rect) where B: Backend {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
        ].as_ref()).margin(1).split(layout_chunk);
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
    pub fn active_event(&self, key: Key, state: &State) {
        match self {
            StrofaBlock::Search => {},
            StrofaBlock::Sort => {},
            StrofaBlock::Library => {
                
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
    }
}
