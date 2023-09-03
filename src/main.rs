/// Robot Finds Kitten
/// Author: Christian Torralba
/// This is the first assignment for Rust Game Development
///
/// The purpose of this game is to move around the map and search each item
/// on the map in search of the kitten.
///
/// Once you find the kitten, the game ends and tells you how many times it took to find the kiten!
extern crate pancurses;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use pancurses::{endwin, initscr, noecho, Input, Window};
use rand::Rng;

/// This enum contains all the sprite types in this game,
/// It is made to be added to, as more sprite should be easy to add.
#[derive(EnumIter, Debug)]
pub enum SpriteType {
    Kitten,
    Robot,
    TrashCan,
    ManHole,
    OilContainer,
    Duster,
    Popcorn,
}

/// Struct position contains two variables,\
/// `x: i32` - Contains the x coordinate of the sprite\
/// `y: i32` - Contains the y coordinate of the sprite.
pub struct Position {
    x: i32,
    y: i32,
}

/// The main Sprite struct conatins a `Position` and a `SpriteType`, declaring the position,
/// and type of sprite.
pub struct Sprite {
    position: Position,
    sprite_type: SpriteType,
}

/// This struct is simply a collection of `Sprites`
pub struct SpriteGroup {
    sprite_list: Vec<Sprite>,
}

impl SpriteGroup {
    /// This funcion goes through each sprite in the `sprite_list` and updates the position and
    /// relative character of each one, and draws it to the `Window`
    ///
    /// Params:\
    /// - `window: &mut Window` - This is the window that we will be drawing to, a part of the
    /// `pancurses` Rust library
    pub fn refresh(&mut self, window: &mut Window) {
        for sprite in self.sprite_list.iter() {
            match &sprite.sprite_type {
                SpriteType::Kitten => {
                    window.mvaddch(sprite.position.y, sprite.position.x, 'K');
                }

                SpriteType::Robot => {
                    // Do nothing for robot, since this is our character
                }

                SpriteType::TrashCan => {
                    window.mvaddch(sprite.position.y, sprite.position.x, 'T');
                }

                SpriteType::ManHole => {
                    window.mvaddch(sprite.position.y, sprite.position.x, 'M');
                }

                SpriteType::OilContainer => {
                    window.mvaddch(sprite.position.y, sprite.position.x, 'O');
                }

                SpriteType::Duster => {
                    window.mvaddch(sprite.position.y, sprite.position.x, 'D');
                }

                SpriteType::Popcorn => {
                    window.mvaddch(sprite.position.y, sprite.position.x, 'P');
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

    /// This function takes two parameters,
    /// Returns an Option containing a reference to a sprite if there is one, or None
    /// if there was no Sprite with the specific location.
    pub fn get_sprite_at_position(&self, x: i32, y: i32) -> Option<&Sprite> {
        self.sprite_list
            .iter()
            .find(|&sprite| sprite.position.x == x && sprite.position.y == y)
    }

    /// Returns a pair of numbers that are not already in use by a sprite
    pub fn get_valid_random_points(&self, window: &mut Window) -> (i32, i32) {
        let mut rng = rand::thread_rng();
        let (mut rand_x, mut rand_y) = (
            rng.gen_range(1..window.get_max_x() - 1),
            rng.gen_range(1..window.get_max_y() - 1),
        );
        while self.is_occupied(rand_x, rand_y) {
            rand_x = rng.gen_range(1..=window.get_max_x() - 1);
            rand_y = rng.gen_range(1..=window.get_max_y() - 1);
        }
        (rand_x, rand_y)
    }

    /// Handles collision for various sprites,
    /// ### Returns:
    ///
    /// - true: Sprite collision involved the kitten
    /// - false: Sprite collision did not involve the kitten
    pub fn handle_collision(&self, x: i32, y: i32, window: &mut Window) -> bool {
        if let Some(sprite) = self.get_sprite_at_position(x, y) {
            match sprite.sprite_type {
                SpriteType::Kitten => {
                    window.mvprintw(0, 0, "You found the kitten! Good Job.");
                    return true;
                }

                SpriteType::Robot => {
                    // This is the player, have to determine what to do with this.
                }

                SpriteType::TrashCan => {
                    window.mvprintw(0, 0, "One persons trash is another ones treasure.");
                }

                SpriteType::ManHole => {
                    window.mvprintw(0, 0, "Careful! You almost fell into the sewer.");
                }

                SpriteType::OilContainer => {
                    window.mvprintw(0, 0, "Robots and oil? Sounds like a good mix to me.");
                }

                SpriteType::Duster => {
                    window.mvprintw(0, 0, "You get all dusted off! But no kitten. :(");
                }

                SpriteType::Popcorn => {
                    window.mvprintw(0, 0, "You found some popcorn, but you're a robot....");
                }
            }
        }
        false
    }
}

fn main() {
    // Init screen and enable input
    let mut window = initscr();
    window.keypad(true);

    // Random placement of points
    let (mut rand_x, mut rand_y);

    // Create sprite group
    let mut sprite_group: SpriteGroup = SpriteGroup {
        sprite_list: Vec::new(),
    };

    // Create a sprite of each type
    for sprite_type in SpriteType::iter() {
        (rand_x, rand_y) = sprite_group.get_valid_random_points(&mut window);
        let sprite = Sprite {
            position: Position {
                x: rand_x,
                y: rand_y,
            },
            sprite_type,
        };
        sprite_group.sprite_list.push(sprite);
    }

    // Draw sprites to screen
    sprite_group.refresh(&mut window);

    // Start cursor in the middle of the screen
    let (mut x_pos, mut y_pos) = (window.get_max_x() / 2, window.get_max_y() / 2);

    // Move cursor to starting position
    window.mv(y_pos, x_pos);

    // Draw box around borders
    window.draw_box(0, 0);

    // Refresh screen
    window.refresh();

    noecho();

    // Counter for collisions
    let mut collisions = 0;
    let mut win = true;
    loop {
        match window.getch() {
            // Basic input do nothing
            Some(Input::KeyDC) => break,

            // Movement
            Some(Input::KeyDown) => {
                if (y_pos + 1) < window.get_max_y() - 1 {
                    y_pos += 1;
                }
            }
            Some(Input::KeyUp) => {
                if (y_pos - 1) > 0 {
                    y_pos -= 1;
                }
            }
            Some(Input::KeyLeft) => {
                if (x_pos - 1) > 0 {
                    x_pos -= 1;
                }
            }
            Some(Input::KeyRight) => {
                if (x_pos + 1) < window.get_max_x() - 1 {
                    x_pos += 1;
                }
            }

            // Random case, do nothing
            Some(_) => (),

            None => (),
        }

        // Clear window and draw border
        window.clear();
        window.draw_box(0, 0);

        if sprite_group.is_occupied(x_pos, y_pos) {
            collisions += 1;
        }

        // Check for collision, if this returns true, we means it collided with a kitten
        if sprite_group.handle_collision(x_pos, y_pos, &mut window) {
            break;
        }

        // Losing condition
        if collisions > 5 {
            win = false;
            break;
        }

        // Update and draw sprites to window
        sprite_group.refresh(&mut window);

        // Move cursor back to starting position
        window.mv(y_pos, x_pos);

        // Refresh window
        window.refresh();
    }

    if win {
        window.clear();
        window.draw_box(0, 0);
        let congrats = "Congrats! You found the kitten!".to_string();
        let attempts = format!("It only took you {} times.", collisions);
        window.mvprintw(
            window.get_max_y() / 2,
            (window.get_max_x() / 2) - (congrats.len() as i32 / 2),
            congrats,
        );
        window.mvprintw(
            (window.get_max_y() / 2) + 1,
            (window.get_max_x() / 2) - (attempts.len() as i32 / 2),
            attempts,
        );
    } else {
        let loss = "Oh no! You didn't find the kitten in time.".to_string();
        window.clear();
        window.draw_box(0, 0);
        window.mvprintw(
            window.get_max_y() / 2,
            (window.get_max_x() / 2) - (loss.len() as i32 / 2),
            loss,
        );
    }
    window.refresh();

    window.getch();
    endwin();
}
