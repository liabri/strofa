// use crate::block::{ Blocks, BlockKind, MainBlock, Library, Queue, SelectableList, Playlists, Search, Sort, Playbar };
use crate::block::{ Blocks, BlockKind, BlockTrait, IndexedBlock, Playlists, Library };
use crate::chunk::Chunks;
use crate::event::Key;
use crate::theme::Theme;
use crate::client::StrofaClient;
use crate::key::KeyBindings;

use tui::backend::Backend;
use anyhow::Result;
use tui::layout::Rect;
use mpd_client::Client;

pub struct State<B> {
    pub chunks: Chunks<B>,
    pub blocks: Blocks<B>,
    pub size: Rect,
    pub theme: Theme,
    pub keys: KeyBindings,
    pub client: Client
}

impl<B: 'static + Backend + Send> State<B> {
    pub async fn new(client: Client) -> Result<Self> {
        Ok(Self {
            chunks: Chunks::<B>::new().await?,
            blocks: Blocks::new(&client).await?,
            size: Rect::default(),
            theme: Theme::default(),
            keys: KeyBindings::default(),
            client,
        })
    }   
}