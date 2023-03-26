#[cfg(test)]
mod tests {
    use super::*;
    use actix::{Actor, System};

    #[actix_rt::test]
    async fn test_chat_manager() {
        // Create a new chat manager
        let chat_manager = ChatManager::new().start();

        // Connect a client to the chat
        let client_addr = WsClientSession::default().start();
        let client_id = chat_manager.send(Connect { client_addr }).await.unwrap();

        // Send a chat message
        let message = "Hello, world!".to_string();
        chat_manager
            .send(ChatMessage {
                client_id,
                message: message.clone(),
            })
            .await
            .unwrap();

        // Disconnect the client from the chat
        chat_manager
            .send(Disconnect { client_id })
            .await
            .unwrap();

        // Check that the client was disconnected
        let sessions = chat_manager
            .send(actix::dev::MessageEnvelope::new(
                crate::session::GetSessions,
                actix::dev::RecipientRequest::new(),
            ))
            .await
            .unwrap()
            .unwrap();
        assert_eq!(sessions.len(), 0);
    }
}
