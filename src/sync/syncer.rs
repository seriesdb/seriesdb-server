use crate::sync::connection::ChannelItem;
use crate::sync::connection::Connection;
use crate::sync::connection::MsgWithTx;
use crate::sync::ref_aware::RefAware;
use actix::prelude::*;
use futures::future::lazy;
use futures::sync::oneshot::{self, Receiver, Sender};
use std::time::Duration;
use tokio::timer::Timeout;

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(3);
const SEND_TIMEOUT: Duration = Duration::from_secs(5);

pub struct Syncer {
    conn_addr: Addr<Connection>,
}

impl Actor for Syncer {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        log::info!("Syncer actor started.");
        self.hb(ctx);
    }

    fn stopped(&mut self, _: &mut Context<Self>) {
        log::info!("Syncer actor stopped.");
    }
}

impl Syncer {
    fn new() -> Self {
        Syncer {
            conn_addr: Connection::start(),
        }
    }

    pub fn start() -> Addr<Self> {
        let arbiter = Arbiter::new();
        Syncer::start_in_arbiter(&arbiter, |_ctx| Syncer::new())
    }

    fn hb(&mut self, ctx: &mut <Self as Actor>::Context) {
        let conn_addr = self.conn_addr.clone();
        let mut count = 0;
        ctx.run_interval(HEARTBEAT_INTERVAL, move |_act, _ctx| {
            log::info!("hb0000...");

            let conn_addr = conn_addr.clone();
            count += 1;
            tokio::spawn(lazy(move || {
                let (tx, rx): (Sender<ChannelItem>, Receiver<ChannelItem>) = oneshot::channel();
                conn_addr.do_send(MsgWithTx {
                    msg: Test { ref0: count },
                    tx,
                });

                let rx2 = Timeout::new(rx, SEND_TIMEOUT);
                rx2.and_then(|msg| {
                    println!("Got `{}`", msg.get_ref());
                    Ok(())
                })
                .map_err(move |e| log::info!("error = {:?}, count:{:?}", e, count))
            }));
        });
    }
}

#[derive(Debug)]
struct Test {
    ref0: u32,
}

impl Message for Test {
    type Result = ();
}

impl RefAware for Test {
    fn get_ref(&self) -> u32 {
        return self.ref0;
    }
}
