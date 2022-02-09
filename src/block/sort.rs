use super::{ Blokka, State, Render, Index, get_color };
use tui::{ 
    Frame,
    backend::Backend, 
    layout::{ Rect }, 
    style::Style, 
    text::{Span, Text}, 
    widgets::{ Block, Borders, BorderType, Paragraph } 
};

pub struct Sort {
    pub entries: [&'static str; 2],
    pub asc: bool,
    pub index: Index 
}

impl Sort {
    pub async fn new() -> Self {
        Self {
            entries: [
                "Date of Release",
                "Language",
            ],
            asc: true,
            index: Index::new(2),
        }        
    }
}

impl<B: Backend> Render<B> for Sort {
    fn render(&self, f: &mut Frame<B>, state: &State<B>, layout_chunk: Rect) {
        let highlight_state = (
            state.blocks.is_active(Blokka::Sort),
            state.blocks.is_hovered(Blokka::Sort)
        );

        let block = Block::default()
            .title(Span::styled(" Sort By ", Style::default().fg(state.theme.text)))
            .borders(Borders::ALL)
            .border_style(get_color(highlight_state, state.theme))
            .border_type(BorderType::Rounded);

        let lines = Text::from(self.entries[0]);
        let sort = Paragraph::new(lines)
            .block(block)
            .style(get_color(highlight_state, state.theme));

        f.render_widget(sort, layout_chunk);
    }
}

use crate::event::Key;
impl Sort {
    pub async fn active_key_event<B>(state: &mut State<B>, key: Key) where B: Backend {}

    pub async fn hovered_key_event<B>(state: &mut State<B>, key: Key) where B: Backend {
        match key {
            Key::Left => state.blocks.set_hover(&Blokka::Search),
            Key::Down => state.blocks.set_hover(&Blokka::Main),
            _ => {},
        }
    }
}