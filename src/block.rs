use crate::{ State, StrofaBlock };
use crate::theme::get_color;

use tui::{
  backend::Backend,
  layout::{Alignment, Constraint, Direction, Layout, Rect},
  style::{Modifier, Style},
  text::{Span, Spans, Text},
  widgets::{Block, Borders, Clear, Gauge, List, ListItem, ListState, Paragraph, Row, Table, Wrap},
  Frame,
};

// pub fn routes<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect) where B: Backend {
//     let chunks = Layout::default()
//         .direction(Direction::Horizontal)
//         .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
//         .split(layout_chunk);

//     draw_user_block(f, app, chunks[0]);

//     let current_route = app.get_current_route();

//     match current_route.id {
//         RouteId::Search => draw_search_results(f, app, chunks[1]),
//         RouteId::TrackTable => draw_song_table(f, app, chunks[1]),
//         RouteId::AlbumTracks => draw_album_table(f, app, chunks[1]),
//         RouteId::RecentlyPlayed => draw_recently_played_table(f, app, chunks[1]),
//         RouteId::Artist => draw_artist_albums(f, app, chunks[1]),
//         RouteId::AlbumList => draw_album_list(f, app, chunks[1]),
//         RouteId::PodcastEpisodes => draw_show_episodes(f, app, chunks[1]),
//         RouteId::Home => draw_home(f, app, chunks[1]),
//         RouteId::MadeForYou => draw_made_for_you(f, app, chunks[1]),
//         RouteId::Artists => draw_artist_table(f, app, chunks[1]),
//         RouteId::Podcasts => draw_podcast_table(f, app, chunks[1]),
//         RouteId::Recommendations => draw_recommendations_table(f, app, chunks[1]),
//         RouteId::Error => {} // This is handled as a "full screen" route in main.rs
//         RouteId::SelectedDevice => {} // This is handled as a "full screen" route in main.rs
//         RouteId::Analysis => {} // This is handled as a "full screen" route in main.rs
//         RouteId::BasicView => {} // This is handled as a "full screen" route in main.rs
//         RouteId::Dialog => {} // This is handled in the draw_dialog function in mod.rs
//     };
// }

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

    f.render_widget(input, layout_chunk);
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