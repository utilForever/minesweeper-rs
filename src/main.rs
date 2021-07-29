mod grid;
mod minesweeper;

use minesweeper::Minesweeper;

use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use bindings::Windows::{
    Foundation::Numerics::Vector2,
    Graphics::SizeInt32,
    System::DispatcherQueueController,
    Win32::Foundation::HWND,
    Win32::System::Com::{CoInitializeEx, COINIT_APARTMENTTHREADED},
    Win32::System::WinRT::{
        CreateDispatcherQueueController, DispatcherQueueOptions, ICompositorDesktopInterop,
        DQTAT_COM_NONE, DQTYPE_THREAD_CURRENT,
    },
    UI::Composition::Compositor,
};

use raw_window_handle::HasRawWindowHandle;
use windows::Interface;

fn create_dispatcher() -> DispatcherQueueController {
    // We need a DispatcherQueue on our thread to properly create a Compositor. Note that since
    // we aren't pumping messages, the Compositor won't commit. This is fine for the test for now.

    let options = DispatcherQueueOptions {
        dwSize: std::mem::size_of::<DispatcherQueueOptions>() as u32,
        threadType: DQTYPE_THREAD_CURRENT,
        apartmentType: DQTAT_COM_NONE,
    };

    unsafe { CreateDispatcherQueueController(options).unwrap() }
}

fn run() -> windows::Result<()> {
    unsafe { CoInitializeEx(std::ptr::null_mut(), COINIT_APARTMENTTHREADED)? };
    let _dispatcher = create_dispatcher();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    window.set_title("Minesweeper");

    // Create desktop window target.
    let compositor = Compositor::new()?;
    let window_handle = window.raw_window_handle();
    let window_handle = match window_handle {
        raw_window_handle::RawWindowHandle::Windows(window_handle) => window_handle.hwnd,
        _ => panic!("Unsupported platform!"),
    };

    let compositor_desktop: ICompositorDesktopInterop = compositor.cast()?;

    let target = unsafe {
        compositor_desktop.CreateDesktopWindowTarget(HWND(window_handle as isize), false)?
    };

    // Create composition root.
    let container_visual = compositor.CreateContainerVisual()?;
    container_visual.SetRelativeSizeAdjustment(Vector2 { X: 1.0, Y: 1.0 })?;
    target.SetRoot(&container_visual)?;

    // Create grid.
    let window_size = window.inner_size();
    let game_board_size = SizeInt32 {
        Width: 30,
        Height: 16,
    };
    let game = Minesweeper::new(
        game_board_size.Width as u32,
        game_board_size.Height as u32,
        99,
    );
    game.show_with_mine_count();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });
}

fn main() {
    let result = run();

    // We do this for nicer HRESULT printing when errors occur.
    if let Err(error) = result {
        error.code().unwrap();
    }
}
