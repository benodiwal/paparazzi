use global_hotkey::{hotkey::{Code, HotKey, Modifiers}, GlobalHotKeyEvent, GlobalHotKeyManager};
use winit::event_loop::{ControlFlow, EventLoopBuilder};
use anyhow::Result;

mod screenshot;

fn main() -> Result<()> {
    let event_loop = EventLoopBuilder::new().build()?;
    let manager = GlobalHotKeyManager::new()?;

    let hotkey = HotKey::new(Some(Modifiers::CONTROL | Modifiers::SHIFT), Code::KeyS);
    manager.register(hotkey)?;

    let receiver = GlobalHotKeyEvent::receiver();
    event_loop.run(move |_event, elwt| {
        elwt.set_control_flow(ControlFlow::Wait);

        if let Ok(event) = receiver.try_recv() {
            if event.state == global_hotkey::HotKeyState::Pressed {
                println!("Hotkey pressed! Taking screenshot...");
            }
        }
    })?;

    Ok(())
}
