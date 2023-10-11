use tcod::colors::*;
use tcod::console::*;

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const FPS_LIMIT: i32 = 20;

struct Tcod {
    root : Root,
}



fn main() {
    let root = Root::initializer()
        .font("res/arial10x10.png",FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH,SCREEN_HEIGHT)
        .title("Rogue_Clone")
        .init();
    let mut tcod = Tcod { root };
    tcod::system::set_fps(FPS_LIMIT);
    while !tcod.root.window_closed() {
        tcod.root.set_default_foreground(WHITE);
        tcod.root.clear();
        tcod.root.put_char(1,1,'@',BackgroundFlag::None);
        tcod.root.flush();
        tcod.root.wait_for_keypress(true);
    }
}
