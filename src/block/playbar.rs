use super::{ State, Render };
use std::time::Duration;
use mpd_client::{ Client, commands, commands::responses::{ SongInQueue, PlayState }};
use tui::{ 
    Frame,
    backend::Backend, 
    layout::{ Rect }, 
    text::Span, 
    style::Style,
    widgets::{ Block, Borders }
};

pub struct Playbar {
    pub song: Option<SongInQueue>,
    pub volume: u8,
    pub shuffle: bool,
    pub repeat: bool,
    // pub history: Vec<SingInQueue>, //depends: do i like mpds current "previous" function ?
}

impl Playbar {
    pub async fn new(client: &Client) -> Self {
        let status = client.command(commands::Status).await.unwrap();

        Self { 
            song: client.command(commands::CurrentSong).await.unwrap(),
            volume: status.volume,
            shuffle: status.random,
            repeat: status.repeat
        }
    }
}

impl<B: Backend> Render<B> for Playbar {
    fn render(&self, f: &mut Frame<B>, state: &State<B>, layout_chunk: Rect) {
        if let Some(song) = &self.song {
            if let Some(title) = song.song.title() {
                let playbar = Block::default()
                    .title(Span::styled(title, Style::default().fg(state.theme.text)))
                    .borders(Borders::NONE);

                f.render_widget(playbar, layout_chunk);  
                return;
            }        
        } 

        let playbar = Block::default()
            .title(Span::styled("poop", Style::default().fg(state.theme.text)))
            .borders(Borders::NONE);

        f.render_widget(playbar, layout_chunk);
    }
}