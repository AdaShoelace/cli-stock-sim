//Crate imports
use crate::{activities::StockOverview, common::Id, components};

// Third party imports
use log::{debug, error};
use tuirealm::tui::layout::{Constraint, Direction, Layout};

impl StockOverview {
    pub(super) fn init(&mut self) {
        assert!(self
            .app
            .mount(
                Id::StockOverview,
                Box::new(components::stock_market::Overview::default()),
                vec![]
            )
            .is_ok());
        assert!(self
            .app
            .mount(
                Id::Header,
                Box::new(components::Header::default()),
                vec![]
            ).is_ok());
        debug!("StockOverview mounted");
        assert!(self.app.active(&Id::Header).is_ok());
        debug!("StockOverview active");
    }

    pub(super) fn view(&mut self) {
        self.redraw = false;

        let mut ctx = self.context.take().unwrap();
        let res = ctx.terminal().raw_mut().draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([Constraint::Length(3), Constraint::Length(3)].as_ref())
                .split(f.size());
            self.app.view(&Id::Header, f, chunks[0]);
            self.app.view(&Id::StockOverview, f, chunks[1]);
        });

        if let Err(err) = res {
            error!("{}", err);
        }
        self.context = Some(ctx);
    }
}
