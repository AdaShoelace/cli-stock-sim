//Crate imports
use crate::common::Id;
use super::{Login, components};

// Third party imports
use log::error;
use tuirealm::tui::layout::{Constraint, Direction, Layout};

impl Login {
    pub(super) fn init(&mut self) {
        assert!(self
            .app
            .mount(Id::Login, Box::new(components::Login::default()), vec![])
            .is_ok());
        assert!(self.app.active(&Id::Login).is_ok());
    }

    pub(super) fn view(&mut self) {
        self.redraw = false;

        let mut ctx = self.context.take().unwrap();
        let res = ctx.terminal().raw_mut().draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(
                    [
                        Constraint::Ratio(1, 3),
                        Constraint::Ratio(1, 3),
                        Constraint::Ratio(1, 3),
                    ]
                    .as_ref(),
                )
                .split(f.size());

            let centered = Layout::default()
                .direction(Direction::Horizontal)
                .margin(1)
                .constraints(
                    [
                        Constraint::Ratio(1, 3),
                        Constraint::Ratio(1, 3),
                        Constraint::Ratio(1, 3),
                    ]
                    .as_ref(),
                )
                .split(chunks[1]);

            self.app.view(&Id::Login, f, centered[1]);
        });

        if let Err(err) = res {
            error!("{}", err);
        }
        self.context = Some(ctx);
    }
}
