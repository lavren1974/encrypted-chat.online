use actix::prelude::*;
use rand::{self, rngs::ThreadRng, Rng};
use serde::Serialize;
use std::collections::HashMap;

use crate::session::WsClientSession;

// Структура SessionData используется для хранения данных, связанных с каждым сеансом WebSocket. 
// В этом случае он просто хранит адрес актера WsClientSession клиента.
struct SessionData {
    client_addr: Addr<WsClientSession>,
}

pub struct ChatManager {
    sessions: HashMap<usize, SessionData>,
    rng: ThreadRng,
}

impl ChatManager {
    pub fn new() -> ChatManager {
        ChatManager {
            rng: rand::thread_rng(),
            sessions: HashMap::new(),
        }
    }
}

impl Actor for ChatManager {
    type Context = Context<Self>;
}

// Connect: этот обработчик вызывается, когда новый клиент WebSocket подключается к серверу. 
// Он генерирует новый идентификатор клиента и сохраняет адрес клиента в хэш-карте.
impl Handler<Connect> for ChatManager {
    type Result = usize;

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        let client_id = self.rng.gen::<usize>();

        println!("Joined (ID: {})", client_id);

        self.sessions.insert(
            client_id,
            SessionData {
                client_addr: msg.client_addr,
            },
        );
        client_id
    }
}

// Disconnect: этот обработчик вызывается, когда клиент WebSocket отключается от сервера. 
// Он удаляет данные сеанса клиента из хэш-карты.
impl Handler<Disconnect> for ChatManager {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        println!("Disconnected (ID: {})", &msg.client_id);
        self.sessions.remove(&msg.client_id);
    }
}


// ChatMessage: этот обработчик вызывается, когда клиент WebSocket отправляет сообщение чата на сервер. 
// Он рассылает сообщение всем подключенным клиентам, кроме отправителя.
impl Handler<ChatMessage> for ChatManager {
    type Result = ();

    fn handle(&mut self, msg: ChatMessage, _: &mut Context<Self>) {
        for (id, session) in &self.sessions {
            if *id != msg.client_id {
                session.client_addr.do_send(msg.clone());
            }
        }
    }
}

#[derive(Message)]
#[rtype(usize)]
pub struct Connect {
    pub client_addr: Addr<WsClientSession>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub client_id: usize,
}

#[derive(Clone, Message, Serialize)]
#[rtype(result = "()")]
pub struct ChatMessage {
    pub client_id: usize,
    pub message: String,
}


// Сообщение Connect возвращает идентификатор нового подключенного клиента. Сообщение ChatMessage ничего не возвращает.

// Код использует инфраструктуру акторов Actix для управления сеансами и сообщениями WebSocket. 
// Он также использует крейт rand для генерации случайных идентификаторов клиентов и крейт serde для сериализации сообщений чата.

