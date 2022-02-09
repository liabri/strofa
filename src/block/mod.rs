mod albums;
pub use albums::{ Albums, AlbumKind };

mod artists;
pub use artists::Artists;

mod library;
pub use library::Library;

mod playbar;
pub use playbar::Playbar;

mod playlists;
pub use playlists::Playlists;

mod podcasts;
pub use podcasts::Podcasts;

mod search;
pub use search::Search;
pub use search::SearchResults;

mod sort;
pub use sort::Sort;

mod tracks;
pub use tracks::{ Tracks, TrackKind };

mod queue;
pub use queue::Queue;

pub use crate::state::State;
pub use crate::theme::get_color;
pub use crate::StrofaBackend;

pub use mpd_client::commands::responses::{ Song, SongInQueue, Playlist, PlayState };
pub use mpd_client::{ Client, commands };
use anyhow::Result;

pub use tui::{
    backend::Backend,
    layout::{ Constraint, Direction, Layout, Rect },
    style::{ Modifier, Style },
    text::{ Span, Text },
    widgets::{ Block, Borders, BorderType, List, ListItem, ListState, Paragraph, Row, Table },
    Frame,
};

pub trait Render<B: Backend> {
    fn render(&self, f: &mut Frame<B>, state: &State, layout_chunk: Rect);
}

//eventually move key events from state to each individual block.
// pub trait KeyEvent {
// 	async fn active_event(&self, key: Key);
//     async fn hovered_event(&self, key: Key) {
// }

#[derive(Copy, Clone, PartialEq)]
pub enum Blokka {
    Search,
    Sort,
    Library,
    Playlists,
    Playbar,
    // Standard,
    // Popup,
    Main
}

pub enum Popup {
	Help,
	Error
}

use std::marker::PhantomData;
use tui::backend::CrosstermBackend;
use std::io::Stdout;

pub type Element = Box<dyn Render<StrofaBackend>>;

pub struct Chunks {
	pub top: Chunk<Top>,
	pub centre: Chunk<Centre>,
	pub bottom: Chunk<Bottom>,
}

impl Chunks {
	pub async fn new() -> Result<Self> {
		let mut centre_elements = Vec::new();
		centre_elements.push(Box::new(Chunk::<Left>::new(Vec::new())?) as Element);

		Ok(Self{
			top: Chunk::<Top>::new(Vec::new())?,
			centre: Chunk::<Centre>::new(centre_elements)?,
			bottom: Chunk::<Bottom>::new(Vec::new())?,
		})
	}
}

pub struct Chunk<T> {
    children: Vec<Element>,
    show: bool,
    _kind: PhantomData<T>,
}

impl<T> Chunk<T> {
	fn new(children: Vec<Element>) -> Result<Self> {
		Ok(Self {
			children,
			show: true,
			_kind: PhantomData,
		})
	}

	// fn render_children<B>(&self, f: &mut Frame<B>, state: &State, layout_chunk: Rect) where B: Backend {
	// 	for child in self.children {
	// 		child.render(&mut (&mut f as Frame<StrofaBackend>), state, layout_chunk);
	// 	}
	// }
}

pub struct Top;
pub struct Left;
pub struct Centre;
pub struct Bottom;

impl<B: Backend> Render<B> for Chunk<Top> {
	fn render(&self, f: &mut Frame<B>, state: &State, layout_chunk: Rect) {
		if self.show {
		    let chunks = Layout::default()
		        .direction(Direction::Horizontal)
		        .constraints([Constraint::Percentage(90), Constraint::Percentage(10)].as_ref())
		        .split(layout_chunk);

		    state.blocks.search.render(f, state, chunks[0]);
		    state.blocks.sort.render(f, state, chunks[1]);
		}
	}
}

impl<B: Backend> Render<B> for Chunk<Left> {
	fn render(&self, f: &mut Frame<B>, state: &State, layout_chunk: Rect) {
		if self.show {
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
	}
}

impl<B: Backend> Render<B> for Chunk<Centre> {
	fn render(&self, f: &mut Frame<B>, state: &State, layout_chunk: Rect) {
		if self.show {
		    let chunks = Layout::default()
		        .direction(Direction::Horizontal)
		        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
		        .split(layout_chunk);

		    // self.render_children(f, state, chunks[0]);
		    state.blocks.main.render(f, state, chunks[1]);
		}
	}
}

impl<B: Backend> Render<B> for Chunk<Bottom> {
	fn render(&self, f: &mut Frame<B>, state: &State, layout_chunk: Rect) {
		if self.show {
		    let chunks = Layout::default()
		        .direction(Direction::Horizontal)
		        .constraints([Constraint::Percentage(100)].as_ref())
		        .split(layout_chunk);

		    state.blocks.playbar.render(f, state, chunks[0]);
		}
	}
}








pub struct Index {
    pub inner: usize,
    max: usize,
}

impl Index {
    pub fn new(max: usize) -> Self {
        Index {
            inner: 0,
            max: max,
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

pub struct TableHeaderItem<'a> {
    text: &'a str,
    width: u16
}

fn selectable_table<B>(f: &mut Frame<B>, state: &State, layout_chunk: Rect, title: &str, header: &[TableHeaderItem], items: Vec<Vec<String>>, selected_index: usize, highlight_state: (bool, bool)) 
where B: Backend {
    let widths = header
        .iter()
        .map(|h| Constraint::Length(h.width))
        .collect::<Vec<tui::layout::Constraint>>();

    let padding = 5;
    let offset = layout_chunk
        .height
        .checked_sub(padding)
        .and_then(|height| selected_index.checked_sub(height as usize))
        .unwrap_or(0);

    let colour = get_color(highlight_state, state.theme);
    let rows = items.iter().skip(offset).enumerate().map(|(i, item)| {
        let formatted_row = item.clone();
        let mut style = Style::default().fg(state.theme.text);

        if Some(i) == selected_index.checked_sub(offset) {
            style = colour.add_modifier(Modifier::BOLD);
        }

        Row::new(formatted_row).style(style)
    });

    let table = Table::new(rows)
        .header(Row::new(header.iter().map(|h| h.text))
            .style(Style::default().fg(state.theme.header)))
        .block(Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(state.theme.text))
            .title(Span::styled(title, colour))
            .border_style(colour)
            .border_type(BorderType::Rounded))   
        .style(Style::default().fg(state.theme.text))
        .widths(&widths);

    f.render_widget(table, layout_chunk);
}

pub fn get_percentage_width(width: u16, percentage: f32) -> u16 {
 	let padding = 3;
 	let width = width - padding;
 	(f32::from(width) * percentage) as u16
}

pub trait Main {
    fn index(&mut self) -> &mut Index;
}

pub enum MainBlock {
    SearchResults(SearchResults),
    Artists(Artists),
    Albums(Albums),
    Tracks(Tracks),
    Podcasts(Podcasts),
    Queue(Queue)
}

impl<B: Backend> Render<B> for MainBlock {
    fn render(&self, f: &mut Frame<B>, state: &State, layout_chunk: Rect) {
        match self {
            MainBlock::SearchResults(x) => x.render(f, state, layout_chunk),
            MainBlock::Artists(x) => x.render(f, state, layout_chunk),
            MainBlock::Albums(x) => x.render(f, state, layout_chunk),
            MainBlock::Tracks(x) => x.render(f, state, layout_chunk),
            MainBlock::Podcasts(x) => x.render(f, state, layout_chunk),
            MainBlock::Queue(x) => x.render(f, state, layout_chunk),

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
            MainBlock::Queue(x) => x.index(),
        }
    }
}