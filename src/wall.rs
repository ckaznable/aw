use fastrand::Rng;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::Color,
    widgets::Widget,
};
use Constraint::*;

pub struct ColorWall;

impl ColorWall {
    fn render_colors(area: Rect, buf: &mut Buffer, color: Color) {
        for x in area.left()..area.right() {
            for y in area.top()..area.bottom() {
                buf.get_mut(x, y).set_char('█').set_fg(color);
            }
        }
    }
}

impl Widget for ColorWall {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Percentage(25),
                Percentage(25),
                Percentage(25),
                Percentage(25),
            ])
            .split(area);

        let mut rng = Rng::new();
        for area in layout.iter().copied() {
            let layout = Layout::default()
                .constraints([Percentage(50), Percentage(50)])
                .split(area);

            Self::render_colors(
                layout[0],
                buf,
                Color::Rgb(rng.u8(0..255), rng.u8(0..255), rng.u8(0..255)),
            );
            Self::render_colors(
                layout[1],
                buf,
                Color::Rgb(rng.u8(0..255), rng.u8(0..255), rng.u8(0..255)),
            );
        }
    }
}
