mod block;

use std::{ io, thread, time::Duration };

use tui::{
    backend::{ Backend, CrosstermBackend },
    widgets::{Widget, Borders},
    layout::{ Layout, Constraint, Direction, Rect },
    Terminal
};

use crossterm::{
    ExecutableCommand,
    event::{ self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode },
    execute,
    terminal::{ disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen, SetTitle },
};

pub const SMALL_TERMINAL_WIDTH: u16 = 150;
pub const SMALL_TERMINAL_HEIGHT: u16 = 45;

fn main() -> Result<(), io::Error> {
    
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    enable_raw_mode()?;

    let mut backend = CrosstermBackend::new(stdout);
    backend.execute(SetTitle("strofa"));

    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor();

    // let events = event::Events::new(user_config.behavior.tick_rate_milliseconds);

    let state = State::default();

      loop {
        // Get the size of the screen on each loop to account for resize event
           if let Ok(size) = terminal.backend().size() {
                // state.size = size;


            }

            terminal.draw(|f| match state.active_block {
                // ActiveBlock::Error => ui::draw_error_screen(&mut f, &app),
                _ => {
                    let margin = {
                        if state.size.height > SMALL_TERMINAL_HEIGHT {
                            1
                        } else {
                            0
                        }
                    };

                    if state.size.width >= SMALL_TERMINAL_WIDTH {
                        let parent_layout = Layout::default()
                            .direction(Direction::Vertical)
                            .constraints([Constraint::Min(1), Constraint::Length(6)].as_ref())
                            .margin(margin)
                            .split(f.size());

                            // block::routes(f, state, parent_layout[0]);
                            // block::polybar(f, state, parent_layout[1]);
                    } else {
                        let parent_layout = Layout::default()
                            .direction(Direction::Vertical)
                            .constraints([
                                Constraint::Length(3),
                                Constraint::Min(1),
                                Constraint::Length(6),
                            ].as_ref()).margin(margin).split(f.size());

                            // block::search(f, state, parent_layout[0]);
                            // block::draw_routes(f, state, parent_layout[1]);
                            // block::playbar(f, state, parent_layout[2]);
                    }
                }
            })?;

            // match events.next()? {
            // event::Event::Input(key) => {
            // if key == Key::Ctrl('c') {
            // break;
            // }

            // let current_active_block = app.get_current_route().active_block;

            // // To avoid swallowing the global key presses `q` and `-` make a special
            // // case for the input handler
            // if current_active_block == ActiveBlock::Input {
            // handlers::input_handler(key, &mut app);
            // } else if key == app.user_config.keys.back {
            // if app.get_current_route().active_block != ActiveBlock::Input {
            // // Go back through navigation stack when not in search input mode and exit the app if there are no more places to back to

            // let pop_result = match app.pop_navigation_stack() {
            //   Some(ref x) if x.id == RouteId::Search => app.pop_navigation_stack(),
            //   Some(x) => Some(x),
            //   None => None,
            // };
            // if pop_result.is_none() {
            //   break; // Exit application
            // }
            // }
            // } else {
            // handlers::handle_app(key, &mut app);
            // }
            // }
            // event::Event::Tick => {
            // app.update_on_tick();
            // }
            // }
    }
}

pub struct State {
    pub active_block: StrofaBlock,
    pub hovered_block: StrofaBlock,
    pub size: Rect,
}

impl Default for State {
    fn default() -> Self {
        Self {
            active_block: StrofaBlock::Home,
            hovered_block: StrofaBlock::Library,
            size: Rect::default()
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum StrofaBlock {
  Analysis,
  PlayBar,
  AlbumTracks,
  AlbumList,
  ArtistBlock,
  Empty,
  Error,
  HelpMenu,
  Home,
  Input,
  Library,
  MyPlaylists,
  Podcasts,
  EpisodeTable,
  RecentlyPlayed,
  SearchResultBlock,
  SelectDevice,
  TrackTable,
  MadeForYou,
  Artists,
  BasicView,
}


pub const LIBRARY_OPTIONS: [&str; 4] = [
    "Songs",
    "Albums",
    "Artists",
    "Podcasts"
];