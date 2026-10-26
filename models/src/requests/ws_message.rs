use crate::Event;

pub enum WsMessage {
    Event(Event),
    Ping,
    Pong,
}
