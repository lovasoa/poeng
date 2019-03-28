use uuid::Uuid;

pub struct Player {
    pub id: Uuid,
    pub points: u32,
    pub y: f32,
}

impl Player {
    pub fn new(id: Uuid) -> Player {
        Player {
            id,
            points: 0,
            y: 0.,
        }
    }
}
