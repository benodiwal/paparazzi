use global_hotkey::{hotkey::{Code, HotKey, Modifiers}, GlobalHotKeyEvent, GlobalHotKeyManager};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::application::ApplicationHandler;
use anyhow::Result;

mod screenshot;
mod terminal;

struct App {
    receiver: crossbeam_channel::Receiver<GlobalHotKeyEvent>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {}
    
    fn window_event(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        _event: winit::event::WindowEvent,
    ) {
    }
    
    fn new_events(&mut self, event_loop: &winit::event_loop::ActiveEventLoop, _cause: winit::event::StartCause) {
        event_loop.set_control_flow(ControlFlow::Wait);
        
        if let Ok(event) = self.receiver.try_recv() {
            if event.state == global_hotkey::HotKeyState::Pressed {
                println!("Hotkey pressed! Taking screenshot...");
                if let Err(err) = handle_screenshot() {
                    eprintln!("Error: {}", err);
                }
            }
        }
    }
}

fn main() -> Result<()> {
    let event_loop = EventLoop::new()?;
    let manager = GlobalHotKeyManager::new()?;
    let hotkey = HotKey::new(Some(Modifiers::CONTROL | Modifiers::SHIFT), Code::KeyS);
    manager.register(hotkey)?;
    let receiver = GlobalHotKeyEvent::receiver().to_owned();
    
    let mut app = App { receiver };
    
    event_loop.run_app(&mut app)?;
    
    Ok(())
}

fn handle_screenshot() -> Result<()> {
    let screenshot_path = screenshot::capture()?;
    println!("✅ Screenshot saved to: {}", screenshot_path);
    let message = screenshot_path + "Analyze this image";
    terminal::send_to_claude_code_terminal(&message)?;

    Ok(())
}
