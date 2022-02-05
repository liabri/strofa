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
    Search, // top
    Library, // home
    Playlists, // home
    PlayBar, // bottom
    Error, // popup 
    Empty, // misc

    // centre 
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

pub const LIBRARY_ENTRIES: [&str; 4] = [
    "Queue"
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
        state.active_block == StrofaBlock::Queue,
        state.hovered_block == StrofaBlock::Queue,
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

// If no track is playing, render paragraph showing which device is selected, if no selected
// give hint to choose a device
if let Some(current_playback_context) = &app.current_playback_context {
if let Some(track_item) = &current_playback_context.item {
    let play_title = if current_playback_context.is_playing {
    "Playing"
    } else {
    "Paused"
    };

let shuffle_text = if current_playback_context.shuffle_state {
"On"
} else {
"Off"
};

let repeat_text = match current_playback_context.repeat_state {
RepeatState::Off => "Off",
RepeatState::Track => "Track",
RepeatState::Context => "All",
};

let title = format!(
"{:-7} ({} | Shuffle: {:-3} | Repeat: {:-5} | Volume: {:-2}%)",
play_title,
current_playback_context.device.name,
shuffle_text,
repeat_text,
current_playback_context.device.volume_percent
);

let current_route = app.get_current_route();
let highlight_state = (
current_route.active_block == ActiveBlock::PlayBar,
current_route.hovered_block == ActiveBlock::PlayBar,
);

let title_block = Block::default()
.borders(Borders::ALL)
.title(Span::styled(
&title,
get_color(highlight_state, app.user_config.theme),
))
.border_style(get_color(highlight_state, app.user_config.theme));

f.render_widget(title_block, layout_chunk);

let (item_id, name, duration_ms) = match track_item {
PlayingItem::Track(track) => (
track.id.to_owned().unwrap_or_else(|| "".to_string()),
track.name.to_owned(),
track.duration_ms,
),
PlayingItem::Episode(episode) => (
episode.id.to_owned(),
episode.name.to_owned(),
episode.duration_ms,
),
};

let track_name = if app.liked_song_ids_set.contains(&item_id) {
format!("{}{}", &app.user_config.padded_liked_icon(), name)
} else {
name
};

let play_bar_text = match track_item {
PlayingItem::Track(track) => create_artist_string(&track.artists),
PlayingItem::Episode(episode) => format!("{} - {}", episode.name, episode.show.name),
};

let lines = Text::from(Span::styled(
play_bar_text,
Style::default().fg(app.user_config.theme.playbar_text),
));

let artist = Paragraph::new(lines)
.style(Style::default().fg(app.user_config.theme.playbar_text))
.block(
Block::default().title(Span::styled(
&track_name,
Style::default()
  .fg(app.user_config.theme.selected)
  .add_modifier(Modifier::BOLD),
)),
);
f.render_widget(artist, chunks[0]);

let progress_ms = match app.seek_ms {
Some(seek_ms) => seek_ms,
None => app.song_progress_ms,
};

let perc = get_track_progress_percentage(progress_ms, duration_ms);

let song_progress_label = display_track_progress(progress_ms, duration_ms);
let modifier = if app.user_config.behavior.enable_text_emphasis {
Modifier::ITALIC | Modifier::BOLD
} else {
Modifier::empty()
};
let song_progress = Gauge::default()
.gauge_style(
Style::default()
.fg(app.user_config.theme.playbar_progress)
.bg(app.user_config.theme.playbar_background)
.add_modifier(modifier),
)
.percent(perc)
.label(Span::styled(
&song_progress_label,
Style::default().fg(app.user_config.theme.playbar_progress_text),
));
f.render_widget(song_progress, chunks[2]);
}
}
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