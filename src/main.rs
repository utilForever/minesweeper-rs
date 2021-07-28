use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use bindings::Windows::{
    System::DispatcherQueueController,
    UI::Composition::Compositor,
    Win32::Foundation::HWND,
    Win32::System::Com::{CoInitializeEx, COINIT_APARTMENTTHREADED},
    Win32::System::WinRT::{CreateDispatcherQueueController, DispatcherQueueOptions, ICompositorDesktopInterop, DQTYPE_THREAD_CURRENT, DQTAT_COM_NONE},
};

use windows::Interface;
use raw_window_handle::HasRawWindowHandle;

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
        raw_window_handle::RawWindowHandle::Windows(window_handle) => {
            window_handle.hwnd
        }
        _ => panic!("Unsupported platform!"),
    };    

    let compositor_desktop: ICompositorDesktopInterop = compositor.cast()?;

    let target = unsafe {
        compositor_desktop
            .CreateDesktopWindowTarget(HWND(window_handle as isize), false)?
    };

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
