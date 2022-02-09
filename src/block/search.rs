use anyhow::Result;
use crate::{ Render, State };
use super::{ Blokka, TableHeaderItem, SelectableList, Index };
use super::{ get_color, get_percentage_width, selectable_table };
use mpd_client::{ Client, commands::{ self, responses::Song } };
use crate::client::StrofaClient;
use tui::{ 
    Frame, backend::Backend, layout::Rect, style::Style, text::{ Span, Text }, 
    widgets::{ Block, Borders, BorderType, ListItem, Paragraph } 
};

#[derive(Default)]
pub struct Search {
    pub cursor_position: u16,
    pub query: String,
}

impl<B: Backend> Render<B> for Search {
    fn render(&self, f: &mut Frame<B>, state: &State<B>, layout_chunk: Rect) {
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

use crate::block::MainBlock;
use crate::event::Key;
impl Search {
    pub async fn active_key_event<B>(state: &mut State<B>, key: Key) where B: Backend {
        match key {
            Key::Enter => { 
                let query = state.blocks.search.query.clone();
                state.blocks.main = MainBlock::SearchResults(SearchResults::new(&state.client, query).await.unwrap());
                state.blocks.set_active(Blokka::Main);
                state.blocks.hovered = Blokka::Main;
            },

            Key::Char(c) => {
                state.blocks.search.query.push(c);
                state.blocks.search.cursor_position+=1;
            },

            Key::Backspace => {
                state.blocks.search.query.pop();
                state.blocks.search.cursor_position-=1;
            }

            _ => {}
        }
    }
}





















pub struct SearchResults {
    pub index: Index,
    pub songs: Vec<Song>
}

impl SelectableList for SearchResults {
    fn index(&mut self) -> &mut Index {
        &mut self.index
    }
}

impl SearchResults {
    pub async fn new(client: &Client, query: String) -> Result<Self> {
        Ok(Self {
            index: Index::new(50),
            songs: client.search(&query).await?
        })
    }
}

impl<B: Backend> Render<B> for SearchResults {
    fn render(&self, f: &mut Frame<B>, state: &State<B>, layout_chunk: Rect) {
        let highlight_state = (
            state.blocks.is_active(Blokka::Main),
            state.blocks.is_hovered(Blokka::Main)
        );

        let items = self.songs
            .iter()
            .map(|song| { 
                let artists = song.artists();
                let artist = if artists.len() > 0 {
                    artists[0].to_string()
                } else {
                    String::new()
                };

                vec![
                    song.title().unwrap_or("none").to_string(), 
                    artist,
                    song.duration.unwrap_or(std::time::Duration::from_secs(1)).as_secs().to_string()
                ]
            }).collect::<Vec<Vec<String>>>();

        let header =  vec![
            TableHeaderItem { text: "Title", width: get_percentage_width(layout_chunk.width, 2.0 / 5.0) - 5 },
            TableHeaderItem { text: "Artist", width: get_percentage_width(layout_chunk.width, 2.0 / 5.0) },
            // TableHeaderItem { text: "Album", width: get_percentage_width(layout_chunk.width, 2.0 / 5.0) },
            TableHeaderItem { text: "Length", width: get_percentage_width(layout_chunk.width, 1.0 / 5.0) },
        ];

        selectable_table(
            f,
            state,
            layout_chunk,
            " Search Results ",
            &header,
            items,
            self.index.inner,
            highlight_state,
        )
    }
}
