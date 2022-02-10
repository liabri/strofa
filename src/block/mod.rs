// mod albums;
// pub use albums::{ Albums, AlbumKind };

// mod artists;
// pub use artists::Artists;

mod library;
pub use library::Library;

// mod playbar;
// pub use playbar::Playbar;

mod playlists;
pub use playlists::Playlists;

// mod podcasts;
// pub use podcasts::Podcasts;

// mod search;
// pub use search::Search;
// pub use search::SearchResults;

// mod sort;
// pub use sort::Sort;

// mod tracks;
// pub use tracks::{ Tracks, TrackKind };

// mod queue;
// pub use queue::Queue;

use crate::{ Element, Render }; 
use crate::state::State;
use crate::theme::get_color;

use mpd_client::commands::responses::{ Song, SongInQueue, Playlist, PlayState };
use mpd_client::{ Client, commands };
use anyhow::Result;

use tui::{
    backend::Backend,
    layout::{ Constraint, Direction, Layout, Rect },
    style::{ Modifier, Style },
    text::{ Span, Text },
    widgets::{ Block, Borders, BorderType, List, ListItem, ListState, Paragraph, Row, Table },
    Frame,
};



// trying funky stuff down here 
// maybe move blocks into chunks rather than a `blocks` struct directly.


use async_trait::async_trait;
use std::marker::PhantomData;
use crate::event::Key;

pub struct Blocks<B> {    
    // pub search: StandardBlock<Search>,
    // pub sort: IndexedBlock<Sort>,
    pub library: IndexedBlock<Library>,
    pub playlists: IndexedBlock<Playlists>,
    // pub main_block: Box<dyn BlockTrait>,
    // pub playbar: StandardBlock<Playbar>,
    // pub popup_block: Option<BlokkaK<dyn Popup>>, 
    pub active: Option<BlockKind>,//Option<&BlokkaK<T>>,
    pub hovered: BlockKind,//&BlokkaK<U>,
    pub hover_history: VecDeque<BlockKind>,
    _backend: PhantomData<B>,
}

#[derive(Copy, Clone, PartialEq)]
pub enum BlockKind {
    Search,
    Sort,
    Library,
    Playlists,
    Playbar,
    Main
}

impl<B: Backend> Blocks<B> {
    pub async fn new(client: &Client) -> Result<Self> {
        Ok(Self {
            // search: Search::default(),
            // sort: Sort::new().await,
            library: IndexedBlock::<Library>::new().await?,
            playlists: IndexedBlock::<Playlists>::new(client).await?,
            // playbar: Playbar::new(client).await,
            // main: MainBlock::Queue(Queue::new(client).await?),
            active: None,
            hovered: BlockKind::Library,
            hover_history: VecDeque::new(),
            _backend: PhantomData,
        })
    }

    pub fn get_block_mut(&mut self, kind: BlockKind) -> &mut dyn BlockTrait<B> {
        match kind {
            // BlockKind::Playlists => (&mut self.playlists) as _,
            // _ => (&mut self.playlists) as _,
            _ => todo!()
        }
    }

    pub fn is_hovered(&self, blk: BlockKind) -> bool {
        if self.hovered==blk { return true; } false
    }

    pub fn is_active(&self, blk: BlockKind) -> bool {
        if self.active==Some(blk) { return true; } false
    } 

    pub fn active_block(&mut self) -> Option<&mut dyn BlockTrait<B>> {
        Some(self.get_block_mut(self.active?))
    }

    pub fn hovered_block(&mut self) -> &mut dyn BlockTrait<B> {
        self.get_block_mut(self.hovered)
    }

    // pub pub fn set_main(&mut self, blk: MainBlock) {
    //     self.main = blk;
    //     self.set_active(Blokka::Main);
    // }

    pub fn set_active(&mut self, blk: BlockKind) {
        self.active = Some(blk);
        self.hovered = blk;
    }

    pub fn set_hover(&mut self, blk: BlockKind) {
        self.hover_history.truncate(5);
        self.hover_history.push_front(self.hovered.clone());
        self.hovered = blk.clone();
    } 
}

#[async_trait]
pub trait BlockTrait<B: Backend> {
    async fn active_event(state: &mut State<B>, key: Key) where Self: Sized;
    async fn hovered_event(state: &mut State<B>, key: Key) where Self: Sized;
}



pub struct StandardBlock<T> {
    inner: T
}

pub struct IndexedBlock<T> {
    index: Index,
    inner: T
}


// pub struct PopupBlock<T> {
//     inner: T
// }

// impl<T> PopupBlock<T> {
//     pub fn new(inner: T) -> Self {
//         Self {
//             inner
//         }
//     }
// }












//----------------------

use std::collections::VecDeque;


// pub struct Blocks {    
//     pub search: Search,
//     pub sort: Sort,
//     pub library: Library,
//     pub playlists: Playlists,
//     pub playbar: Playbar,
//     pub main: MainBlock,
//     pub active: Option<Blokka>,
//     pub hovered: Blokka,
//     pub hover_history: VecDeque<Blokka>,
// }

// impl Blocks {
//     pub async fn new(client: &Client) -> Result<Self> {
//         Ok(Self {
//             search: Search::default(),
//             sort: Sort::new().await,
//             library: Library::new().await,
//             playlists: Playlists::new(client).await?,
//             playbar: Playbar::new(client).await,
//             main: MainBlock::Queue(Queue::new(client).await?),
//             active: None,
//             hovered: Blokka::Library,
//             hover_history: VecDeque::new() 
//         })
//     }

//     pub fn is_hovered(&self, blk: Blokka) -> bool {
//         if self.hovered==blk { return true; } false
//     }

//     pub fn is_active(&self, blk: Blokka) -> bool {
//         if self.active==Some(blk) { return true; } false
//     } 

//     pub fn set_main(&mut self, blk: MainBlock) {
//         self.main = blk;
//         self.set_active(Blokka::Main);
//     }

//     pub fn set_active(&mut self, blk: Blokka) {
//         self.active = Some(blk);
//         self.hovered = blk;
//     }

//     pub fn set_hover(&mut self, blk: &Blokka) {
//         self.hover_history.truncate(5);
//         self.hover_history.push_front(self.hovered.clone());
//         self.hovered = blk.clone();
//     }  
// }

// #[derive(Copy, Clone, PartialEq)]
// pub enum Blokka {
//     Search,
//     Sort,
//     Library,
//     Playlists,
//     Playbar,
//     Main
// }

// pub trait SelectableList {
//     fn index(&mut self) -> &mut Index;
// }

// pub enum MainBlock {
//     SearchResults(SearchResults),
//     Artists(Artists),
//     Albums(Albums),
//     Tracks(Tracks),
//     Podcasts(Podcasts),
//     Queue(Queue)
// }

// impl<B: Backend> Render<B> for MainBlock {
//     fn render(&self, f: &mut Frame<B>, state: &State<B>, layout_chunk: Rect) {
//         match self {
//             MainBlock::SearchResults(x) => x.render(f, state, layout_chunk),
//             MainBlock::Artists(x) => x.render(f, state, layout_chunk),
//             MainBlock::Albums(x) => x.render(f, state, layout_chunk),
//             MainBlock::Tracks(x) => x.render(f, state, layout_chunk),
//             MainBlock::Podcasts(x) => x.render(f, state, layout_chunk),
//             MainBlock::Queue(x) => x.render(f, state, layout_chunk),

//         }
//     }
// }

// impl SelectableList for MainBlock {
//     fn index(&mut self) -> &mut Index {
//         match self {
//             MainBlock::SearchResults(x) => x.index(), 
//             MainBlock::Artists(x) => x.index(), 
//             MainBlock::Albums(x) => x.index(), 
//             MainBlock::Tracks(x) => x.index(), 
//             MainBlock::Podcasts(x) => x.index(), 
//             MainBlock::Queue(x) => x.index(),
//         }
//     }
// }

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

fn selectable_list<B>(f: &mut Frame<B>, state: &State<B>, layout_chunk: Rect, title: &str, items: Vec<ListItem>, highlight_state: (bool, bool), selected_index: Option<usize>) where B: Backend {
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

fn selectable_table<B>(f: &mut Frame<B>, state: &State<B>, layout_chunk: Rect, title: &str, header: &[TableHeaderItem], items: Vec<Vec<String>>, selected_index: usize, highlight_state: (bool, bool)) 
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

fn get_percentage_width(width: u16, percentage: f32) -> u16 {
     let padding = 3;
     let width = width - padding;
     (f32::from(width) * percentage) as u16
}