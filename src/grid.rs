use bindings::Windows::UI::Composition::{Compositor, ContainerVisual};

pub struct Grid {
    container_visual: ContainerVisual,
    compositor: Compositor,
    width: u32,
    height: u32,
}

impl Grid {
    pub fn new(
        container_visual: &ContainerVisual,
        width: u32,
        height: u32,
    ) -> windows::Result<Self> {
        Ok(Self {
            container_visual: container_visual.clone(),
            compositor: container_visual.Compositor()?.clone(),
            width,
            height,
        })
    }
}
