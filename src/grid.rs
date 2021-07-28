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
        let color_brush = self.compositor.CreateColorBrushWithColor(Colors::Red()?)?;

        for x in 0..8 {
            for y in 0..8 {
                let visual = self.compositor.CreateSpriteVisual()?;
                visual.SetSize(Vector2::new(25.0, 25.0))?;
                visual.SetBrush(&color_brush)?;
                visual.SetCenterPoint(Vector3::new(12.5, 12.5, 0.0))?;
                visual.SetOffset(Vector3::new(1.25, 1.25, 0.0) + (Vector3::new(27.5, 27.5, 27.5) * Vector3::new(x as f32, y as f32, 0.0)))?;
                self.container_visual.Children()?.InsertAtTop(&visual)?;
            }
        }

        Ok(())
    }
}
