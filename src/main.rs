#![feature(async_stream)]
#![feature(if_let_guard)]

mod state;
use state::State;

mod block;
use block::{ Blokka, Playbar, Render };

mod event;
mod theme;
mod client;

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

use futures_util::Stream;
use futures_util::StreamExt;
use tracing_subscriber::{ EnvFilter, FmtSubscriber };
use mpd_client::{ Client, Subsystem, commands };
use tokio::net::TcpStream;
use std::io::{ Stdout, stdout };

pub const SMALL_TERMINAL_WIDTH: u16 = 150;
pub const SMALL_TERMINAL_HEIGHT: u16 = 45;

// pub type StrofaBackend = CrosstermBackend<Stdout>;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let connection = TcpStream::connect("localhost:6600").await?;
    // let connection = UnixSocket::connect("/run/user/1000/mpd").await?;
    let (client, mut state_changes) = Client::connect(connection).await?;
    futures_util::pin_mut!(state_changes);


    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    enable_raw_mode()?;

    let mut backend = CrosstermBackend::new(stdout);
    backend.execute(SetTitle("strofa"))?;

    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    let mut state = State::new(client.clone()).await?;
    let events = event::Events::new();
    futures_util::pin_mut!(events);

    loop {
        if let Ok(size) = terminal.backend().size() {
            state.size = size;
        }

        // drawing 
        terminal.autoresize()?;
        terminal.draw(|f| {
            let margin = if state.size.height > SMALL_TERMINAL_HEIGHT { 1 } else { 0 };
            let constraints = //if state.size.width > SMALL_TERMINAL_WIDTH {
                // vec![Constraint::Min(1), Constraint::Length(6)]
            // } else {
                vec![Constraint::Length(3), Constraint::Min(1), Constraint::Length(6)]
                // vec![Constaint::]
            // };
            ;

            let parent_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(constraints.as_ref())
                .margin(margin)
                .split(f.size());


            state.chunks.top.render(f, &state, parent_layout[0]);
            state.chunks.centre.render(f, &state, parent_layout[1]);
            state.chunks.bottom.render(f, &state, parent_layout[2]);
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

        // crossterm events
        if let Some(event::Event::Input(key)) = events.next().await {
            if state.blocks.active==Some(Blokka::Search) {
                if let event::Key::Char(_) = key {
                    state.active_event(key).await;
                    continue;
                };
            }

            match key {
                event::Key::Ctrl('c') => break,
                event::Key::Esc => {

                    // some nice fluidity
                    // if let Some(Blokka::Main) = state.blocks.active {
                    //     let blk = state.blocks.hover_previous(1).clone();
                    //     state.blocks.set_hover(&blk);
                    // }

                    state.blocks.active=None
                },

                _ if let Some(cmd) = state.keys.0.clone().get(&key) => {
                    state.handle_keybind(&cmd.clone()).await?;
                },

                _ => {
                    if let None = state.blocks.active {
                        state.hovered_event(key);
                    } else {
                        state.active_event(key).await; 
                    }
                }
            }
        }

        // mpd events
        match futures::poll!(state_changes.next()) {
            futures::task::Poll::Ready(x) => {
                state.blocks.active=Some(Blokka::Search);

                match x.transpose()? {
                    Some(Subsystem::Player) => state.blocks.playbar=Playbar::new(client.clone()).await, 
                    Some(Subsystem::Queue) => {println!("queue")},
                    Some(Subsystem::StoredPlaylist) => {},
                    Some(Subsystem::Update) => {}
                    Some(Subsystem::Database) => {}
                    _ => { continue; }
                }
            },

            futures::task::Poll::Pending => {}
        }

        // match state_changes.next().await.transpose()? {
        //     Some(Subsystem::Player) => state.blocks.playbar=Playbar::new(client.clone()).await, 
        //     Some(Subsystem::Queue) => {println!("queue")},
        //     Some(Subsystem::StoredPlaylist) => {},
        //     Some(Subsystem::Update) => {}
        //     Some(Subsystem::Database) => {}
        //     _ => { continue; }
        // }        
    }

    // close strofa
    terminal.show_cursor()?;
    disable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, LeaveAlternateScreen)?;

    Ok(())
}
