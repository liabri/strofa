mod state;
use state::State;

mod event;
mod block;
mod theme;

use anyhow::Result;

use tui::{
    backend::{ Backend, CrosstermBackend },
    layout::{ Layout, Constraint, Direction },
    Terminal
};

use crossterm::{
    ExecutableCommand,
    event::EnableMouseCapture,
    execute,
    terminal::{ enable_raw_mode, EnterAlternateScreen, SetTitle },
};

pub const SMALL_TERMINAL_WIDTH: u16 = 150;
pub const SMALL_TERMINAL_HEIGHT: u16 = 45;

fn main() -> Result<()> {
    
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    enable_raw_mode()?;

    let mut backend = CrosstermBackend::new(stdout);
    backend.execute(SetTitle("strofa"));

    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor();

    let events = event::Events::new();

    let mut state = State::default();

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

                let constraints = if state.size.width > SMALL_TERMINAL_WIDTH {
                    vec![Constraint::Min(1), Constraint::Length(6)]
                } else {
                    vec![Constraint::Length(3), Constraint::Min(1), Constraint::Length(6)]
                };

                let parent_layout = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints(constraints.as_ref())
                    .margin(margin)
                    .split(f.size());

                block::search(f, &state, parent_layout[0]);
                block::home(f, &state, parent_layout[1]);
            }
        })?;

        match events.next().unwrap() {
            event::Event::Input(key) => {
                match key {
                    // event::Key::Esc => handle_escape(state),

                    _ if &key==state.keys.get("jump_to_album")? => {}//handle_jump_to_album(state),
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
                
                    // block events 
                    _ => {
                        // StrofaBlock::Analysis => analysis::handler(key, app),
                        // StrofaBlock::ArtistBlock => artist::handler(key, app),
                        // StrofaBlock::Search => search::handler(key, app),
                        // StrofaBlock::MyPlaylists => playlist::handler(key, app),
                        // StrofaBlock::TrackTable => track_table::handler(key, app),
                        // StrofaBlock::EpisodeTable => episode_table::handler(key, app),
                        // StrofaBlock::HelpMenu => help_menu::handler(key, app),
                        // StrofaBlock::Error => error_screen::handler(key, app),
                        // StrofaBlock::SelectDevice => select_device::handler(key, app),
                        // StrofaBlock::SearchResultBlock => search_results::handler(key, app),
                        // StrofaBlock::AlbumList => album_list::handler(key, app),
                        // StrofaBlock::AlbumTracks => album_tracks::handler(key, app),
                        // StrofaBlock::Library => library::handler(key, app),
                        // StrofaBlock::Empty => empty::handler(key, app),
                        // StrofaBlock::Artists => artists::handler(key, app),
                        // StrofaBlock::Podcasts => podcasts::handler(key, app),
                        // StrofaBlock::PlayBar => playbar::handler(key, app),
                        // StrofaBlock::BasicView => basic_view::handler(key, app),
                        // StrofaBlock::Dialog(_) => dialog::handler(key, app),
                    }
                }
            }
        
            event::Event::Tick => {
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
        }
    }
}