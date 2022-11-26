use std::net::{SocketAddr, TcpStream};
use std::sync::{ mpsc::Sender as SyncSender};
use anathema::display::{Attributes, Color, Style};
use anathema::runtime::{Event, KeyCode, KeyEvent, KeyModifiers, Runtime};
use anathema::templates::DataCtx;
use anathema::widgets::{NodeId, ScrollView, Text, TextSpan, Widget, WidgetContainer};
use crosslogger::core::logger::LogType;
use crosslogger_server::server::{Log, Server};

pub enum RunnerEvent {
    ServerEvent(Log),
    UserConnected(SocketAddr),
    UserDisconnected(SocketAddr),
    ServerStarted,
}

struct Runner {

}

impl Runner {
    pub fn start() {
        // create the runtime of type `RunnerEvent`
        let runtime = Runtime::<RunnerEvent>::new();

        // load the template from the file into a string
        let template = include_str!("templates/main.tiny");

        // save the sender into a variable [tx]
        let tx = runtime.tx();

        // Spawn a new thread to run the server
        std::thread::spawn(move ||{
            let serv: Server = Server::new(8787);
            let _ = tx.send(Event::User(RunnerEvent::ServerStarted));
            loop {
                let mut connection: Option<TcpStream> = serv.accept();
                match connection {
                    Some(_) => {
                        let address = connection.as_ref().unwrap().peer_addr().unwrap();
                        let _ = tx.send(Event::User(RunnerEvent::UserConnected(address.clone())));
                        loop {
                            let packet = serv.receive_packet(connection.as_mut().unwrap());
                            match packet {
                                Some(packet) => {
                                    let _ = tx.send(Event::User(RunnerEvent::ServerEvent(packet)));
                                },
                                None => {
                                    break;
                                }
                            }
                        }
                        let _ = tx.send(Event::User(RunnerEvent::UserDisconnected(address.clone())));
                    },
                    None => {}
                }
            }
        });


        // Run TUI
        runtime.start(template, DataCtx::with_value("ip", "N/A"), |_ev: Event<RunnerEvent>,
                                             _root: &mut WidgetContainer,
                                             _ctx: &mut DataCtx,
                                             _tx: &mut SyncSender<Event<RunnerEvent>>| {
            // Do TUI event handling here
            do_events(_ev, _root, _ctx, _tx);
        }).unwrap();
    }
}

pub fn run() {
    Runner::start();
}

fn do_events(_ev: Event<RunnerEvent>,
             _root: &mut WidgetContainer,
             _ctx: &mut DataCtx,
             _tx: &mut SyncSender<Event<RunnerEvent>>) {
    if _ev.ctrl_c() {
        _tx.send(Event::Quit).unwrap();
    }

    let scroll_id = NodeId::Value(4u64.into());

    if let Event::User(RunnerEvent::ServerEvent(ref log)) = _ev {
        let scroll_view: &mut ScrollView = _root.by_id(&scroll_id).unwrap().to::<ScrollView>();


        if scroll_view.children().len() > 100 {
            scroll_view.children.remove(0);
        }
        scroll_view.add_child(create_log_widget(&log));
        scroll_view.scroll_forward(1000);
    }
    else if let Event::User(RunnerEvent::ServerStarted) = _ev {
        *_ctx.get_string_mut("ip").unwrap() = "0.0.0.0:8787".to_string();
    }

    // if KeyCode is Up, then scroll up
    let scroll_view = _root.by_id(&scroll_id).unwrap().to::<ScrollView>();

    if let Event::Key(KeyEvent { code: KeyCode::Down, modifiers: KeyModifiers::NONE }) = _ev {
        scroll_view.scroll_forward(1);
    } else if let Event::Key(KeyEvent { code: KeyCode::Up, modifiers: KeyModifiers::NONE }) = _ev {
        scroll_view.scroll_back(1);
    }
}

fn create_log_widget(log: &Log) -> WidgetContainer {
    let mut text = Text::new();

    // TIME
    text.add_span(TextSpan {
        // format the time to 12 hour format
        text: format!("[{}] ", chrono::Local::now().format("%I:%M:%S %p")),
        style: Style {
            fg: Some(Color::Grey),
            bg: None,
            attributes: Attributes::empty(),
        },
    });

    // TYPE
    text.add_span(TextSpan {
        text: format!("[{}]", log.log_type.to_string()),
        style: Style {
            fg: match log.log_type {
                LogType::INFO => Some(Color::Blue),
                LogType::WARN => Some(Color::Yellow),
                LogType::ERROR => Some(Color::Red),
            },
            bg: None,
            attributes: Attributes::empty(),
        },
    });

    // MESSAGE
    text.add_span(TextSpan {
        text: format!(" {}", log.message),
        style: Style {
            fg: None,
            bg: None,
            attributes: Attributes::empty(),
        },
    });
    text.into_container(NodeId::auto())
}