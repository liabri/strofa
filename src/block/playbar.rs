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
    // pub history: Vec<SingInQueue>,
}

impl Playbar {
    pub async fn new(client: Client) -> Self {
        Self { 
            song: client.command(commands::CurrentSong).await.unwrap(),
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

    pub async fn set_volume(&self, o: i8, client: Client) {
        let current_volume = client.command(commands::Status).await.unwrap().volume;
        let new_volume = (current_volume as i8)+o;
        
        let vol = if new_volume < 0 {
            0
        } else if new_volume > 100 {
            100
        } else {
            new_volume
        };

        client.command(commands::SetVolume(vol.try_into().unwrap())).await.unwrap();
    }

    // pub async fn jump_to_start(&self, client: Client) {
    //     if let Some(current_song) = &self.song {
    //         let current_id = current_song.id;
    //         let pos = commands::SongPosition(0);
    //         client.command(commands::Move::id(current_id).to_position(pos)).await.unwrap();
    //     }
    // }
}

impl<B: Backend> Render<B> for Playbar {
    fn render(&self, f: &mut Frame<B>, state: &State, layout_chunk: Rect) {
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