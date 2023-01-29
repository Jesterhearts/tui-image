use image::{
    imageops::FilterType,
    DynamicImage,
    GenericImageView,
};
use tui::{
    buffer::Buffer,
    layout::Rect,
    style::{
        Color,
        Style,
    },
    widgets::{
        Block,
        Widget,
    },
};

/// A tui widget for displaying images.
/// All images will be displayed centered vertically & horizontally on the
/// available space.
///
/// No support for transparancy is provided.
pub struct ImageWidget<'a> {
    image: &'a DynamicImage,
    block: Option<Block<'a>>,
    style: Style,
    scale_up: bool,
    filter_mode: FilterType,
}

impl<'a> ImageWidget<'a> {
    pub fn new(image: &DynamicImage) -> ImageWidget {
        ImageWidget {
            image,
            block: None,
            style: Style::default(),
            scale_up: false,
            filter_mode: FilterType::Lanczos3,
        }
    }

    /// Set the style of the background around the displayed image.
    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    /// Indicate if the image should be scaled up to fit the available area.
    /// Defaults to `false`.
    pub fn upscale(mut self, upscale: bool) -> Self {
        self.scale_up = upscale;
        self
    }

    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }

    /// Set the filter mode for upscaling/downscaling.
    /// Defaults to [`FilterType::Lanczos3`].
    pub fn filter_mode(mut self, filter_mode: FilterType) -> Self {
        self.filter_mode = filter_mode;
        self
    }
}

impl Widget for ImageWidget<'_> {
    fn render(mut self, area: Rect, buf: &mut Buffer) {
        const HALF_BLOCK: char = '▄';
        buf.set_style(area, self.style);

        let area = match self.block.take() {
            Some(block) => {
                let inner_area = block.inner(area);
                block.render(area, buf);
                inner_area
            }
            None => area,
        };

        let mut image_area = area;
        if !self.scale_up {
            image_area.width = u32::from(area.width).min(self.image.width()) as u16;
            image_area.height = u32::from(area.height * 2).min(self.image.height()) as u16;
        } else {
            image_area.height *= 2;
        }

        if image_area.width % 2 == 1 {
            image_area.width -= 1;
        }

        if image_area.height % 2 == 1 {
            image_area.height -= 1;
        }

        let image = self.image.resize(
            u32::from(image_area.width),
            u32::from(image_area.height),
            image::imageops::FilterType::Lanczos3,
        );

        let x_start = (area.width - image.width() as u16) / 2 + image_area.left();
        let y_start = (area.height - image.height() as u16 / 2) / 2 + image_area.top();

        for (y_actual, y_pixel) in (0..image.height() as u16).step_by(2).enumerate() {
            for x in 0..image.width() as u16 {
                let cell = buf.get_mut(x_start + x, y_start + y_actual as u16);

                let bg_pixel = image.get_pixel(u32::from(x), u32::from(y_pixel));
                let fg_pixel = image.get_pixel(u32::from(x), u32::from(y_pixel + 1));

                cell.set_char(HALF_BLOCK)
                    .set_bg(Color::Rgb(bg_pixel[0], bg_pixel[1], bg_pixel[2]))
                    .set_fg(Color::Rgb(fg_pixel[0], fg_pixel[1], fg_pixel[2]));
            }
        }
    }
}
