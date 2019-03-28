use crate::ball::Ball;
use crate::player::Player;
use std::sync::MutexGuard;
use std::time::{Duration, SystemTime};
use uuid::Uuid;

pub struct Game {
    pub width: f32,
    pub height: f32,
    pub pad_height: f32,
    pub left_player: Player,
    pub right_player: Player,
    pub ball: Option<Ball>,
    pub last_activity: SystemTime,
}

impl Game {
    pub fn new(mut left_player: Player, mut right_player: Player) -> Game {
        let width = 100.;
        let height = 60.;

        left_player.y = height / 2.;
        right_player.y = height / 2.;

        Game {
            width,
            height,
            pad_height: 20.,
            left_player: left_player,
            right_player: right_player,
            ball: Some(Ball::new(width / 2., height / 2.)),
            last_activity: SystemTime::now(),
        }
    }

    pub fn tick(&mut self) {
        let mut ball = self.ball.take().unwrap();
        ball.tick(self);
        self.ball = Some(ball);
    }

    pub fn is_active(&self) -> bool {
        self.last_activity.elapsed().unwrap() < Duration::from_secs(60)
    }
}

pub fn find_game<'a>(id: Uuid, games_guard: &'a MutexGuard<Vec<Game>>) -> Option<&'a Game> {
    games_guard
        .iter()
        .find(|g| g.left_player.id == id || g.right_player.id == id)
}

pub fn find_game_mut<'a>(
    id: Uuid,
    games_guard: &'a mut MutexGuard<Vec<Game>>,
) -> Option<&'a mut Game> {
    games_guard
        .iter_mut()
        .find(|g| g.left_player.id == id || g.right_player.id == id)
}
