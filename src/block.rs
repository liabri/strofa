use crate::state::State;
use crate::theme::get_color;

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
    PlayBar,
    AlbumTracks,
    AlbumList,
    ArtistBlock,
    Empty,
    Error,
    HelpMenu,
    Search,
    Library,
    Playlists,
    Podcasts,
    Queue,
    SearchResultBlock,
    TrackList,
    Artists,
}

// specific blocks

pub fn home<B>(f: &mut Frame<B>, state: &State, layout_chunk: Rect) where B: Backend {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), 
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

    match state.active_block {
        StrofaBlock::Queue => {},
        StrofaBlock::AlbumTracks => {},
        StrofaBlock::AlbumList => {},
        StrofaBlock::ArtistBlock => {},
        StrofaBlock::SearchResultBlock => {},
        StrofaBlock::TrackList => {},

        _ => {}
    }
}

pub const LIBRARY_ENTRIES: [&str; 4] = [
    "Songs",
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
        &LIBRARY_ENTRIES,
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
        .constraints([Constraint::Percentage(100), Constraint::Percentage(10)].as_ref())
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
}

pub fn queue<B>(f: &mut Frame<B>, state: &State, layout_chunk: Rect) where B: Backend {
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