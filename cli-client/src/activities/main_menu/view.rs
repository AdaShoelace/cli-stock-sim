//Crate imports
use crate::{activities::MainMenu, common::Id, components};

// Third party imports
use tuirealm::tui::layout::{Layout, Constraint, Direction};
use log::error;

impl MainMenu {
    pub(super) fn init(&mut self) {
        assert!(self
            .app
            .mount(
                Id::MainMenu,
                Box::new(components::MainMenu::default()),
                vec![]
            )
            .is_ok());
        assert!(self.app.active(&Id::MainMenu).is_ok());
    }

    pub(super) fn view(&mut self) {
        
        self.redraw = false;

        let mut ctx = self.context.take().unwrap();
        let res =  ctx.terminal().raw_mut().draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([Constraint::Length(3)].as_ref())
                .split(f.size());
            self.app.view(&Id::MainMenu, f, chunks[0]);
        });

        if let Err(err) = res {
            error!("{}", err);
        }
        self.context = Some(ctx);
    }
}
