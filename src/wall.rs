use std::rc::Rc;

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

    fn layout(area: Rect) -> Vec<Rc<[Rect]>> {
        let (column, row) = Self::column_and_row(&area);

        Layout::default()
            .direction(Direction::Horizontal)
            .constraints((0..column).map(|_| Ratio(1, column)).collect::<Vec<_>>())
            .split(area)
            .iter()
            .copied()
            .map(|area| Layout::default()
                .constraints((0..row).map(|_| Ratio(1, row)).collect::<Vec<_>>())
                .split(area)
            )
            .collect::<Vec<_>>()
    }

    #[inline]
    fn column_and_row(area: &Rect) -> (u32, u32) {
        (
            Self::limit_size(area.width as u32),
            Self::limit_size(area.height as u32),
        )
    }

    #[inline]
    fn limit_size(len: u32) -> u32 {
        if len < 100 {
            len % 25
        } else {
            len / 25
        }
    }
}

impl Widget for ColorWall {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let layout = Self::layout(area);

        let mut rng = Rng::new();
        for columns in layout.into_iter() {
            for row in columns.iter().copied() {
                let color = Color::Rgb(rng.u8(0..255), rng.u8(0..255), rng.u8(0..255));
                Self::render_colors(row, buf, color);
            }
        }
    }
}
