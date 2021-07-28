fn main() {
    windows::build! {
        Windows::Foundation::Numerics::Vector2,
        Windows::System::DispatcherQueueController,
        Windows::UI::Composition::{Compositor, ContainerVisual},
        Windows::UI::Composition::Desktop::DesktopWindowTarget,
        Windows::Win32::System::Com::CoInitializeEx,
        Windows::Win32::System::WinRT::{CreateDispatcherQueueController, ICompositorDesktopInterop},
    };
}
