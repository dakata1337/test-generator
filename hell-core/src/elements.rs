use rckive_genpdf::{
    elements::{BulletPoint, LinearLayout},
    error::Error,
    render,
    style::Style,
    Context, Element, Position, RenderResult,
};

pub struct AlphabeticOrderedList {
    layout: LinearLayout,
    number: u32,
    start_char: char,
}
impl AlphabeticOrderedList {
    pub fn new(ch: char) -> Self {
        Self::with_start(0, ch)
    }
    pub fn with_start(start: u32, ch: char) -> Self {
        Self {
            layout: LinearLayout::vertical(),
            number: start,
            start_char: ch,
        }
    }

    pub fn push<E: Element + 'static>(&mut self, element: E) {
        // TODO: make this safer
        let mut point = BulletPoint::new(element);
        let ch = self.start_char as u32 + self.number;
        let ch = char::from_u32(ch).unwrap();
        point.set_bullet(format!("{})", ch));
        self.layout.push(point);
        self.number += 1;
    }
}

impl Element for AlphabeticOrderedList {
    fn render(
        &mut self,
        context: &Context,
        area: render::Area<'_>,
        style: Style,
    ) -> Result<RenderResult, Error> {
        self.layout.render(context, area, style)
    }
}

pub struct DottedLine;
impl Element for DottedLine {
    fn render(
        &mut self,
        context: &rckive_genpdf::Context,
        area: rckive_genpdf::render::Area<'_>,
        style: rckive_genpdf::style::Style,
    ) -> Result<rckive_genpdf::RenderResult, rckive_genpdf::error::Error> {
        let mut result = RenderResult::default();

        let width_per_ch = style.char_width(&context.font_cache, '.');
        let n_chars = area.size().width / f64::from(width_per_ch);

        area.print_str(
            &context.font_cache,
            Position::default(),
            style,
            ".".repeat(f64::from(n_chars) as usize),
        )?;

        let line_height = style.line_height(&context.font_cache);
        result.size.height = line_height;
        Ok(result)
    }
}
