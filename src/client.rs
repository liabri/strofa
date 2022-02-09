use async_trait::async_trait;
use anyhow::Result;
use mpd_client::{ Client, CommandError, commands, commands::responses::{ PlayState  }};
use std::time::Duration;

#[async_trait]
pub trait StrofaClient {
    async fn toggle_playback(&self) -> Result<(), CommandError>;
    async fn set_volume(&self, o: i8) -> Result<(), CommandError>;
    async fn next_track(&self) -> Result<(), CommandError>;
    async fn previous_track(&self) -> Result<(), CommandError>;
    async fn seek_forwards(&self, o: u64) -> Result<(), CommandError>;
    async fn seek_backwards(&self, o: u64) -> Result<(), CommandError>;
    async fn toggle_shuffle(&self) -> Result<(), CommandError>;
    async fn toggle_repeat(&self) -> Result<(), CommandError>;
}

#[async_trait]
impl StrofaClient for Client {
    async fn toggle_playback(&self) -> Result<(), CommandError> {
        let status = self.command(commands::Status).await?;
        match status.state {
            PlayState::Stopped => self.command(commands::SetPause(true)).await,
            PlayState::Playing => self.command(commands::SetPause(true)).await,
            PlayState::Paused => self.command(commands::Play::current()).await,
        }
    }

    async fn set_volume(&self, o: i8) -> Result<(), CommandError> {
        let current_volume = self.command(commands::Status).await?.volume;
        let new_volume = (current_volume as i8)+o;
        
        let vol = if new_volume < 0 {
            0
        } else if new_volume > 100 {
            100
        } else {
            new_volume
        };

        self.command(commands::SetVolume(vol.try_into().unwrap())).await
    }

    async fn next_track(&self) -> Result<(), CommandError> {
        self.command(commands::Next).await
    }

    async fn previous_track(&self) -> Result<(), CommandError> {
        self.command(commands::Previous).await
    }

    async fn seek_forwards(&self, o: u64) -> Result<(), CommandError> {
        self.command(commands::Seek(commands::SeekMode::Forward(Duration::from_secs(o)))).await
    }

    async fn seek_backwards(&self, o: u64) -> Result<(), CommandError> {
        self.command(commands::Seek(commands::SeekMode::Backward(Duration::from_secs(o)))).await
    }

    async fn toggle_shuffle(&self) -> Result<(), CommandError> {
        let current_shuffle = self.command(commands::Status).await?.random;
        self.command(commands::SetRandom(!current_shuffle)).await
    }

    async fn toggle_repeat(&self) -> Result<(), CommandError> {
        let current_repeat = self.command(commands::Status).await?.repeat;
        self.command(commands::SetRepeat(!current_repeat)).await
    }

    // async fn jump_to_start(&self, client: Client) {
    //     if let Some(current_song) = &self.song {
    //         let current_id = current_song.id;
    //         let pos = commands::SongPosition(0);
    //         client.command(commands::Move::id(current_id).to_position(pos)).await.unwrap();
    //     }
    // }
}