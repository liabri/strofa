use super::{ Blokka, State, Render, TableHeaderItem, Main, Index, selectable_table, get_percentage_width };
use mpd_client::{ Client, commands, commands::responses::SongInQueue };
use tui::{ backend::Backend, layout::Rect, Frame };

pub struct Podcasts {
    pub index: Index,
}

impl Main for Podcasts {
    fn index(&mut self) -> &mut Index {
        &mut self.index
    }
}

impl Podcasts {
    pub async fn new() -> Self {
        Self {
            index: Index::new(50),
        }
    }
}

impl<B: Backend> Render<B> for Podcasts {
    fn render(&self, f: &mut Frame<B>, state: &State<B>, layout_chunk: Rect) {
        let highlight_state = (
            state.blocks.is_active(Blokka::Main),
            state.blocks.is_hovered(Blokka::Main)
        );

        // let items: Vec<ListItem> = Vec::new(); 

        // selectable_list(
        //     f,
        //     state,
        //     layout_chunk,
        //     " Podcasts ",
        //     items,
        //     highlight_state,
        //     Some(self.index.inner)
        // );
    }
}
