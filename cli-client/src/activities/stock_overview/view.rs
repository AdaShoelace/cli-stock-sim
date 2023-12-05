//Crate imports
use super::{components, StockOverview};
use crate::{
    common::{Id, UserEvent},
    common_components,
    util::popup,
};

// Third party imports
use log::error;
use tuirealm::{
    event::{Key, KeyEvent, KeyModifiers},
    tui::{
        layout::{Constraint, Direction, Layout},
        widgets::Clear,
    },
    Sub, SubClause, SubEventClause,
};

impl StockOverview {
    pub(super) fn init(&mut self) {
        assert!(self
            .app
            .remount(
                Id::GlobalListener,
                Box::new(common_components::GlobalListener::default()),
                vec![Sub::new(
                    SubEventClause::Keyboard(KeyEvent {
                        code: Key::Esc,
                        modifiers: KeyModifiers::NONE
                    }),
                    SubClause::Always
                )]
            )
            .is_ok());
        assert!(self
            .app
            .remount(
                Id::TitleBar,
                Box::new(common_components::TitleBar::default()),
                vec![Sub::new(SubEventClause::Any, SubClause::Always)]
            )
            .is_ok());
        assert!(self
            .app
            .remount(Id::Header, Box::new(components::Header::default()), vec![])
            .is_ok());

        assert!(self
            .app
            .remount(
                Id::StockList,
                Box::new(components::StockList::default()),
                vec![Sub::new(
                    SubEventClause::User(UserEvent::Init),
                    SubClause::Always
                )]
            )
            .is_ok());

        assert!(self
            .app
            .remount(
                Id::StockChart,
                Box::new(components::StockChart::new()),
                vec![Sub::new(SubEventClause::Any, SubClause::Always)]
            )
            .is_ok());
        assert!(self.app.active(&Id::Header).is_ok());

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
                        Constraint::Percentage(5),
                        Constraint::Percentage(5),
                        Constraint::Percentage(45),
                        Constraint::Percentage(45),
                    ]
                    .as_ref(),
                )
                .split(f.size());

            self.app.view(&Id::TitleBar, f, chunks[0]);
            self.app.view(&Id::Header, f, chunks[1]);
            self.app.view(&Id::StockList, f, chunks[2]);
            self.app.view(&Id::StockChart, f, chunks[3]);

            if self.app.mounted(&Id::ExitPopUp) {
                let pop_rect = popup::Popup(popup::Size::Percentage(50), popup::Size::Percentage(20))
                    .draw_in(f.size());
                f.render_widget(Clear, pop_rect);

                let _popup_chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                    .split(pop_rect);
                self.app.view(&Id::ExitPopUp, f, pop_rect);
                if let Some(&Id::ExitPopUp) = self.app.focus() {
                } else {
                    assert!(self.app.active(&Id::ExitPopUp).is_ok());
                }
            }
        });

        if let Err(err) = res {
            error!("{}", err);
        }
        self.context = Some(ctx);
    }
}
