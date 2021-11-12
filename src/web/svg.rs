// plotters-iced
//
// Iced backend for Plotters
// Copyright: 2021, Joylei <leingliu@gmail.com>
// License: MIT

use super::{text::Measurement, AsBumpStr};
use crate::{
    utils::{AndExt, RotateAngle},
    Error,
};
use dodrio::{
    builder::*,
    bumpalo::{self, collections::String},
};
use plotters_backend::{
    text_anchor, BackendColor, BackendCoord, BackendStyle, BackendTextStyle, DrawingBackend,
    DrawingErrorKind,
};

pub(crate) struct SvgBackend<'b, 'n> {
    size: (u32, u32),
    bump: &'b bumpalo::Bump,
    nodes: &'n mut Vec<dodrio::Node<'b>>,
    measurement: Measurement,
}

impl<'b, 'n> SvgBackend<'b, 'n> {
    #[inline]
    pub fn new(
        bump: &'b bumpalo::Bump,
        size: (u32, u32),
        nodes: &'n mut Vec<dodrio::Node<'b>>,
    ) -> Self {
        Self {
            size,
            bump,
            nodes,
            measurement: Measurement::new(),
        }
    }
}

impl<'b, 'n> DrawingBackend for SvgBackend<'b, 'n> {
    type ErrorType = Error;
    #[inline(always)]
    fn get_size(&self) -> (u32, u32) {
        self.size
    }

    #[inline(always)]
    fn ensure_prepared(&mut self) -> Result<(), DrawingErrorKind<Error>> {
        Ok(())
    }

    #[inline(always)]
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

    #[inline]
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

    #[inline]
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
            .and(|node| {
                if fill {
                    node.attr("stroke", "none")
                        .attr("fill", color.as_bump_str(bump))
                        .attr("fill-opacity", opacity)
                } else {
                    node.attr("stroke", color.as_bump_str(bump))
                        .attr("stroke-width", style.stroke_width().as_bump_str(bump))
                        .attr("stoke-opacity", opacity)
                }
            })
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
            .and(|node| {
                if fill {
                    node.attr("stroke", "none")
                        .attr("fill", color.as_bump_str(bump))
                        .attr("fill-opacity", opacity)
                } else {
                    node.attr("stroke", color.as_bump_str(bump))
                        .attr("stroke-width", style.stroke_width().as_bump_str(bump))
                        .attr("stoke-opacity", opacity)
                }
            })
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
        //super::log(&format!("pos:{},{}:{}", pos.0, pos.1, content));
        let bump = self.bump;
        let (width, height) = self.estimate_text_size(content, style)?;
        let width = width as i32;
        let height = height as i32;
        let rotate = match style.transform().angle() {
            None => "",
            Some(angle) => bumpalo::format!(
                in bump, "rotate({}, {}, {})", angle, pos.0, pos.1)
            .into_bump_str(),
        };
        let dx: i32 = match style.anchor().h_pos {
            text_anchor::HPos::Left => 0,
            text_anchor::HPos::Right => -width,
            text_anchor::HPos::Center => -width / 2,
        };
        //baseline aligned
        let dy: i32 = match style.anchor().v_pos {
            text_anchor::VPos::Top => height * 2 / 3,
            text_anchor::VPos::Bottom => 0,
            text_anchor::VPos::Center => height / 3,
        };
        let style = bumpalo::format!(
            in bump,
            "font-size:{}px;font-family:{};font-style:{}",
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

    #[inline(always)]
    fn estimate_text_size<S: BackendTextStyle>(
        &self,
        text: &str,
        style: &S,
    ) -> Result<(u32, u32), DrawingErrorKind<Self::ErrorType>> {
        self.measurement
            .measure(text, style)
            .map_err(|e| DrawingErrorKind::DrawingError(e.into()))
    }

    #[inline(always)]
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

    const SVG_NAMESPACE: &str = "http://www.w3.org/2000/svg";

    #[allow(unused)]
    #[inline(always)]
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
        ElementBuilder::new(bump, "g").namespace(Some(SVG_NAMESPACE))
    }

    #[inline(always)]
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
        ElementBuilder::new(bump, "text").namespace(Some(SVG_NAMESPACE))
    }

    #[allow(unused)]
    #[inline(always)]
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
        ElementBuilder::new(bump, "foreignObject").namespace(Some(SVG_NAMESPACE))
    }
}
