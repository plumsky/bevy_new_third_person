use super::*;
use std::borrow::Cow;

#[derive(Debug, Clone)]
pub struct Opts {
    // text
    pub text_layout: TextLayout,
    pub text: Cow<'static, str>,
    pub font: TextFont,
    // layout
    pub border_radius: f32,
    pub border_color: Color,
    pub bg_color: Color,
    pub color: Color,
    pub node: Node,
}

#[allow(dead_code)]
impl Opts {
    pub fn new(s: impl Into<Cow<'static, str>>) -> Self {
        let text = s.into();
        // a bit of a hack IMO - it's weird that text node is not the width of the text by default
        // let min_width = Px(text.len() as f32 * FONT_SIZE / 1.2);
        Self {
            text,
            text_layout: TextLayout::new_with_justify(JustifyText::Center),
            font: TextFont::from_font_size(FONT_SIZE),
            node: Node {
                align_items: AlignItems::Center,
                align_content: AlignContent::Center,
                justify_items: JustifyItems::Center,
                justify_content: JustifyContent::Center,
                border: UiRect::all(Px(2.0)),
                padding: UiRect::horizontal(Vw(3.0)),
                ..Default::default()
            },
            color: WHITEISH,
            bg_color: TRANSPARENT,
            border_color: WHITEISH,
            border_radius: BORDER_RADIUS,
        }
    }

    pub fn text(mut self, text: impl Into<Cow<'static, str>>) -> Self {
        self.text = text.into();
        self
    }
    pub fn font(mut self, font: TextFont) -> Self {
        self.font = font;
        self
    }
    pub fn font_size(mut self, s: f32) -> Self {
        self.font.font_size = s;
        self
    }
    pub fn node(mut self, new: Node) -> Self {
        self.node = new;
        self
    }
    pub fn width(mut self, w: Val) -> Self {
        self.node.width = w;
        self
    }
    pub fn height(mut self, h: Val) -> Self {
        self.node.height = h;
        self
    }
    pub fn row_gap(mut self, g: Val) -> Self {
        self.node.row_gap = g;
        self
    }
    pub fn margin(mut self, m: UiRect) -> Self {
        self.node.margin = m;
        self
    }
    pub fn padding(mut self, p: UiRect) -> Self {
        self.node.padding = p;
        self
    }
}

// For something like "my-label".into()
impl<T: Into<Cow<'static, str>>> From<T> for Opts {
    fn from(value: T) -> Self {
        Opts::new(value)
    }
}
