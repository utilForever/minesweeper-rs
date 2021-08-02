use bindings::Windows::{
    Foundation::Numerics::{Vector2, Vector3},
    UI::Colors,
    UI::Composition::{Compositor, ContainerVisual, SpriteVisual},
};

#[derive(Clone)]
pub struct Grid {
    compositor: Compositor,
    root: ContainerVisual,
    tiles: Vec<SpriteVisual>,

    tile_size: Vector2,
    margin: Vector2,
}

impl Grid {
    pub fn new(
        compositor: &Compositor,
        tile_size: &Vector2,
        margin: &Vector2,
    ) -> windows::Result<Self> {
        let compositor = compositor.clone();
        let root = compositor.CreateContainerVisual()?;

        Ok(Self {
            compositor: compositor,
            root: root,
            tiles: Vec::new(),

            tile_size: tile_size.clone(),
            margin: margin.clone(),
        })
    }

    pub fn draw(&mut self) -> windows::Result<()> {
        let children = self.root.Children()?;
        children.RemoveAll()?;
        self.tiles.clear();

        self.root.SetSize(
            (&self.tile_size + Vector2::new(2.5, 2.5)) * Vector2::new(16.0 as f32, 16.0 as f32),
        )?;

        for x in 0..8 {
            for y in 0..8 {
                let visual = self.compositor.CreateSpriteVisual()?;
                visual.SetSize(&self.tile_size)?;
                visual.SetCenterPoint(Vector3::new(
                    self.tile_size.X / 2.0,
                    self.tile_size.Y / 2.0,
                    0.0,
                ))?;
                visual.SetOffset(
                    Vector3::new(self.margin.X / 2.0, self.margin.Y / 2.0, 0.0)
                        + (Vector3::new(
                            self.tile_size.X + self.margin.X,
                            self.tile_size.Y + self.margin.Y,
                            0.0,
                        ) * Vector3::new(x as f32, y as f32, 0.0)),
                )?;

                children.InsertAtTop(&visual)?;
                self.tiles.push(visual);
            }
        }

        Ok(())
    }

    pub fn root(&self) -> &ContainerVisual {
        &self.root
    }

    pub fn tiles_iter(&self) -> impl std::iter::Iterator<Item = &SpriteVisual> {
        self.tiles.iter()
    }
}
