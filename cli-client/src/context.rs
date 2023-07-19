use tuirealm::terminal::TerminalBridge;

pub struct Context {
    terminal: TerminalBridge,
}

impl Context {
    pub fn new() -> Self {
        Self {
            terminal: TerminalBridge::new().expect("Failed to initialize terminal"),
        }
    }

    pub fn terminal(&mut self) -> &mut TerminalBridge {
        &mut self.terminal
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        let _ = self.terminal.disable_raw_mode();
        let _ = self.terminal.leave_alternate_screen();
        let _ = self.terminal.clear_screen();
    }
}
