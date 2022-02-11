use std::marker::PhantomData;
use tui::layout::{ Direction, Layout, Constraint, Rect };
use crate::block::{ IndexedBlock, BlockTrait, Library, Playlists };
use tui::backend::Backend;
use tui::Frame;
use crate::state::State;
use crate::Render;
use crate::Element;
use anyhow::Result;
use mpd_client::Client;

pub struct Chunks {
    pub top: Chunk<Top>,
    pub centre: Chunk<Centre>,
    pub bottom: Chunk<Bottom>
}

impl Chunks {
    pub async fn new(client: &Client) -> Result<Self> {
        Ok(Self{
            top: Chunk::<Top>::new().await?,
            centre: Chunk::<Centre>::new(client).await?,
            bottom: Chunk::<Bottom>::new().await?,
        })
    }
}

pub struct Top {

}

pub struct Left {
    pub library: IndexedBlock<Library>,
    pub playlists: IndexedBlock<Playlists>
}

pub struct Centre {
    pub left_chunk: Chunk<Left>,
}

pub struct Bottom {

}

pub struct Chunk<T> {
    show: bool,
    inner: T,
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
    fn render(&self, f: &mut Frame<B>, state: &State<B>, layout_chunk: Rect) {
        if self.show {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(90), Constraint::Percentage(10)].as_ref())
                .split(layout_chunk);

            // state.blocks.search.render(f, state, chunks[0]);
            // state.blocks.sort.render(f, state, chunks[1]);
        }
    }
}

impl Chunk<Left> {
    async fn new(client: &Client) -> Result<Self> {
        Ok(Self {
            show: true,
            inner: Left {
                library: IndexedBlock::<Library>::new().await?,
                playlists: IndexedBlock::<Playlists>::new(client).await?
            }
        })
    }
}

impl<B: 'static + Backend + Send> Render<B> for Chunk<Left> {
    fn render(&self, f: &mut Frame<B>, state: &State<B>, layout_chunk: Rect) {
        if self.show {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Percentage(30), 
                    Constraint::Percentage(70)
                ].as_ref())
                .split(layout_chunk);


            self.inner.library.render(f, state, chunks[0]);
            self.inner.playlists.render(f, state, chunks[1]);
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


impl<B: 'static + Backend + Send> Render<B> for Chunk<Centre> {
    fn render(&self, f: &mut Frame<B>, state: &State<B>, layout_chunk: Rect) {
        if self.show {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
                .split(layout_chunk);

            self.inner.left_chunk.render(f, state, chunks[0]);
            // state.blocks.main.render(f, state, chunks[1]);
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
    fn render(&self, f: &mut Frame<B>, state: &State<B>, layout_chunk: Rect) {
        if self.show {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(100)].as_ref())
                .split(layout_chunk);

            // state.blocks.playbar.render(f, state, chunks[0]);
        }
    }
}