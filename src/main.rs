#![feature(async_stream)]
#![feature(if_let_guard)]

mod state;
use state::State;

mod block;
use block::{ StrofaBlock, MainBlock, TrackKind };

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
use mpd_client::{ Client, Subsystem };
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

    let mut state = State::new(client);
    let events = event::Events::new();//std::pin::Pin::new(&mut event::Events::new());
    futures_util::pin_mut!(events);

    loop {
        if let Ok(size) = terminal.backend().size() {
            state.size = size;
        }

        terminal.draw(|f| match state.active_block {
            // StrofaBlock::Error => ui::draw_error_screen(&mut f, &app),
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

        if state.active_block==StrofaBlock::Search {
            terminal.show_cursor()?;
            
            // move cursor to search box
            terminal.backend_mut().execute(MoveTo(
              1 + state.blocks.search.cursor_position,
              1,
            ))?;

        } else {
            terminal.hide_cursor()?;
        }

        match events.next().await {
            Some(event::Event::Input(key)) => {
                match key {
                    event::Key::Esc => state.active_block=StrofaBlock::Empty,
                    event::Key::Ctrl('c') => break,

                    _ if let Some(cmd) = state.keys.0.get(&key) => {
                        match cmd.as_str() {
                            "to_queue" => state.set_active(StrofaBlock::MainBlock(MainBlock::Tracks(TrackKind::Queue))),
                            "search" => state.set_active(StrofaBlock::Search),
                            _ => {},
                        } 
                    },

                    // _ if &key==state.keys.get("jump_to_album")? => handle_jump_to_album(state),
                    // _ if &key==state.keys.get("jump_to_artist_album")? => handle_jump_to_artist_album(state),
                    // _ if &key==state.keys.get("jump_to_context")? => handle_jump_to_context(state),
                    // _ if &key==state.keys.get("manage_devices")? => state.dispatch(IoEvent::GetDevices),                
                    // _ if &key==state.keys.get("decrease_volume")? => state.decrease_volume(),
                    // _ if &key==state.keys.get("increase_volume")? => state.increase_volume(),
                    // _ if &key==state.keys.get("toggle_playback")? => state.toggle_playback(), 
                    // _ if &key==state.keys.get("seek_backwards")? => state.seek_backwards(),
                    // _ if &key==state.keys.get("seek_forwards")? => state.seek_forwards(),
                    // _ if &key==state.keys.get("next_track")? => state.dispatch(IoEvent::NextTrack),
                    // _ if &key==state.keys.get("previous_track")? => state.previous_track(),
                    // _ if &key==state.keys.get("help")? => state.set_current_route_state(Some(StrofaBlock::HelpMenu), None),
                    // _ if &key==state.keys.get("shuffle")? => state.shuffle(),
                    // _ if &key==state.keys.get("repeat")? => state.repeat(),
                    // _ if &key==state.keys.get("search")? => state.set_current_route_state(Some(StrofaBlock::Input), Some(StrofaBlock::Input)),
                    // _ if &key==state.keys.get("copy_song_url")? => state.copy_song_url(),
                    // _ if &key==state.keys.get("copy_album_url")? => state.copy_album_url(),
                    // _ if &key==state.keys.get("audio_analysis")? => state.get_audio_analysis(),
                    // _ if &key==state.keys.get("basic_view")? => state.push_navigation_stack(RouteId::BasicView, StrofaBlock::BasicView),
                
                    _ => {
                        let active_block = state.active_block.clone();
                        let hovered_block = state.hovered_block.clone();

                        if active_block!=StrofaBlock::Empty {
                            active_block.active_event(key, &mut state);
                        } else {
                            hovered_block.hovered_event(key, &mut state);
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
