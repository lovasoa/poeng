use uuid::Uuid;

pub enum Message {
    Register,
    Play(Uuid),
    Config(Uuid),
    Side(Uuid),
    Points(Uuid),
    Opponent(Uuid),
    Ball(Uuid),
    Move(Uuid, f32),
    Exit(Uuid),
}

impl Message {
    pub fn from(msg: &str) -> Option<(Message, String)> {
        if msg.starts_with("register") {
            Some((Message::Register, "register".to_string()))
        } else {
            let mut parts = msg.split_whitespace();
            let id = match parts.next()?.parse() {
                Ok(id) => id,
                Err(_) => return None,
            };

            let command = parts.next()?;
            let msg = match command {
                "play" => Some(Message::Play(id)),
                "config" => Some(Message::Config(id)),
                "side" => Some(Message::Side(id)),
                "points" => Some(Message::Points(id)),
                "opponent" => Some(Message::Opponent(id)),
                "ball" => Some(Message::Ball(id)),
                "move" => Some(Message::Move(
                    id,
                    match parts.next()?.parse() {
                        Ok(pos) => pos,
                        Err(_) => return None,
                    },
                )),
                "exit" => Some(Message::Exit(id)),
                _ => None,
            };

            match msg {
                Some(msg) => Some((msg, command.to_string())),
                None => None,
            }
        }
    }
}
