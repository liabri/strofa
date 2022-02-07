use super::{ Blokka, State, Render, Index, get_color, selectable_list };
use mpd_client::{ Client, commands, commands::responses::Playlist };
use tui::{ 
    Frame,
    backend::Backend, 
    layout::{ Rect }, 
    text::Span, 
    widgets::ListItem
};

pub struct Library {
   pub entries: [&'static str; 5],
   pub index: Index 
}

impl Library {
    pub async fn new() -> Self {
        Self {
            entries: [
                "Queue",
                "Tracks",
                "Albums",
                "Artists",
                "Podcasts"
            ],
            index: Index::new(5),
        }        
    }
}

impl<B: Backend> Render<B> for Library {
    fn render(&self, f: &mut Frame<B>, state: &State, layout_chunk: Rect) {
        let highlight_state = (
            state.blocks.is_active(Blokka::Library),
            state.blocks.is_hovered(Blokka::Library)
        );

        let items: Vec<ListItem> = self.entries
            .into_iter()
            .map(|i| ListItem::new(Span::raw(i)))
            .collect();

        selectable_list(
            f,
            state,
            layout_chunk,
            " Library ",
            items,
            highlight_state,
            Some(self.index.inner)
        );
    }    
}