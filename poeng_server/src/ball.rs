use crate::game::Game;
use rand::Rng;

pub struct Ball {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
}

impl Ball {
    pub fn new(x: f32, y: f32) -> Ball {
        Ball {
            x,
            y,
            vx: 1.,
            vy: 0.,
        }
    }

    pub fn tick(&mut self, game: &mut Game) {
        self.normalize_velocity();
        self.step();
        self.collide(game);
        self.normalize_velocity();
    }

    fn normalize_velocity(&mut self) {
        self.vy = self.vy.max(-1.5);
        self.vy = self.vy.min(1.5);
        let abs = (self.vx.powi(2) + self.vy.powi(2)).sqrt();
        self.vx /= abs;
        self.vy /= abs;
    }

    fn step(&mut self) {
        self.x += self.vx;
        self.y += self.vy;
    }

    fn collide(&mut self, game: &mut Game) {
        if self.x < 0. {
            self.vx *= -1.;
            self.vy += rand::thread_rng().gen_range(-0.5, 0.5);
            self.x = self.x * -1.;
            if (self.y - game.left_player.y).abs() > game.pad_height / 2. {
                game.right_player.points += 1;
            }
        } else if self.x > game.width {
            self.vx *= -1.;
            self.vy += rand::thread_rng().gen_range(-0.5, 0.5);
            self.x = 2. * game.width - self.x;
            if (self.y - game.right_player.y).abs() > game.pad_height / 2. {
                game.left_player.points += 1;
            }
        }

        if self.y < 0. {
            self.vy *= -1.;
            self.y = self.y * -1.;
        } else if self.y > game.height {
            self.vy *= -1.;
            self.y = 2. * game.height - self.y;
        }
    }
}
