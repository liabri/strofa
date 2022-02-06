#![feature(async_stream)]
#![feature(if_let_guard)]

mod state;
use state::State;

mod block;
use block::{ Blokka };

mod event;
mod theme;

use anyhow::Result;

use tui::{
    backend::{ Backend, CrosstermBackend },
    layout::{ Layout, Constraint, Direction },
    Terminal
};

use crossterm::{
    ExecutableCommand,
    execute,
    terminal::{ enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen, SetTitle },
    cursor::MoveTo
};

use futures_util::StreamExt;
// use tracing_subscriber::{ EnvFilter, FmtSubscriber };
use mpd_client::{ Client, Subsystem, commands };
use tokio::net::TcpStream;

pub const SMALL_TERMINAL_WIDTH: u16 = 150;
pub const SMALL_TERMINAL_HEIGHT: u16 = 45;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    // FmtSubscriber::builder()
    //     .with_env_filter(EnvFilter::from_default_env())
    //     .init();

    let connection = TcpStream::connect("localhost:6600").await?;
    // let connection = UnixSocket::connect("/run/user/1000/mpd").await?;
    let (client, mut state_changes) = Client::connect(connection).await?;


    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    enable_raw_mode()?;

    let mut backend = CrosstermBackend::new(&stdout);
    backend.execute(SetTitle("strofa"))?;

    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    let mut state = State::new();
    let events = event::Events::new();
    futures_util::pin_mut!(events);

    loop {
        if let Ok(size) = terminal.backend().size() {
            state.size = size;
        }

        terminal.draw(|f| match state.blocks.active {
            // Blokka::Error => ui::draw_error_screen(&mut f, &app),
            _ => {
                let margin = if state.size.height > SMALL_TERMINAL_HEIGHT {
                    1
                } else {
                    0
                };

                let constraints = //if state.size.width > SMALL_TERMINAL_WIDTH {
                    // vec![Constraint::Min(1), Constraint::Length(6)]
                // } else {
                    vec![Constraint::Length(3), Constraint::Min(1), Constraint::Length(6)]
                // };
                ;

                let parent_layout = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints(constraints.as_ref())
                    .margin(margin)
                    .split(f.size());

                block::top(f, &state, parent_layout[0]);
                block::centre(f, &state, parent_layout[1]);
                block::bottom(f, &state, parent_layout[2]);
            }
        })?;

        if state.blocks.active==Some(Blokka::Search) {
            terminal.show_cursor()?;
            terminal.backend_mut().execute(MoveTo(
              2 + state.blocks.search.cursor_position,
              2,
            ))?;
        } else {
            terminal.hide_cursor()?;
        }

        match events.next().await {
            Some(event::Event::Input(key)) => {

                if state.blocks.active==Some(Blokka::Search) {
                    if let event::Key::Char(_) = key {
                        state.blocks.active_event(key);
                        continue;
                    };
                }

                match key {
                    event::Key::Esc => state.blocks.active=None,
                    event::Key::Ctrl('c') => break,

                    _ if let Some(cmd) = state.keys.0.get(&key) => {

                        //move this match into either State or Blocks
                        match cmd.as_str() {
                            "to_queue" => {
                                let songs = client.command(commands::Queue).await.unwrap();
                                state.blocks.set_main(block::MainBlock::Tracks(block::Tracks::new(&block::TrackKind::Queue, songs)));
                            },
                            "search" => state.blocks.set_active(Blokka::Search),
                            _ => {},
                        } 
                    },

                    _ => {
                        if let None = state.blocks.active {
                            state.blocks.hovered_event(key);
                        } else {
                            state.blocks.active_event(key); 
                        }
                    }
                }
            }
        
            Some(event::Event::Tick) => {

                loop {
                    println!("hello there");

                    match state_changes.next().await.transpose()? {
                        None => {},//break 'outer,             // connection was closed by the server
                        Some(Subsystem::Player) => { println!("pppppp"); break }, // something relevant changed
                        Some(_) => continue,              // something changed but we don't care
                    }
                }
                // if let Some(CurrentlyPlaybackContext {
                //     item: Some(item),
                //     progress_ms: Some(progress_ms),
                //     is_playing,
                //     ..
                // }) = &self.current_playback_context {
                //   // Update progress even when the song is not playing,
                //   // because seeking is possible while paused
                //   let elapsed = if *is_playing {
                //     self
                //       .instant_since_last_current_playback_poll
                //       .elapsed()
                //       .as_millis()
                //   } else {
                //     0u128
                //   } + u128::from(*progress_ms);

                //   let duration_ms = match item {
                //     PlayingItem::Track(track) => track.duration_ms,
                //     PlayingItem::Episode(episode) => episode.duration_ms,
                //   };

                //   if elapsed < u128::from(duration_ms) {
                //     self.song_progress_ms = elapsed;
                //   } else {
                //     self.song_progress_ms = duration_ms.into();
                //   }
                // }
            }

            None => {}
        }
    }

    // close strofa
    terminal.show_cursor()?;
    disable_raw_mode()?;
    execute!(&stdout, LeaveAlternateScreen)?;

    Ok(())
}
