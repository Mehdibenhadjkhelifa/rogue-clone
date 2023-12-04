use tcod::colors::*;
use tcod::console::*;
const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const FPS_LIMIT: i32 = 20;
const MAP_WIDTH: i32 = 80;
const MAP_HEIGHT: i32 = 45;
const COLOR_DARK_WALL: Color = Color { r: 0, g: 0, b: 100 };
const COLOR_DARK_GROUND: Color = Color {
    r: 50,
    g: 50,
    b: 150,
};
type Map = Vec<Vec<Tile>>;
struct Game {
    map: Map,
}
fn make_map() -> Map {
    let mut map = vec![vec![Tile::empty(); MAP_HEIGHT as usize]; MAP_WIDTH as usize];
    map[30][22] = Tile::wall();
    map[50][22] = Tile::wall();
    map
}
struct Tcod {
    root: Root,
    con: Offscreen,
}
#[derive(Clone,Copy,Debug)]
struct Tile {
    blocked: bool,
    block_sight: bool,
}
impl Tile {
    pub fn empty() -> Self {
        Tile { blocked : false, block_sight: false,}
    }
    pub fn wall() -> Self {
        Tile { blocked : true, block_sight: true,}
    }
}
#[derive(Clone, Copy, Debug)]
struct Rect{
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}
impl Rect{
    pub fn new(x: i32,y: i32,w: i32,h: i32) -> Self{
        Rect{
            x1: x,
            y1: y,
            x2: x + w,
            y2: y + h,
        }
    }
}
fn create_room(room: Rect, map: &mut Map){
    for x in (room.x1 + 1)..room.x2{
        for y in (room.y1 + 1)..room.y2{
            map[x as usize][y as usize] = Tile::empty();
        }
    }
}
#[derive(Debug)]
struct Object {
    x: i32,
    y: i32,
    char: char,
    color: Color,
}
impl Object {
    pub fn new(x: i32,y: i32,char: char,color: Color) -> Self{
        Object {x,y,char,color}
    }
    pub fn move_by(&mut self,dx: i32, dy: i32, game: &Game) {
        if !game.map[(self.x + dx) as usize][(self.y + dy) as usize].blocked {
            self.x+= dx;
            self.y+= dy;
        }
    }
    pub fn draw(&self , con:&mut dyn Console) {
        con.set_default_foreground(self.color);
        con.put_char(self.x,self.y,self.char,BackgroundFlag::None);
    }


}
fn handle_keys(tcod: &mut Tcod, game: &Game, player: &mut Object) -> bool {
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
        Key { code: Up, .. } =>player.move_by(0,-1,&game), 
        Key { code: Down, .. } => player.move_by(0,1,game),
        Key { code: Left, .. } => player.move_by(-1,0,game),
        Key { code: Right, .. } => player.move_by(1,0,game),
        _ => {}
    }
    false
}
fn render_all(tcod: &mut Tcod, game: &Game, objects: &[Object]){
    for object in objects{
        object.draw(&mut tcod.con);
    }
    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH{
            let wall = game.map[x as usize][y as usize].block_sight;
            if wall {
            tcod.con.set_char_background(x,y,COLOR_DARK_WALL,BackgroundFlag::Set);
            }
            else{
                tcod.con.set_char_background(x,y,COLOR_DARK_GROUND,BackgroundFlag::Set);
            }
        }
    }
    blit(&tcod.con,(0,0),(SCREEN_WIDTH,SCREEN_HEIGHT),&mut tcod.root,(0,0),1.0,1.0);

}
fn main() {
    //Game initialisation
    let root = Root::initializer()
        .font("res/arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Rogue_Clone")
        .init();
    let con = Offscreen::new(MAP_WIDTH,MAP_HEIGHT);
    let mut tcod = Tcod { root , con };
    tcod::system::set_fps(FPS_LIMIT);
    //create the player
    let player = Object::new(SCREEN_WIDTH / 2,SCREEN_HEIGHT / 2,'@',WHITE);
    let npc = Object::new(SCREEN_WIDTH / 2,SCREEN_HEIGHT / 2,'@',RED);
    let mut objects = [player , npc];
    let game = Game { map:make_map(),};
    //game loop
    while !tcod.root.window_closed() {
        tcod.con.clear();
        render_all(&mut tcod,&game,&objects);
        tcod.root.flush();
        let  player = &mut objects[0];
        let exit = handle_keys(&mut tcod,&game,player);
        if exit {
            break;
        }
    }
    
}
