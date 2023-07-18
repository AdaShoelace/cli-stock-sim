// Crate imports
use crate::common::UserEvent;

// Std imports
use std::{sync::mpsc::{Receiver, Sender, channel}, thread, time};

// Third party imports
use log::{debug, error};
use simple_ntp::sntp;
use tuirealm::{
    listener::{ListenerResult, Poll},
    Event,
};

pub struct TimeDataGenerator {
    rx: Receiver<Result<std::time::Duration, sntp::NtpError>>
}

impl Default for TimeDataGenerator {
    fn default() -> Self {
        let (tx, rx) = channel();
        
        _ = thread::spawn(move || {
            let mut dt = time::Instant::now();

            loop {
                if time::Instant::now().duration_since(dt) >= time::Duration::from_secs(3) {
                    if let Err(e) = tx.send(sntp::unix_timestamp("ntp.aliyun.com")) {
                        error!("{}", e);
                    }
                    dt = time::Instant::now();
                }
            }
        });

        Self { rx }
    }
}

impl Poll<UserEvent> for TimeDataGenerator {
    fn poll(&mut self) -> ListenerResult<Option<Event<UserEvent>>> {
        let res = if let Ok(dur) = self.rx.try_recv() {
            debug!("Time: {:?}", dur);
            let dur = dur.unwrap().as_secs() as i64;
            Some(Event::User(UserEvent::TimeTick(dur)))
        } else {
            None
        };
        Ok(res)
    }
}
