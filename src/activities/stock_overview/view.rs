//Crate imports
use crate::{activities::StockOverview, common::{Id, UserEvent}, components};

// Third party imports
use log::{debug, error};
use tuirealm::{
    props::{AttrValue, Attribute},
    tui::layout::{Constraint, Direction, Layout},
    Sub, SubClause, SubEventClause,
};

impl StockOverview {
    pub(super) fn init(&mut self) {
        assert!(self
            .app
            .mount(Id::Header, Box::new(components::Header::default()), vec![])
            .is_ok());
        debug!("Header mounted");

        assert!(self
            .app
            .mount(
                Id::StockOverview,
                Box::new(components::stock_market::Overview::default()),
                vec![Sub::new(SubEventClause::User(UserEvent::Init), SubClause::Always)]
            )
            .is_ok());
        debug!("StockOverview mounted");

        assert!(self
            .app
            .mount(
                Id::StockChart,
                Box::new(components::stock_market::StockChart::new()),
                vec![Sub::new(SubEventClause::Any, SubClause::Always)]
            )
            .is_ok());
        debug!("StockChart mounted");
        assert!(self.app.active(&Id::Header).is_ok());
        debug!("Header active");

        _ = self.tx.send(UserEvent::Init);
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
                        Constraint::Percentage(6),
                        Constraint::Percentage(47),
                        Constraint::Percentage(47),
                    ]
                    .as_ref(),
                )
                .split(f.size());
            self.app.view(&Id::Header, f, chunks[0]);
            self.app.view(&Id::StockOverview, f, chunks[1]);
            self.app.view(&Id::StockChart, f, chunks[2]);
        });

        if let Err(err) = res {
            error!("{}", err);
        }
        self.context = Some(ctx);
    }
}
