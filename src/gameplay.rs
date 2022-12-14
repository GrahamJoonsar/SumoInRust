#![allow(dead_code)]
pub mod game {
    use macroquad::prelude::*;
    fn dist_center_squared(player: &Player) -> f32 {
        (player.x - screen_width()/2.0).powf(2.0) + (player.y - screen_height()/2.0).powf(2.0)
    }
    
    pub fn check_winner(player_1: &Player, player_2: &Player, text_for_winner: &mut String) {
        if *text_for_winner == String::from(""){
            if dist_center_squared(player_1) > 50625.0 {
                *text_for_winner = String::from("Player 2 Wins!");
            }
            if dist_center_squared(player_2) > 50625.0 {
                *text_for_winner = String::from("Player 1 Wins!");
            }
        }
    }

    pub fn check_collision(player_1: &mut Player, player_2: &mut Player){
        if ((player_1.x - player_2.x).powf(2.0) + (player_1.y - player_2.y).powf(2.0)).sqrt() < player_1.r + player_2.r {
            player_1.bounce((player_1.y-player_2.y).atan2(player_1.x-player_2.x));
            player_2.bounce((player_2.y-player_1.y).atan2(player_2.x-player_1.x));
        }
    }
    
    pub struct ControlScheme {
        up: KeyCode,
        down: KeyCode,
        left: KeyCode,
        right: KeyCode,
    }
    
    impl ControlScheme {
        pub fn new (up: KeyCode, down: KeyCode, left: KeyCode, right: KeyCode) -> ControlScheme {
            ControlScheme{
                up,
                down,
                left,
                right,
            }
        }
    }
    
    pub struct Player {
        x: f32,
        original_x: f32,
        y: f32,
        original_y: f32,
        r: f32,
        color: Color,
    
        x_vel: f32,
        y_vel: f32,
        
        speed: f32,
        drag: f32,
        bounciness: f32,
    
        controls: ControlScheme,
    }
    
    impl Player {
        pub fn new (x: f32, y: f32, r: f32, color: Color, controls: ControlScheme) -> Player {
            Player {
                x,
                original_x: x,
                y,
                original_y: y,
                r,
                color,
                x_vel: 0.0,
                y_vel: 0.0,
                speed: 0.1,
                drag: 0.05,
                bounciness: 3.0,
                controls,
            }
        }
        
        pub fn reset (&mut self) {
            self.x = self.original_x;
            self.y = self.original_y;

            self.x_vel = 0.0;
            self.y_vel = 0.0;
        }

        pub fn draw (&self) {
            // Drawing a circle representing the player to the screen
            draw_circle(self.x, self.y, self.r, self.color);
        }
    
        pub fn update (&mut self) {
            // Vertical Movement
            if is_key_down(self.controls.up) {
                self.y_vel -= self.speed;
            }
            if is_key_down(self.controls.down) {
                self.y_vel += self.speed;
            }
    
            // Horizontal Movement
            if is_key_down(self.controls.left) {
                self.x_vel -= self.speed;
            }
            if is_key_down(self.controls.right) {
                self.x_vel += self.speed;
            }
            
            // Keeping the player in the window
            if (self.y + self.y_vel > screen_height()) || (self.y + self.y_vel < 0.0) {
                self.y_vel *= -1.0;
            }
            if (self.x + self.x_vel > screen_width()) || (self.x + self.x_vel < 0.0) {
                self.x_vel *= -1.0;
            }
    
            // Move the player
            self.x += self.x_vel;
            self.y += self.y_vel;
    
            // Implementing drag
            if self.x_vel > self.drag {
                self.x_vel -= self.drag/2.0;
            } else if self.x_vel < -self.drag {
                self.x_vel += self.drag/2.0;
            } else {
                self.x_vel = 0.0;
            }
    
            if self.y_vel > self.drag {
                self.y_vel -= self.drag/2.0;
            } else if self.y_vel < -self.drag {
                self.y_vel += self.drag/2.0;
            } else {
                self.y_vel = 0.0;
            }
        }

        pub fn bounce (&mut self, angle: f32) {
            self.x_vel = angle.cos() * self.bounciness;
            self.y_vel = angle.sin() * self.bounciness;
        }
    }
}