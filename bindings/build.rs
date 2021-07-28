fn main() {
    windows::build! {
        Windows::System::DispatcherQueueController,
        Windows::UI::Colors,
        Windows::UI::Composition::{Compositor, ContainerVisual, SpriteVisual, VisualCollection, CompositionColorBrush},
        Windows::UI::Composition::Desktop::DesktopWindowTarget,
        Windows::Win32::System::Com::CoInitializeEx,
        Windows::Win32::System::WinRT::{CreateDispatcherQueueController, ICompositorDesktopInterop},
    };
}
