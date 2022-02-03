use crate::{ State, StrofaBlock, LIBRARY_OPTIONS };

use tui::{
  backend::Backend,
  layout::{Alignment, Constraint, Direction, Layout, Rect},
  style::{Modifier, Style},
  text::{Span, Spans, Text},
  widgets::{Block, Borders, Clear, Gauge, List, ListItem, ListState, Paragraph, Row, Table, Wrap},
  Frame,
};

pub fn library<B>(f: &mut Frame<B>, state: &State, layout_chunk: Rect) where B: Backend {
    let highlight_state = (
        state.active_block == StrofaBlock::Library,
        state.hovered_block == StrofaBlock::Library,
    );

    selectable_list(
        f,
        state,
        layout_chunk,
        "Library",
        &LIBRARY_OPTIONS,
        highlight_state,
        Some(0)// Some(app.library.selected_index),
    );
}






fn selectable_list<B, S>(f: &mut Frame<B>, state: &State, layout_chunk: Rect, title: &str, items: &[S], highlight_state: (bool, bool), selected_index: Option<usize>) 
where B: Backend, S: std::convert::AsRef<str> {
    let mut state = ListState::default();
    state.select(selected_index);

    let lst_items: Vec<ListItem> = items
        .iter()
        .map(|i| ListItem::new(Span::raw(i.as_ref())))
        .collect();

    let list = List::new(lst_items)
        .block(
            Block::default()
            // .title(Span::styled(
                // title,
                // get_color(highlight_state, app.user_config.theme),
            // )).borders(Borders::ALL)
            // .border_style(get_color(highlight_state, app.user_config.theme)),
        // ).style(Style::default().fg(app.user_config.theme.text))
        // .highlight_style(
        //     get_color(highlight_state, app.user_config.theme).add_modifier(Modifier::BOLD),
        );

    f.render_stateful_widget(list, layout_chunk, &mut state);
}