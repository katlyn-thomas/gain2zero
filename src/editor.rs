use std::sync::Arc;

use atomic_float::AtomicF32;
use nih_plug::prelude::Editor;
use nih_plug_vizia::widgets::{ResizeHandle, ParamSlider};
use nih_plug_vizia::{create_vizia_editor, ViziaTheming, assets};
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::{ViziaState, vizia::state::Model};

use crate::GainToZeroParams;

#[derive(Lens)]
struct Data {
    params: Arc<GainToZeroParams>,
    reduction_readout: Arc<AtomicF32>,
}

impl Model for Data {}

pub(crate) fn default_state() -> Arc<ViziaState> {
    ViziaState::new(|| (200, 150))
}

pub(crate) fn create(
    params: Arc<GainToZeroParams>,
    reduction_readout: Arc<AtomicF32>,
    editor_state: Arc<ViziaState>,
) -> Option<Box<dyn Editor>> {
    create_vizia_editor(editor_state, ViziaTheming::Custom, move |cx, _| {
        assets::register_noto_sans_light(cx);
        assets::register_noto_sans_thin(cx);

        Data {
            params: params.clone(),
            reduction_readout: reduction_readout.clone(),
        }
        .build(cx);

        ResizeHandle::new(cx);

        VStack::new(cx, |cx| {
            Label::new(cx, "Gain2Zero")
                .font_family(vec![FamilyOwned::Name(String::from(assets::NOTO_SANS_THIN))])
                .font_size(30.)
                .height(Pixels(50.))
                .child_top(Stretch(1.))
                .child_bottom(Pixels(0.));

            Label::new(cx, "Threshold");
            ParamSlider::new(cx, Data::params, |params| &params.threshold);
        })
        .row_between(Pixels(0.))
        .child_left(Stretch(1.0))
        .child_right(Stretch(1.0));
    })
}