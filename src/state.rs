use crate::block::Blocks;
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
    pub chunks: Chunks,
    pub blocks: Blocks<B>,
    pub size: Rect,
    pub theme: Theme,
    pub keys: KeyBindings,
    pub client: Client
    // pub _temp: std::marker::PhantomData<B>,
}

impl<B: Backend + Send> State<B> {
    pub async fn new(client: Client) -> Result<Self> {
        Ok(Self {
            chunks: Chunks::new(&client).await?,
            blocks: Blocks::new(&client).await?,
            size: Rect::default(),
            theme: Theme::default(),
            keys: KeyBindings::default(),
            client,
        })
    }   
}