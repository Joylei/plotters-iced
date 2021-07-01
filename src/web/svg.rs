use super::{AndExt, AsBumpStr};
use crate::Error;
use dodrio::{
    builder::*,
    bumpalo::{self, collections::String},
};
use plotters_backend::{
    text_anchor, BackendColor, BackendCoord, BackendStyle, BackendTextStyle, DrawingBackend,
    DrawingErrorKind, FontTransform,
};

pub(crate) struct SvgBackend<'b, 'n> {
    size: (u32, u32),
    bump: &'b bumpalo::Bump,
    nodes: &'n mut Vec<dodrio::Node<'b>>,
    div: web_sys::Element,
}

impl<'b, 'n> SvgBackend<'b, 'n> {
    pub fn new(
        bump: &'b bumpalo::Bump,
        size: (u32, u32),
        nodes: &'n mut Vec<dodrio::Node<'b>>,
    ) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();
        let div = document.create_element("div").unwrap();
        div.set_attribute(
            "style",
            "border:0;padding:0;margin:0;display:none;z-index:-100;",
        )
        .unwrap();
        body.append_child(&div).unwrap();
        Self {
            size,
            bump,
            nodes,
            div,
        }
    }
}

impl<'b, 'n> Drop for SvgBackend<'b, 'n> {
    fn drop(&mut self) {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();
        body.remove_child(&self.div).ok();
    }
}

impl<'b, 'n> DrawingBackend for SvgBackend<'b, 'n> {
    type ErrorType = Error;
    #[inline]
    fn get_size(&self) -> (u32, u32) {
        self.size
    }

    #[inline]
    fn ensure_prepared(&mut self) -> Result<(), DrawingErrorKind<Error>> {
        Ok(())
    }

    #[inline]
    fn present(&mut self) -> Result<(), DrawingErrorKind<Error>> {
        Ok(())
    }

    #[inline]
    fn draw_pixel(
        &mut self,
        point: BackendCoord,
        color: BackendColor,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        if color.alpha == 0.0 {
            return Ok(());
        }
        let bump = self.bump;
        let node = rect(bump)
            .attr("x", point.0.as_bump_str(bump))
            .attr("y", point.1.as_bump_str(bump))
            .attr("width", "1")
            .attr("height", "1")
            .attr("stroke", "none")
            .attr("fill-opacity", color.alpha.as_bump_str(bump))
            .attr("fill", color.as_bump_str(bump))
            .finish();
        self.nodes.push(node);
        Ok(())
    }

    fn draw_line<S: BackendStyle>(
        &mut self,
        from: BackendCoord,
        to: BackendCoord,
        style: &S,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        let color = style.color();
        if color.alpha == 0.0 {
            return Ok(());
        }
        let bump = self.bump;
        let node = line(bump)
            .attr("x1", from.0.as_bump_str(bump))
            .attr("y1", from.1.as_bump_str(bump))
            .attr("x2", to.0.as_bump_str(bump))
            .attr("y2", to.1.as_bump_str(bump))
            .attr("stroke", color.as_bump_str(bump))
            .attr("stroke-width", style.stroke_width().as_bump_str(bump))
            .attr("stroke-opacity", color.alpha.as_bump_str(bump))
            .finish();
        self.nodes.push(node);
        Ok(())
    }

    fn draw_rect<S: BackendStyle>(
        &mut self,
        upper_left: BackendCoord,
        bottom_right: BackendCoord,
        style: &S,
        fill: bool,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        let color = style.color();
        if color.alpha == 0.0 {
            return Ok(());
        }
        let width = bottom_right.0 - upper_left.0;
        let height = bottom_right.1 - upper_left.1;
        let bump = self.bump;
        let opacity = color.alpha.as_bump_str(bump);
        let node = rect(bump)
            .attr("x", upper_left.0.as_bump_str(bump))
            .attr("y", upper_left.1.as_bump_str(bump))
            .attr("width", width.as_bump_str(bump))
            .attr("height", height.as_bump_str(bump))
            .attr(
                "stroke",
                if !fill {
                    color.as_bump_str(bump)
                } else {
                    "none"
                },
            )
            .attr("stroke-width", style.stroke_width().as_bump_str(bump))
            .and(|node| {
                if !fill {
                    node.attr("stoke-opacity", opacity)
                } else {
                    node.attr("fill-opacity", opacity)
                }
            })
            .attr(
                "fill",
                if fill {
                    color.as_bump_str(bump)
                } else {
                    "none"
                },
            )
            .finish();
        self.nodes.push(node);
        Ok(())
    }

    fn draw_path<S: BackendStyle, I: IntoIterator<Item = BackendCoord>>(
        &mut self,
        path_points: I,
        style: &S,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        let color = style.color();
        if color.alpha == 0.0 {
            return Ok(());
        }
        let bump = self.bump;
        let mut points = String::new_in(bump);
        for (i, (x, y)) in path_points.into_iter().enumerate() {
            let prefix = if i == 0 { "M" } else { "L" };
            points.push_str(
                bumpalo::format!(
                in bump, "{}{},{}", prefix,x,y)
                .into_bump_str(),
            );
        }
        let node = path(bump)
            .attr("d", points.into_bump_str())
            .attr("fill", "none")
            .attr("stroke", color.as_bump_str(bump))
            .attr("stroke-width", style.stroke_width().as_bump_str(bump))
            .attr("stroke-opacity", color.alpha.as_bump_str(bump))
            .attr("stroke-linejoin", "round")
            .finish();
        self.nodes.push(node);
        Ok(())
    }

    fn draw_circle<S: BackendStyle>(
        &mut self,
        center: BackendCoord,
        radius: u32,
        style: &S,
        fill: bool,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        let color = style.color();
        if color.alpha == 0.0 {
            return Ok(());
        }
        let bump = self.bump;
        let opacity = color.alpha.as_bump_str(bump);
        let node = circle(bump)
            .attr("cx", center.0.as_bump_str(bump))
            .attr("cy", center.1.as_bump_str(bump))
            .attr("r", radius.as_bump_str(bump))
            .attr(
                "stroke",
                if !fill {
                    color.as_bump_str(bump)
                } else {
                    "none"
                },
            )
            .attr("stroke-width", style.stroke_width().as_bump_str(bump))
            .and(|node| {
                if !fill {
                    node.attr("stoke-opacity", opacity)
                } else {
                    node.attr("fill-opacity", opacity)
                }
            })
            .attr(
                "fill",
                if fill {
                    color.as_bump_str(bump)
                } else {
                    "none"
                },
            )
            .finish();
        self.nodes.push(node);

        Ok(())
    }

    fn fill_polygon<S: BackendStyle, I: IntoIterator<Item = BackendCoord>>(
        &mut self,
        vert: I,
        style: &S,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        let color = style.color();
        if color.alpha == 0.0 {
            return Ok(());
        }
        let bump = self.bump;
        let mut points = String::new_in(bump);
        for (x, y) in vert.into_iter() {
            points.push_str(
                bumpalo::format!(
                in bump, "{},{} ",x,y)
                .into_bump_str(),
            );
        }
        points.push_str("Z");
        let node = polygon(bump)
            .attr("d", points.into_bump_str())
            .attr("fill-opacity", color.alpha.as_bump_str(bump))
            .attr("fill", color.as_bump_str(bump))
            .finish();
        self.nodes.push(node);
        Ok(())
    }

    fn draw_text<S: BackendTextStyle>(
        &mut self,
        content: &str,
        style: &S,
        pos: BackendCoord,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        let color = style.color();
        if color.alpha == 0.0 {
            return Ok(());
        }
        let bump = self.bump;
        let (width, height) = self.estimate_text_size(content, style)?;
        let width = width as i32;
        let height = height as i32;
        let rotate = match style.transform() {
            FontTransform::None => "",
            FontTransform::Rotate90 => bumpalo::format!(
                in bump, "rotate(90, {}, {})", pos.0, pos.1)
            .into_bump_str(),
            FontTransform::Rotate180 => bumpalo::format!(
                in bump, "rotate(180, {}, {})", pos.0, pos.1)
            .into_bump_str(),
            FontTransform::Rotate270 => bumpalo::format!(
                in bump, "rotate(270, {}, {})", pos.0, pos.1)
            .into_bump_str(),
            FontTransform::RotateAngle(angle) => bumpalo::format!(
                in bump, "rotate({}, {}, {})", angle, pos.0, pos.1)
            .into_bump_str(),
        };
        let dx: i32 = match style.anchor().h_pos {
            text_anchor::HPos::Left => 0,
            text_anchor::HPos::Right => -width,
            text_anchor::HPos::Center => -width / 2,
        };
        let dy: i32 = match style.anchor().v_pos {
            text_anchor::VPos::Top => height / 2,
            text_anchor::VPos::Bottom => height + height / 2,
            text_anchor::VPos::Center => height / 2,
        };
        let style = bumpalo::format!(
            in bump,
            "font-size: {}px; font-family: {}; font-style: {}",
            style.size(),
            style.family().as_str(),
            style.style().as_str()
        )
        .into_bump_str();

        let node = svg_builder::text(bump)
            .attr("x", (pos.0 + dx).as_bump_str(bump))
            .attr("y", (pos.1 + dy).as_bump_str(bump))
            .attr("style", style)
            .attr("fill", color.as_bump_str(bump))
            .attr("fill-opacity", color.alpha.as_bump_str(bump))
            .and(|node| {
                if !rotate.is_empty() {
                    node.attr("transform", rotate)
                } else {
                    node
                }
            })
            .child(text(content.as_bump_str(bump)))
            .finish();

        self.nodes.push(node);

        Ok(())
    }

    fn estimate_text_size<S: BackendTextStyle>(
        &self,
        text: &str,
        style: &S,
    ) -> Result<(u32, u32), DrawingErrorKind<Self::ErrorType>> {
        let angle = match style.transform() {
            FontTransform::None => 0 as f32,
            FontTransform::Rotate90 => 90 as f32,
            FontTransform::Rotate180 => 180 as f32,
            FontTransform::Rotate270 => 270 as f32,
            FontTransform::RotateAngle(angle) => angle,
        };
        let style = format!(
            "border:0;padding:0;margin:0;position:fixed;left:-10000;\
            display:block;width:auto;height;z-index:-100;\
            font-size: {}px; font-family: {}; font-style: {}; transform: rotate({}deg);",
            style.size(),
            style.family().as_str(),
            style.style().as_str(),
            angle
        );
        self.div.set_attribute("style", &style).unwrap();
        self.div.set_text_content(Some(text));
        let rect = self.div.get_bounding_client_rect();
        let size = (rect.width().ceil() as u32, rect.height().ceil() as u32);
        //super::log(&format!("{},{}:{}", size.0, size.1, text));
        Ok(size)
    }

    fn blit_bitmap(
        &mut self,
        _pos: BackendCoord,
        (_iw, _ih): (u32, u32),
        _src: &[u8],
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        // Not supported yet (rendering ignored)

        Ok(())
    }
}

mod svg_builder {
    use dodrio::builder::ElementBuilder;
    use dodrio::bumpalo::{self, Bump};
    use dodrio::{Attribute, Listener, Node};

    #[allow(unused)]
    pub fn g<'a, B>(
        bump: B,
    ) -> ElementBuilder<
        'a,
        bumpalo::collections::Vec<'a, Listener<'a>>,
        bumpalo::collections::Vec<'a, Attribute<'a>>,
        bumpalo::collections::Vec<'a, Node<'a>>,
    >
    where
        B: Into<&'a Bump>,
    {
        ElementBuilder::new(bump, "g").namespace(Some("http://www.w3.org/2000/svg"))
    }

    pub fn text<'a, B>(
        bump: B,
    ) -> ElementBuilder<
        'a,
        bumpalo::collections::Vec<'a, Listener<'a>>,
        bumpalo::collections::Vec<'a, Attribute<'a>>,
        bumpalo::collections::Vec<'a, Node<'a>>,
    >
    where
        B: Into<&'a Bump>,
    {
        ElementBuilder::new(bump, "text").namespace(Some("http://www.w3.org/2000/svg"))
    }

    #[allow(unused)]
    pub fn foreign_object<'a, B>(
        bump: B,
    ) -> ElementBuilder<
        'a,
        bumpalo::collections::Vec<'a, Listener<'a>>,
        bumpalo::collections::Vec<'a, Attribute<'a>>,
        bumpalo::collections::Vec<'a, Node<'a>>,
    >
    where
        B: Into<&'a Bump>,
    {
        ElementBuilder::new(bump, "foreignObject").namespace(Some("http://www.w3.org/2000/svg"))
    }
}
