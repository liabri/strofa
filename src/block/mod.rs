// mod search;
// pub use search::Search;
// pub use search::SearchResults;

// mod sort;
// pub use sort::Sort;

mod library;
pub use library::Library;

mod playlists;
pub use playlists::Playlists;

// mod playbar;
// pub use playbar::Playbar;

// mod podcasts;
// pub use podcasts::Podcasts;

// mod tracks;
// pub use tracks::{ Tracks, TrackKind };

// mod queue;
// pub use queue::Queue;

// mod albums;
// pub use albums::{ Albums, AlbumKind };

// mod artists;
// pub use artists::Artists;

use crate::Render; 
use crate::state::State;
use crate::theme::get_color;
use crate::event::Key;

use std::collections::VecDeque;
use async_trait::async_trait;
use std::marker::PhantomData;
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

pub struct Blocks<B> {    
    // pub search: StandardBlock<Search>,
    // pub sort: IndexedBlock<Sort>,
    pub library: IndexedBlock<Library>,
    pub playlists: IndexedBlock<Playlists>,
    // pub main_block: Box<dyn BlockTrait>,
    // pub playbar: StandardBlock<Playbar>,
    // pub popup_block: Option<BlokkaK<dyn Popup>>, 
    pub active: Option<BlockKind>,
    pub hovered: BlockKind,
    pub hover_history: VecDeque<BlockKind>,
    _backend: PhantomData<B>,
}

impl<B: Send + Backend> Blocks<B> {
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
            BlockKind::Library => &mut self.library as &mut dyn BlockTrait<B>,
            BlockKind::Playlists => &mut self.playlists as &mut dyn BlockTrait<B>,
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

    pub async fn active_event(state: &mut State<B>, key: Key) {
        match state.blocks.active {
            // Some(BlockKind::Search) => Search::active_key_event(self, key).await,
            // Some(BlockKind::Sort) => Sort::active_key_event(self, key).await,
            Some(BlockKind::Library) => IndexedBlock::<Library>::active_event(state, key).await,
            Some(BlockKind::Playlists) => IndexedBlock::<Playlists>::active_event(state, key).await,

        //     Some(BlockKind::Main) => { 
        //         match key {
        //             Key::Up => self.blocks.main.index().dec(),
        //             Key::Down => self.blocks.main.index().inc(),
        //             _ => {}
        //         }

        //         match &self.blocks.main {
        //             MainBlock::Tracks(x) => {
        //                 match key {
        //                     Key::Enter => x.play(&self.client, x.index.inner).await,
        //                     // Key::Char('A') => self.client.add_song_to_playlist(x.songs.get(x.index.inner).unwrap()).await
        //                     _ => {}
        //                 }
        //             },

        //             MainBlock::Queue(x) => {
        //                 // x.active_key_event(self, key);
        //                 // match key {
        //                 //     Key::Enter => x.play(self.client.clone(), x.index.inner).await,
        //                 //     Key::Char('c') => self.client.clear_queue().await.unwrap(),
        //                 //     // Key::Char('p') => self.client.proritise_song_in_queue(x.index.inner)
        //                 //     // Key::Char('w') => self.client.move_song_up_in_queue(x.songs.get(x.index.inner).unwrap()).await
        //                 //     // Key::Char('s') => self.client.move_song_down_in_queue(x.songs.get(x.index.inner).unwrap()).await
        //                 //     // Key::Char('A') => self.client.add_song_to_playlist(x.songs.get(x.index.inner).unwrap()).await
        //                 //     // Key::Char('o') => x.jump_to_current_song().await
        //                 //     _ => {}
        //                 // }
        //             },

        //             _ => {}
        //         }
        //     },

            _ => {}
        }
    }

    pub async fn hovered_event(state: &mut State<B>, key: Key) {
        match state.blocks.hovered {
            // BlockKind::Search => Search::hovered_key_event(self, key).await,
            // BlockKind::Sort => Sort::hovered_key_event(self, key).await,
            BlockKind::Library => IndexedBlock::<Library>::hovered_event(state, key).await,
            BlockKind::Playlists => IndexedBlock::<Playlists>::hovered_event(state, key).await,

            // BlockKind::Main => {
            //     match key {
            //         Key::Up => self.blocks.set_hover(&BlockKind::Search),
            //         Key::Left => {
            //             for previous in self.blocks.hover_history.clone().into_iter() {
            //                 if previous==BlockKind::Library || previous==BlockKind::Playlists {
            //                     self.blocks.set_hover(&previous);
            //                     return;
            //                 }
            //             }

            //             self.blocks.set_hover(&BlockKind::Library)
            //         },

            //         Key::Right => self.blocks.set_hover(&BlockKind::Sort),
            //         Key::Down => {
            //             self.blocks.set_active(BlockKind::Main);
            //             self.blocks.main.index().inc();
            //         },

            //         _ => {},
            //     }
            // },

            _ => {}   
        }

        // common behaviour
        match key {
            Key::Enter => state.blocks.set_active(state.blocks.hovered),
            _ => {}
        }
    }   
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

// #[derive(Copy, Clone, PartialEq)]
// pub enum BlockPositions {
//     TopLeft,
//     TopRight,
//     LeftTop,
//     LeftBottom,
//     Bottom,
//     Centre
// }

// impl BlockPositions {
//     pub fn event(state: &mut State) {
        
//     }
// }

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