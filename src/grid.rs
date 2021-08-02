use bindings::Windows::{
    Foundation::Numerics::{Vector2, Vector3},
    UI::Colors,
    UI::Composition::{Compositor, ContainerVisual},
};

pub struct Grid {
    compositor: Compositor,
    root: ContainerVisual,

    tile_size: Vector2,
}

impl Grid {
    pub fn new(compositor: &Compositor, tile_size: Vector2) -> windows::Result<Self> {
        let compositor = compositor.clone();
        let root = compositor.CreateContainerVisual()?;

        Ok(Self {
            compositor: compositor,
            root: root,
            tile_size: tile_size.clone(),
        })
    }

    pub fn draw(&self) -> windows::Result<()> {
        let children = self.root.Children()?;
        let color_brush = self.compositor.CreateColorBrushWithColor(Colors::Red()?)?;

        self.root.SetSize(
            (&self.tile_size + Vector2::new(2.5, 2.5)) * Vector2::new(16.0 as f32, 16.0 as f32),
        )?;

        for x in 0..8 {
            for y in 0..8 {
                let visual = self.compositor.CreateSpriteVisual()?;
                visual.SetSize(Vector2::new(25.0, 25.0))?;
                visual.SetBrush(&color_brush)?;
                visual.SetCenterPoint(Vector3::new(12.5, 12.5, 0.0))?;
                visual.SetOffset(
                    Vector3::new(1.25, 1.25, 0.0)
                        + (Vector3::new(27.5, 27.5, 27.5) * Vector3::new(x as f32, y as f32, 0.0)),
                )?;
                children.InsertAtTop(&visual)?;
            }
        }

        Ok(())
    }
}
