use std::marker::PhantomData;
use tui::layout::{ Direction, Layout, Constraint, Rect };
use tui::backend::Backend;
use tui::Frame;
use crate::state::State;
use crate::Render;
use crate::Element;
use anyhow::Result;

pub struct Top;
pub struct Left;
pub struct Centre;
pub struct Bottom;

pub struct Chunks<B> {
    pub top: Chunk<B, Top>,
    pub centre: Chunk<B, Centre>,
    pub bottom: Chunk<B, Bottom>,
}

impl<B: 'static + Backend + Send> Chunks<B> {
    pub async fn new() -> Result<Self> {
        let mut centre_elements = Vec::new();
        centre_elements.push(Box::new(Chunk::<B, Left>::new(Vec::new())?) as Element<B>);

        Ok(Self{
            top: Chunk::<B, Top>::new(Vec::new())?,
            centre: Chunk::<B, Centre>::new(centre_elements)?,
            bottom: Chunk::<B, Bottom>::new(Vec::new())?,
        })
    }
}

pub struct Chunk<B, T> {
    children: Vec<Element<B>>,
    show: bool,
    _location: PhantomData<T>,
}

impl<B: Backend + Send, T> Chunk<B, T> {
    fn new(children: Vec<Element<B>>) -> Result<Self> {
        Ok(Self {
            children,
            show: true,
            _location: PhantomData,
        })
    }

    fn render_children(&self, f: &mut Frame<B>, state: &State<B>, layout_chunk: Rect) {
        for child in &self.children {
            child.render(f, state, layout_chunk);
        }
    }
}

impl<B: Backend + Send> Render<B> for Chunk<B, Top> {
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

impl<B: 'static + Backend + Send> Render<B> for Chunk<B, Left> {
    fn render(&self, f: &mut Frame<B>, state: &State<B>, layout_chunk: Rect) {
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

impl<B: Backend + Send> Render<B> for Chunk<B, Centre> {
    fn render(&self, f: &mut Frame<B>, state: &State<B>, layout_chunk: Rect) {
        if self.show {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
                .split(layout_chunk);

            self.render_children(f, state, chunks[0]);
            // state.blocks.main.render(f, state, chunks[1]);
        }
    }
}

impl<B: Backend + Send> Render<B> for Chunk<B, Bottom> {
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