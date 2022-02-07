use super::{ Blokka, State, Render, TableHeaderItem, Main, Index, get_color, selectable_table, selectable_list, get_percentage_width };
use mpd_client::{ Client, commands, commands::responses::SongInQueue };
use tui::{ 
    Frame,
    backend::Backend, 
    layout::{ Rect }, 
    style::Style, 
    text::{Span, Text}, 
    widgets::{ Block, Borders, BorderType, ListItem, Paragraph } 
};

#[derive(Default)]
pub struct Search {
    pub index: usize,
    pub cursor_position: u16,
    pub query: String,
}

impl<B: Backend> Render<B> for Search {
    fn render(&self, f: &mut Frame<B>, state: &State, layout_chunk: Rect) {
        let highlight_state = (
            state.blocks.is_active(Blokka::Search),
            state.blocks.is_hovered(Blokka::Search)
        );

        let lines = Text::from((&self.query).as_str());
        let search = Paragraph::new(lines).block(
            Block::default()
                .borders(Borders::ALL)
                .title(Span::styled(
                    " Search ",
                    get_color(highlight_state, state.theme),
                )).border_style(get_color(highlight_state, state.theme))
                .border_type(BorderType::Rounded),
        );

        f.render_widget(search, layout_chunk);
    }    
}





//results
pub struct SearchResults {
    pub index: Index,
    pub query: String,
}

impl Main for SearchResults {
    fn index(&mut self) -> &mut Index {
        &mut self.index
    }
}

impl SearchResults {
    pub async fn new(query: String) -> Self {
        Self {
            query,
            index: Index::new(50),
        }
    }
}

impl<B: Backend> Render<B> for SearchResults {
    fn render(&self, f: &mut Frame<B>, state: &State, layout_chunk: Rect) {
        let highlight_state = (
            state.blocks.is_active(Blokka::Main),
            state.blocks.is_hovered(Blokka::Main)
        );

        let items: Vec<ListItem> = Vec::new(); 

        selectable_list(
            f,
            state,
            layout_chunk,
            " Search Results ",
            items,
            highlight_state,
            Some(self.index.inner)
        );
    }
}
