use crate::grid::Grid;

use bindings::Windows::{
    Foundation::Numerics::{Vector2, Vector3},
    UI::Colors,
    UI::Composition::{CompositionBorderMode, Compositor, ContainerVisual},
};

#[derive(Clone)]
pub struct GUI {
    compositor: Compositor,
    window_size: Vector2,

    game_board: Grid,
}

impl GUI {
    pub fn new(container_visual: &ContainerVisual, window_size: &Vector2) -> windows::Result<Self> {
        let compositor = container_visual.Compositor()?;
        let root = compositor.CreateSpriteVisual()?;

        root.SetRelativeSizeAdjustment(Vector2::new(1.0, 1.0))?;
        root.SetBrush(compositor.CreateColorBrushWithColor(Colors::White()?)?)?;
        root.SetBorderMode(CompositionBorderMode::Hard)?;
        container_visual.Children()?.InsertAtTop(&root)?;

        let tile_size = Vector2::new(25.0, 25.0);
        let margin = Vector2::new(2.5, 2.5);
        let game_board = Grid::new(&compositor, &tile_size, &margin)?;

        let game_board_visual = game_board.root();
        game_board_visual.SetRelativeOffsetAdjustment(Vector3::new(0.5, 0.5, 0.0))?;
        game_board_visual.SetAnchorPoint(Vector2::new(0.5, 0.5))?;
        root.Children()?.InsertAtTop(game_board_visual)?;

        let result = Self {
            compositor: compositor,
            window_size: window_size.clone(),
            game_board: game_board,
        };

        Ok(result)
    }

    pub fn draw_grid(&self) -> windows::Result<()> {
        self.game_board.draw()?;

        Ok(())
    }
}
