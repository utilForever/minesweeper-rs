use bindings::Windows::{
    Foundation::Numerics::{Vector2, Vector3},
    UI::Colors,
    UI::Composition::{Compositor, ContainerVisual}
};

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

    pub fn draw(&self) -> windows::Result<()> {
        let visual = self.compositor.CreateSpriteVisual()?;

        visual.SetSize(Vector2 { X: 30.0, Y: 30.0 })?;
        visual.SetBrush(self.compositor.CreateColorBrushWithColor(Colors::Red()?)?)?;
        visual.SetOffset(Vector3 {
            X: self.width as f32 / 2.0,
            Y: self.height as f32 / 2.0,
            Z: 0.0,
        })?;
        self.container_visual.Children()?.InsertAtTop(&visual)?;

        Ok(())
    }
}
