use tui::style::{ Color, Style };

#[derive(Copy, Clone, Debug)]
pub struct Theme {
  pub analysis_bar: Color,
  pub analysis_bar_text: Color,
  pub active: Color,
  pub banner: Color,
  pub error_border: Color,
  pub error_text: Color,
  pub hint: Color,
  pub hovered: Color,
  pub inactive: Color,
  pub playbar_background: Color,
  pub playbar_progress: Color,
  pub playbar_progress_text: Color,
  pub playbar_text: Color,
  pub selected: Color,
  pub text: Color,
  pub header: Color,
}

impl Default for Theme {
  fn default() -> Self {
    Theme {
      analysis_bar: Color::LightCyan,
      analysis_bar_text: Color::Reset,
      active: Color::Cyan,
      banner: Color::LightCyan,
      error_border: Color::Red,
      error_text: Color::LightRed,
      hint: Color::Yellow,
      hovered: Color::Magenta,
      inactive: Color::Gray,
      playbar_background: Color::Black,
      playbar_progress: Color::LightCyan,
      playbar_progress_text: Color::LightCyan,
      playbar_text: Color::Reset,
      selected: Color::LightCyan,
      text: Color::Reset,
      header: Color::Reset,
    }
  }
}

pub fn get_color((is_active, is_hovered): (bool, bool), theme: Theme) -> Style {
	match (is_active, is_hovered) {
		(true, _) => Style::default().fg(theme.selected),
		(false, true) => Style::default().fg(theme.hovered),
		_ => Style::default().fg(theme.inactive),
	}
}