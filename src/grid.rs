use bindings::Windows::UI::Composition::{Compositor, ContainerVisual};

pub struct Grid {
    container_visual: ContainerVisual,
    compositor: Compositor,
    width: u32,
    height: u32,
}
