## Robot Finds Kitten

### Build instructions

Linux: Please make sure you have ncurses installed on you build.
- Debian/Ubuntu: `sudo apt-get install libncurses5-dev libncursesw5-dev`
- CentOS/RHEL/Fedora: `sudo yum install ncurses-devel`


Window: Please have a C compiler, either mingw or gcc


Rust: If you have not already installed Rust, please install rustc and cargo via this [link](https://www.rust-lang.org/tools/install)

### Game Build

You may easily run this game with `cargo run`

### Objective

This game is about rescuing a lost kitten! There are a number of items on the map that you may discover, but only one of them has the kitten. Try to find it in less than four trys, or else it might run away.

### Questions:
- I wanted to keep the game very simple and still zenlike, so I essentially made it the original, but with a counter, so you must find the kitten in less than 4 trys to win. Sadly, I had to make the symbols just letters, and I could not figure out a way to have this library support UNICODE characters.A

- I really wish I could have made the symbols different, and thinking about it now, I will probably try to make them random, as each sprite has the same symbol to represent it, making it very easy to beat.A

- I added a counter, although I forgot to make it visible on the board, which I will add soon. I think a timer would also be really cool, or maybe have the sprites try to move and attack you after a certain amount of time.

- I could not find any extreme bugs, and I think most of them are out of my control, like sometimes if I ctrl+c my program, it will just totally brick my terminal window, I could possibly add a signal listener to handle things like those.

- I had my mom play test the game and she enjoyed it, saying it was a little boring, which I could understand.

- I think a AAA version of this game would be 3D, with ray tracing, and perhaps some path finding for the sprite to start attacking you, somehow making it a little more interactive. Different zones and areas would be cool, as well as maybe abilities.


