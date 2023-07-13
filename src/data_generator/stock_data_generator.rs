// Crate imports
use crate::common::UserEvent;

// Third party imports
use tuirealm::{listener::{ListenerResult, Poll}, Event};


pub struct StockDataGenerator;

impl Poll<UserEvent> for StockDataGenerator {
    fn poll(&mut self) -> ListenerResult<Option<Event<UserEvent>>> {
        //Ok(Some(Event::User(UserEvent::DataGenerated(self.generate()))))
        unimplemented!()
    }
}
