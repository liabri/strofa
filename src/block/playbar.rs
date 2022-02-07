use super::{ State, Render };
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
}

impl Playbar {
    pub async fn new() -> Self {
        Self { 
            song: None,
        }
    }

    pub async fn toggle(&self, client: Client) {
        let status = client.command(commands::Status).await.unwrap();
        match status.state {
            PlayState::Stopped => client.command(commands::SetPause(true)).await.unwrap(),
            PlayState::Playing => client.command(commands::SetPause(true)).await.unwrap(),
            PlayState::Paused => client.command(commands::Play::current()).await.unwrap(),
        }
    }
}

impl<B: Backend> Render<B> for Playbar {
    fn render(&self, f: &mut Frame<B>, state: &State, layout_chunk: Rect) {
        let playbar = Block::default()
            .title(Span::styled(/*self.current_song.as_ref().unwrap().song.title().unwrap_or("Empty")*/ "pla", Style::default().fg(state.theme.text)))
            .borders(Borders::NONE);

        f.render_widget(playbar, layout_chunk);
    }
}