use crate::element::LayoutContext;
use util::ResultExt;

use crate::element::AnyElement;

struct Adapter<V>(AnyElement<V>);

impl<V: 'static> gpui::Element<V> for Adapter<V> {
    type LayoutState = ();
    type PaintState = ();

    fn layout(
        &mut self,
        constraint: gpui::SizeConstraint,
        view: &mut V,
        legacy_cx: &mut gpui::LayoutContext<V>,
    ) -> (gpui::geometry::vector::Vector2F, Self::LayoutState) {
        legacy_cx.push_layout_engine();
        let node = self
            .0
            .layout(view, &mut LayoutContext { legacy_cx })
            .log_err();

        if let Some(node) = node {
            legacy_cx
                .layout_engine()
                .unwrap()
                .compute_layout(node, constraint.max)
                .log_err();
        }
        legacy_cx.pop_layout_engine();

        (constraint.max, ())
    }

    fn paint(
        &mut self,
        scene: &mut gpui::SceneBuilder,
        bounds: gpui::geometry::rect::RectF,
        visible_bounds: gpui::geometry::rect::RectF,
        layout: &mut Self::LayoutState,
        view: &mut V,
        cx: &mut gpui::PaintContext<V>,
    ) -> Self::PaintState {
        todo!()
    }

    fn rect_for_text_range(
        &self,
        range_utf16: std::ops::Range<usize>,
        bounds: gpui::geometry::rect::RectF,
        visible_bounds: gpui::geometry::rect::RectF,
        layout: &Self::LayoutState,
        paint: &Self::PaintState,
        view: &V,
        cx: &gpui::ViewContext<V>,
    ) -> Option<gpui::geometry::rect::RectF> {
        todo!()
    }

    fn debug(
        &self,
        bounds: gpui::geometry::rect::RectF,
        layout: &Self::LayoutState,
        paint: &Self::PaintState,
        view: &V,
        cx: &gpui::ViewContext<V>,
    ) -> gpui::serde_json::Value {
        todo!()
    }
}
