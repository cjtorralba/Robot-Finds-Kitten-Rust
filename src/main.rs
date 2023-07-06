extern crate pancurses;

use pancurses::{initscr, endwin, Input, noecho, set_blink, Window};

pub enum SpriteType {
    Kitten,
    Robot,
    TrashCan,
    ManHole,
    OilContainer

}

pub struct Position {
    x: i32,
    y: i32,
}

pub struct Sprite {
    position: Position,
    sprite_type: SpriteType,
}

pub struct SpriteGroup {
    sprite_list: Vec<Sprite>,
}

impl SpriteGroup {
    pub fn refresh(&mut self, window: &mut Window) {
        for sprite in self.sprite_list.iter() {
            match &sprite.sprite_type {
                SpriteType::Kitten => {
                    window.mvaddch(sprite.position.y, sprite.position.x, 'K');
                },

                SpriteType::Robot => {
                    window.mvaddch(sprite.position.y, sprite.position.x, 'R');
                },

                SpriteType::TrashCan => {
                    window.mvaddch(sprite.position.y, sprite.position.x, 'T');
                },

                SpriteType::ManHole => {

                    window.mvaddch(sprite.position.y, sprite.position.x, 'M');
                },

                SpriteType::OilContainer => {
                    window.mvaddch(sprite.position.y, sprite.position.x, 'O');
                },

                _ => {

                }
            }
        }

    }


    /// This function takes an X and Y coordinate and returns true if the space is currently
    /// occupied by a sprite, and false if the space is not occupied by anything
    pub fn is_occupied(&self, x: i32, y: i32) -> bool {
        for sprite in self.sprite_list.iter() {
            if sprite.position.x == x && sprite.position.y == y {
                return true;
            }
        }
        false
    }

    pub fn get_sprite_at_position(&self, x: i32, y: i32) -> Option<&Sprite> {
        for sprite in self.sprite_list.iter() {
            if sprite.position.x == x && sprite.position.y == y {
                return Some(sprite);
            }
        }
        None
    }


    // Handles collision
    pub fn handle_collision(&self, x: i32, y: i32, window: &mut Window) -> bool {
        match self.get_sprite_at_position(x, y) {
            Some(sprite) => {
                match sprite.sprite_type {
                    SpriteType::Kitten => {
                        window.mvprintw(0, 0, "You found the kitten! Good Job.".to_string());
                        return true;
                    },

                    SpriteType::Robot => {
                        // This is the player, have to determine what to do with this.
                    },

                    SpriteType::TrashCan => {
                        window.mvprintw(0, 0, "One persons trash is another ones treasure.".to_string());
                    },

                    SpriteType::ManHole => {
                        window.mvprintw(0, 0, "Careful! You almost fell into the sewer.".to_string());

                    },

                    SpriteType::OilContainer => {
                        window.mvprintw(0, 0, "Robots and oil? Sounds like a good mix to me.".to_string());

                    },
                    _ => {}
                }
            }
            None => {}
        }
        false
    }
}









fn main() {

    // Initialize sprites 
    let mut kitten: Sprite = Sprite {
        position: Position {
            x: 5,
            y: 5,
        },
        sprite_type: SpriteType::Kitten,
    };
    let mut trash_can: Sprite = Sprite {
        position: Position {
            x: 25,
            y: 25,
        },
        sprite_type: SpriteType::TrashCan,
    };
    let mut robot: Sprite = Sprite {
        position: Position {
            x: 40,
            y: 25,
        },
        sprite_type: SpriteType::Robot,
    };

    // Create sprite group
    let mut sprite_group: SpriteGroup = SpriteGroup {
        sprite_list: vec![kitten, trash_can, robot],
    };



    // Init screen and enable input
    let mut window = initscr();
    window.keypad(true); 


    // Draw sprites to screen
    sprite_group.refresh(&mut window);



    // Start cursor in the middle of the screen
    let (mut prev_x, mut prev_y) = (0,0);
    let (mut x_pos, mut y_pos) = (window.get_max_x() / 2, window.get_max_y() / 2);


    // Move cursor to starting position
    window.mv(y_pos, x_pos);


    // Draw box around borders
    window.draw_box(0,0);

    // Refresh screen
    window.refresh();



    noecho();

    loop {
        match window.getch() {

            // Basic input do nothing

            Some(Input::KeyDC) => break,


            // Movement
            Some(Input::KeyDown) => {
                if (y_pos + 1) < window.get_max_y() - 1 {
                    prev_y = y_pos;
                    y_pos += 1;
                }

            },
            Some(Input::KeyUp) => {
                if (y_pos - 1) > 0 {
                    prev_y = y_pos;
                    y_pos -= 1;
                }
            },
            Some(Input::KeyLeft) => {
                if (x_pos - 1) > 0 {
                    prev_x = x_pos;
                    x_pos -= 1;
                }
            },
            Some(Input::KeyRight) => {
                if (x_pos + 1) < window.get_max_x() - 1 {
                    prev_x = x_pos;
                    x_pos += 1;
                }
            },

            Some(input) => (),
            Some(Input::Character(c)) => (),

            None => ()
        }

        // Clear window and draw border
        window.clear();
        window.draw_box(0,0);

        // Check for collision, if this returns true, we means it collided with a kitten
        if sprite_group.handle_collision(x_pos, y_pos, &mut window) {
           break; 
        }


        // Update and draw sprites to window
        sprite_group.refresh(&mut window);

        // Move cursor back to starting position
        window.mv(y_pos, x_pos);

        // Refresh window
        window.refresh();
    }

    window.clear();
    window.draw_box(0,0);
    let congrats = "Congratulations".to_string();
    window.bkgd(20 as u32);
    window.mvprintw(window.get_max_y() / 2, 
                    window.get_max_x() / 2 - congrats.len() as i32,
                    "Congrtulations");
    window.color_set(5 as i16);
    window.bkgd(5 as u32);
    window.refresh();
    window.color_set(5 as i16);
    window.bkgd(5 as u32);
 
    window.getch();
    endwin();



}
