use crate::settings::SETTINGS;
use crate::sync::ref_aware::RefAware;
use actix::{io::SinkWrite, prelude::*};
use actix_codec::Framed;
use awc::{
    error::WsProtocolError,
    ws::{Codec, Frame, Message as WsMessage},
    BoxedSocket, Client,
};
use futures::{
    lazy,
    stream::{SplitSink, Stream},
    sync::oneshot::Sender,
    Future,
};
use std::collections::HashMap;
use std::time::Duration;

const RECONNECT_INTERVAL: Duration = Duration::from_secs(1);

pub type ChannelItem = Box<dyn RefAware<Result = ()>>;

////////////////////////////////////////////////////////////////////////////////
/// Connection part
////////////////////////////////////////////////////////////////////////////////
pub struct Connection {
    ws_addr: Option<Addr<Websocket>>,
}

impl Actor for Connection {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        log::info!("Connection actor started.");
        self.connect(ctx);
    }

    fn stopped(&mut self, _: &mut Context<Self>) {
        log::info!("Connection actor stopped.");
    }
}

impl Handler<OnWebsocketCreated> for Connection {
    type Result = ();

    fn handle(&mut self, msg: OnWebsocketCreated, _ctx: &mut Context<Self>) {
        log::info!("Received msg: OnWebsocketCreated");
        self.ws_addr = Some(msg.ws_addr.clone());
    }
}

impl Handler<ConnectCommand> for Connection {
    type Result = ();

    fn handle(&mut self, _msg: ConnectCommand, ctx: &mut Context<Self>) {
        log::info!("Received msg: ConnectCommand");
        self.connect(ctx);
    }
}

impl Handler<ReconnectCommand> for Connection {
    type Result = ();

    fn handle(&mut self, _msg: ReconnectCommand, ctx: &mut Context<Self>) {
        log::info!("Received msg: ReconnectCommand");
        self.ws_addr = None;
        ctx.run_later(RECONNECT_INTERVAL, |_act, ctx2| {
            ctx2.address().do_send(ConnectCommand);
        });
    }
}

impl<T: RefAware<Result = ()>> Handler<MsgWithTx<T>> for Connection {
    type Result = ();

    fn handle(&mut self, msg: MsgWithTx<T>, _: &mut Context<Self>) {
        log::info!("received msg*********: {:?}", msg.get_ref());
        if self.ws_addr.is_some() {
            self.ws_addr.as_ref().unwrap().do_send(msg);
        } else {
            let msg2: ChannelItem = Box::new(Test2 { ref0: 1000 });
            msg.tx
                .send(msg2)
                .map(|res| log::info!("result: {:?}", res))
                .map_err(|_e| log::info!("error = "))
                .unwrap();
        }
    }
}

impl Connection {
    pub fn new() -> Self {
        Connection { ws_addr: None }
    }

    pub fn start() -> Addr<Connection> {
        let conn = Connection::new();
        Actor::start(conn)
    }

    fn connect(&self, ctx: &mut <Self as Actor>::Context) {
        let conn_addr = ctx.address();
        let conn_addr2 = conn_addr.clone();
        log::info!("Connecting: endpoint: {:?}", Self::build_url());
        Arbiter::spawn(lazy(|| {
            Client::new()
                .ws(Self::build_url())
                .connect()
                .map_err(move |err| {
                    log::info!(
                        "Failed to connect: endpoint: {:?}, error: {:?}",
                        Self::build_url(),
                        err
                    );
                    conn_addr.do_send(ReconnectCommand);
                })
                .map(|(response, framed)| {
                    log::info!(
                        "Connection connected: endpoint: {:?}, response: {:?}",
                        Self::build_url(),
                        response
                    );
                    let (sink, stream) = framed.split();
                    Websocket::create(|ctx2| {
                        Websocket::add_stream(stream, ctx2);
                        Websocket {
                            sink: SinkWrite::new(sink, ctx2),
                            conn_addr: conn_addr2,
                            mapping: HashMap::new(),
                        }
                    });
                })
        }));
    }

    fn build_url() -> String {
        format!("{}{}", "http://", SETTINGS.master_endpoint)
    }
}

////////////////////////////////////////////////////////////////////////////////
/// Websocket part
////////////////////////////////////////////////////////////////////////////////
struct Websocket {
    sink: SinkWrite<SplitSink<Framed<BoxedSocket, Codec>>>,
    conn_addr: Addr<Connection>,
    mapping: HashMap<u32, Sender<ChannelItem>>,
}

impl Actor for Websocket {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        log::info!("Websocket actor started.");
        self.conn_addr.do_send(OnWebsocketCreated {
            ws_addr: ctx.address(),
        });
    }

    fn stopped(&mut self, _: &mut Context<Self>) {
        log::info!("Websocket actor stopped.");
    }
}

impl<T: RefAware<Result = ()>> Handler<MsgWithTx<T>> for Websocket {
    type Result = ();

    fn handle(&mut self, msg: MsgWithTx<T>, _: &mut Context<Self>) {
        log::info!("received msg2: {:?}", msg.get_ref());
        self.mapping.insert(msg.get_ref(), msg.tx);
        self.sink.write(WsMessage::Ping(String::new())).unwrap();
    }
}

impl StreamHandler<Frame, WsProtocolError> for Websocket {
    fn handle(&mut self, msg: Frame, _ctx: &mut Context<Self>) {
        match msg {
            Frame::Ping(ping) => println!("Server: {:?}", ping),
            Frame::Pong(pong) => println!("pong: {:?}", pong),
            Frame::Text(txt) => println!("Server: {:?}", txt),
            Frame::Binary(bin) => println!("Server: {:?}", bin),
            Frame::Close(close) => println!("Server: {:?}", close),
        }
    }

    fn started(&mut self, _ctx: &mut Context<Self>) {
        log::info!("Websocket stream opened.");
    }

    fn finished(&mut self, ctx: &mut Context<Self>) {
        log::info!("Websocket stream closed.");
        self.conn_addr.do_send(ReconnectCommand);
        ctx.stop();
    }
}

impl actix::io::WriteHandler<WsProtocolError> for Websocket {}

////////////////////////////////////////////////////////////////////////////////
/// Protocol part
////////////////////////////////////////////////////////////////////////////////

#[derive(Message)]
#[rtype(())]
struct OnWebsocketCreated {
    ws_addr: Addr<Websocket>,
}

#[derive(Message)]
#[rtype(())]
struct ConnectCommand;

#[derive(Message)]
#[rtype(())]
struct ReconnectCommand;

#[derive(Message)]
#[rtype(())]
pub struct MsgWithTx<T: RefAware<Result = ()>> {
    pub msg: T,
    pub tx: Sender<ChannelItem>,
}

impl<T: RefAware<Result = ()>> RefAware for MsgWithTx<T> {
    fn get_ref(&self) -> u32 {
        self.msg.get_ref()
    }
}

#[derive(Debug)]
struct Test2 {
    ref0: u32,
}

impl Message for Test2 {
    type Result = ();
}

impl RefAware for Test2 {
    fn get_ref(&self) -> u32 {
        return self.ref0;
    }
}
