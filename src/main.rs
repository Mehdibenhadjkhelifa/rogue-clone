use tcod::colors::*;
use tcod::console::*;
const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const FPS_LIMIT: i32 = 20;

struct Tcod {
    root: Root,
}
fn handle_keys(tcod: &mut Tcod, player_x: &mut i32, player_y: &mut i32) -> bool {
    use tcod::input::Key;
    use tcod::input::KeyCode::*;
    let key = tcod.root.wait_for_keypress(true);
    match key {
        Key {
            code: Enter,
            alt: true,
            ..
        } => tcod.root.set_fullscreen(!tcod.root.is_fullscreen()), // enter+alt to switch between fullscreen and windowed modes.
        Key { code: Escape, .. } => return true, //escape to exit game by returning true
        //mapping keyboard input to player movement
        Key { code: Up, .. } => *player_y -= 1,
        Key { code: Down, .. } => *player_y += 1,
        Key { code: Left, .. } => *player_x -= 1,
        Key { code: Right, .. } => *player_x += 1,
        _ => {}
    }
    false
}

fn main() {
    //Game initialisation
    let root = Root::initializer()
        .font("res/arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Rogue_Clone")
        .init();
    let mut tcod = Tcod { root };
    tcod::system::set_fps(FPS_LIMIT);

    let mut player_x: i32 = SCREEN_WIDTH / 2;
    let mut player_y: i32 = SCREEN_HEIGHT / 2;

    //game loop
    while !tcod.root.window_closed() {
        tcod.root.set_default_foreground(WHITE);
        tcod.root.clear();
        tcod.root
            .put_char(player_x, player_y, '@', BackgroundFlag::None);
        tcod.root.flush();
        let exit = handle_keys(&mut tcod, &mut player_x, &mut player_y);
        if exit {
            break;
        }
    }
}
