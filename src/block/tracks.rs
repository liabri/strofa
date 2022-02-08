use super::{ Blokka, State, Render, TableHeaderItem, Main, Index, selectable_table, get_percentage_width };
use mpd_client::{ Client, commands, commands::responses::Song };
use tui::{ backend::Backend, layout::Rect, Frame };
use anyhow::Result;

pub struct Tracks {
    pub index: Index,
    pub kind: String,
    pub tracks: Vec<Song>,
}

pub enum TrackKind {
    Album(String),
    Artist(String),
    Playlist(String),
    Queue,
    All,
}

impl Tracks {
    pub async fn new(kind: TrackKind, client: Client) -> Result<Self> {
        let tracks: Vec<Song> = match kind {
            TrackKind::Playlist(ref name) => client.command(commands::GetPlaylist(name.to_string())).await?,
            TrackKind::Queue => client.command(commands::Queue).await?.into_iter().map(|x| x.song).collect(),
             _ => Vec::new(),
        };

        Ok(Self {
            kind: kind.to_string(),
            index: Index::new(tracks.len()),
            tracks,
        })
    }

    pub async fn play(&self, client: Client, index: usize) {
        // let song = self.tracks.get(index).unwrap().id;
        // client.command(commands::Play::song(song)).await.unwrap();
    }
}

impl<B: Backend> Render<B> for Tracks {
    fn render(&self, f: &mut Frame<B>, state: &State, layout_chunk: Rect) {
        let highlight_state = (
            state.blocks.is_active(Blokka::Main),
            state.blocks.is_hovered(Blokka::Main)
        );

        let items = self.tracks
            .iter()
            .map(|track| { 
                let artists = track.artists();
                let artist = if artists.len() > 0 {
                    artists[0].to_string()
                } else {
                    String::new()
                };         

                //if title is empty, take file name

                vec![
                    String::from("0"),// track.position.0.to_string(), 
                    track.title().unwrap_or("none").to_string(), 
                    artist,
                    track.duration.unwrap_or(std::time::Duration::from_secs(1)).as_secs().to_string()
                ]
            }).collect::<Vec<Vec<String>>>();


        let header =  vec![
            TableHeaderItem { text: "#", width: 3 },
            TableHeaderItem { text: "Title", width: get_percentage_width(layout_chunk.width, 2.0 / 5.0) - 5 },
            TableHeaderItem { text: "Artist", width: get_percentage_width(layout_chunk.width, 2.0 / 5.0) },
            // TableHeaderItem { text: "Album", width: get_percentage_width(layout_chunk.width, 2.0 / 5.0) },
            TableHeaderItem { text: "Length", width: get_percentage_width(layout_chunk.width, 1.0 / 5.0) },
        ];

        selectable_table(
            f,
            state,
            layout_chunk,
            &self.kind,
            &header,
            items,
            self.index.inner,
            highlight_state,
        )
    }
}

impl Main for Tracks {
    fn index(&mut self) -> &mut Index {
        &mut self.index
    }
}

impl std::fmt::Display for TrackKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TrackKind::Album(s) => write!(f, " Album {} ", s),
            TrackKind::Artist(s) => write!(f, " Artist {} ", s),
            TrackKind::Playlist(s) => write!(f, " Playlist ───┤ {} ├", s),
            TrackKind::Queue => write!(f, " Queue "),
            TrackKind::All => write!(f, " Tracks ")
        }
    }
}