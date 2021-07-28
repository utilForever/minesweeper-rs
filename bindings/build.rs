fn main() {
    windows::build! {
        Windows::System::DispatcherQueueController,
        Windows::Win32::System::Com::CoInitializeEx,
        Windows::Win32::System::WinRT::CreateDispatcherQueueController,
    };
}
