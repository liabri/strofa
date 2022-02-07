use super::{ Blokka, State, Render, Index, get_color, selectable_list };
use mpd_client::{ Client, commands, commands::responses::Playlist };
use tui::{ 
    Frame,
    backend::Backend, 
    layout::{ Rect }, 
    text::Span, 
    widgets::ListItem
};

pub struct Playlists {
   pub entries: Vec<Playlist>,
   pub index: Index 
}

impl Playlists {
    pub async fn new() -> Self {
        Self {
            entries: Vec::new(),
            index: Index::new(50),
        }        
    }
}

impl<B: Backend> Render<B> for Playlists {
    fn render(&self, f: &mut Frame<B>, state: &State, layout_chunk: Rect) {
        let highlight_state = (
            state.blocks.is_active(Blokka::Playlists),
            state.blocks.is_hovered(Blokka::Playlists)
        );

        let items: Vec<ListItem> = self.entries
            .iter()
            .map(|i| ListItem::new(Span::from(i.name.as_str())))
            .collect();

        selectable_list(
            f,
            state,
            layout_chunk,
            " Playlists ",
            items,
            highlight_state,
            Some(self.index.inner)
        );
    }    
}
