use std::marker::PhantomData;
use tui::layout::{ Direction, Layout, Constraint, Rect };
use crate::block::{ IndexedBlock, BlockTrait, Library, Playlists };
use tui::backend::Backend;
use tui::Frame;
use crate::state::State;
use crate::Render;
use anyhow::Result;
use mpd_client::Client;
use std::collections::VecDeque;
use crate::event::Key;

//move hover events to chunks/blocks

pub struct Chunks {
    pub top: Chunk<Top>,
    pub centre: Chunk<Centre>,
    pub bottom: Chunk<Bottom>,
    pub active: Option<BlockKind>,
    pub hovered: BlockKind,
    pub hover_history: VecDeque<BlockKind>,
}

#[derive(Copy, Clone, PartialEq)]
pub enum BlockKind {
    TopLeft,
    TopRight,
    LeftTop,
    LeftBottom,
    Bottom,
    Centre
}

impl BlockKind {
    pub fn event<B>(&self, state: &mut State) where B: Backend + Send {
        
    }
}

impl Chunks {
    pub async fn new(client: &Client) -> Result<Self> {
        Ok(Self {
            top: Chunk::<Top>::new().await?,
            centre: Chunk::<Centre>::new(client).await?,
            bottom: Chunk::<Bottom>::new().await?,
            active: None,
            hovered: BlockKind::LeftTop,
            hover_history: VecDeque::new()
        })
    }

    pub fn set_active(&mut self, blk: BlockKind) {
        self.active = Some(blk);
        self.hovered = blk;
    }

    pub fn set_hover(&mut self, blk: BlockKind) {
        self.hover_history.truncate(5);
        self.hover_history.push_front(self.hovered.clone());
        self.hovered = blk.clone();
    }

    pub fn is_hovered(&self, blk: BlockKind) -> bool {
        if self.hovered==blk { return true; } false
    }

    pub fn is_active(&self, blk: BlockKind) -> bool {
        if self.active==Some(blk) { return true; } false
    } 

    pub async fn active_event(state: &mut State, key: Key) {
        match state.chunks.active {
            Some(BlockKind::LeftTop) => IndexedBlock::<Library>::active_event(state, key).await,
            Some(BlockKind::LeftBottom) => IndexedBlock::<Playlists>::active_event(state, key).await,
            _ => {}
        }

        match key {
            Key::Esc => state.chunks.active=None,
            _ => {}
        }
    }

    pub async fn hovered_event(state: &mut State, key: Key) {
        match state.chunks.hovered {
            BlockKind::TopLeft => {
                match key {
                    Key::Down => state.chunks.set_hover(BlockKind::LeftTop),
                    Key::Right => state.chunks.set_hover(BlockKind::TopRight),
                    _ => {},
                }

                // IndexedBlock::<Search>::hovered_event(state, key).await;
            }

            BlockKind::TopRight => {
                match key {
                    Key::Down => state.chunks.set_hover(BlockKind::Centre),
                    Key::Left => state.chunks.set_hover(BlockKind::TopLeft),
                    _ => {},
                }

                // IndexedBlock::<Sort>::hovered_event(state, key).await;
            }

            BlockKind::LeftTop => {
                match key {
                    Key::Up => state.chunks.set_hover(BlockKind::TopLeft),
                    Key::Down => state.chunks.set_hover(BlockKind::LeftBottom),
                    Key::Right => state.chunks.set_hover(BlockKind::Centre),
                    _ => {},
                }

                IndexedBlock::<Library>::hovered_event(state, key).await;
            }

            BlockKind::LeftBottom => {
                match key {
                    Key::Up => state.chunks.set_hover(BlockKind::TopLeft),
                    Key::Down => state.chunks.set_hover(BlockKind::LeftBottom),
                    Key::Right => state.chunks.set_hover(BlockKind::Centre),
                    _ => {},
                }

                IndexedBlock::<Playlists>::hovered_event(state, key).await;
            }

            BlockKind::Centre => {
                match key {
                    Key::Up => state.chunks.set_hover(BlockKind::TopLeft),
                    Key::Left => {
                        for previous in &state.chunks.hover_history {
                            if *previous==BlockKind::LeftTop || *previous==BlockKind::LeftBottom {
                                state.chunks.set_hover(*previous);
                                return;
                            }
                        }

                        state.chunks.set_hover(BlockKind::LeftTop)
                    },

                    Key::Right => state.chunks.set_hover(BlockKind::TopRight),
                    Key::Down => {
                        state.chunks.set_active(BlockKind::Centre);
                    },

                    _ => {},
                }

                // IndexedBlock::<dyn Main>::hovered_event(state, key).await;
                // state.chunks.main.index().inc();
            }

            _ => {}
        }

        // MOVE TO MAIN some nice fluidity
        // if let Some(Blokka::Main) = state.chunks.active {
        //     let blk = state.chunks.hover_previous(1).clone();
        //     state.chunks.set_hover(&blk);
        // }

        // common behaviour
        match key {
            Key::Enter => state.chunks.set_active(state.chunks.hovered),
            _ => {}
        }
    }   
}

pub struct Top {

}

pub struct Left {
    pub top: IndexedBlock<Library>,
    pub bottom: IndexedBlock<Playlists>
}

pub struct Centre {
    pub left_chunk: Chunk<Left>,
}

pub struct Bottom {

}

pub struct Chunk<T> {
    pub show: bool,
    pub inner: T,
}

impl Chunk<Top> {
    async fn new() -> Result<Self> {
        Ok(Self {
            show: true,
            inner: Top {}
        })
    }
}

impl<B: Backend + Send> Render<B> for Chunk<Top> {
    fn render(&self, f: &mut Frame<B>, state: &State, layout_chunk: Rect) {
        if self.show {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(90), Constraint::Percentage(10)].as_ref())
                .split(layout_chunk);

            // state.chunks.search.render(f, state, chunks[0]);
            // state.chunks.sort.render(f, state, chunks[1]);
        }
    }
}

impl Chunk<Left> {
    async fn new(client: &Client) -> Result<Self> {
        Ok(Self {
            show: true,
            inner: Left {
                top: IndexedBlock::<Library>::new().await?,
                bottom: IndexedBlock::<Playlists>::new(client).await?
            }
        })
    }
}

impl<B: Backend + Send> Render<B> for Chunk<Left> {
    fn render(&self, f: &mut Frame<B>, state: &State, layout_chunk: Rect) {
        if self.show {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Percentage(30), 
                    Constraint::Percentage(70)
                ].as_ref())
                .split(layout_chunk);


            self.inner.top.render(f, state, chunks[0]);
            self.inner.bottom.render(f, state, chunks[1]);
        }
    }
}

impl Chunk<Centre> {
    async fn new(client: &Client) -> Result<Self> {
        Ok(Self {
            show: true,
            inner: Centre {
                left_chunk: Chunk::<Left>::new(client).await?,
            }
        })
    }
}


impl<B: Backend + Send> Render<B> for Chunk<Centre> {
    fn render(&self, f: &mut Frame<B>, state: &State, layout_chunk: Rect) {
        if self.show {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
                .split(layout_chunk);

            self.inner.left_chunk.render(f, state, chunks[0]);
            // state.chunks.main.render(f, state, chunks[1]);
        }
    }
}

impl Chunk<Bottom> {
    async fn new() -> Result<Self> {
        Ok(Self {
            show: true,
            inner: Bottom {}
        })
    }
}

impl<B: Backend + Send> Render<B> for Chunk<Bottom> {
    fn render(&self, f: &mut Frame<B>, state: &State, layout_chunk: Rect) {
        if self.show {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(100)].as_ref())
                .split(layout_chunk);

            // state.chunks.playbar.render(f, state, chunks[0]);
        }
    }
}