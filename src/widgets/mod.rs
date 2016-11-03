mod block;
mod text;
mod list;
mod gauge;
mod sparkline;
mod chart;
mod barchart;
mod tabs;
mod table;
pub mod canvas;

pub use self::block::Block;
pub use self::text::Text;
pub use self::list::List;
pub use self::gauge::Gauge;
pub use self::sparkline::Sparkline;
pub use self::chart::{Chart, Axis, Dataset, Marker};
pub use self::barchart::BarChart;
pub use self::tabs::Tabs;
pub use self::table::Table;

use buffer::Buffer;
use layout::Rect;
use terminal::Terminal;
use style::Color;

/// Bitflags that can be composed to set the visible borders essentially on the block widget.
pub mod border {
    bitflags! {
        pub flags Flags: u32 {
            const NONE  = 0b00000001,
            const TOP   = 0b00000010,
            const RIGHT = 0b00000100,
            const BOTTOM = 0b0001000,
            const LEFT = 0b00010000,
            const ALL = TOP.bits | RIGHT.bits | BOTTOM.bits | LEFT.bits,
        }
    }
}

/// Base requirements for a Widget
pub trait Widget {
    /// Draws the current state of the widget in the given buffer. That the only method required to
    /// implement a custom widget.
    fn draw(&self, area: &Rect, buf: &mut Buffer);
    /// Helper method to quickly set the background of all cells inside the specified area.
    fn background(&self, area: &Rect, buf: &mut Buffer, color: Color) {
        for y in area.top()..area.bottom() {
            for x in area.left()..area.right() {
                buf.set_bg(x, y, color);
            }
        }
    }
    /// Helper method that can be chained with a widget's builder methods to render it.
    fn render(&self, area: &Rect, t: &mut Terminal)
        where Self: Sized
    {
        t.render(self, area);
    }
}
