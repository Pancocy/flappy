use bracket_lib::prelude::*;
const SCREEN_WIDTH : i32 = 40;
const SCREEN_HEIGHT : i32 = 25;
const FRAME_DURATION : f32 = 75.0;
const DRAGON_FRAMES : [u16; 6] = [ 64, 1, 2, 3, 2, 1 ];
//游戏模式
enum GameMode {
    Menu,
    Playing,
    End
}
//存储游戏状态
pub struct State {
    //存储模式到状态中
    mode: GameMode,
    frame_time:f32,
    player:Player

}
//玩家
struct Player{
    x:i32,
    y:f32,
    gravity:f32,
    frame:usize
}
//障碍
impl Player{
    //初始化方法
    fn new(x:i32,y:i32) -> Self {
        Player{
            x,
            y:y as f32,
            gravity:0.0,//重力
            frame:0
        }
    }
    fn gravity_and_move(&mut self){
        //定义平衡状态为2.0,当小于临界值是，重力自增，Player的y左边随重力自增而自增加，Player下沉
        if self.gravity < 2.0{
            self.gravity += 0.2;
        }
        self.y += self.gravity;

        //player到顶端后，让其坐标归0
        if self.y < 0.0{
            self.y = 0.0
        }
        self.x += 1;
        self.frame += 1;
        self.frame = self.frame % 6
    }
    fn fly(&mut self){
        self.gravity = -1.0;
    }

    fn render(&mut self,ctx:&mut BTerm){
        ctx.set_active_console(1);
        ctx.cls();
        ctx.set_fancy(
            PointF::new(0.0, self.y),
            1,
            Degrees::new(0.0),
            PointF::new(2.0, 2.0),
            WHITE,
            NAVY,
            DRAGON_FRAMES[self.frame]
        );
        ctx.set_active_console(0);
    }
}
impl State {
    //为状态创建一个初始化状态的关联函数;初始化游戏模式为菜单模式、
    fn new() -> State {
        State {
            mode: GameMode::Menu,
            frame_time:0.0,
            player: Player::new(5,12),
        }
    }
    //菜单方法
    fn home_menu(&mut self,ctx:&mut BTerm){
        ctx.cls();
        ctx.print_centered(6,"♫ ♪ Hello Bracket World ☺");
        common_func( self,ctx);
    }
    //结束的方法
    fn dead(&mut self,ctx:&mut BTerm){
        ctx.cls();
        ctx.print_centered(6,"You are dead~");
        common_func( self,ctx);
    }
    //重新开始方法
    fn restart(&mut self){
        self.player = Player::new(5, SCREEN_HEIGHT/2);
        self.frame_time = 0.0;
        self.mode = GameMode::Playing;
    }
    //正在玩的方法
    fn play(&mut self,ctx:&mut BTerm){
        ctx.cls_bg(NAVY);
        //帧时间累加等于帧毫秒时间
        self.frame_time += ctx.frame_time_ms;
        if self.frame_time > FRAME_DURATION{
            self.frame_time = 0.0;
            self.player.gravity_and_move()
        }
        if let Some(VirtualKeyCode::Space) = ctx.key{
            self.player.fly()
        }
        self.player.render(ctx);
        ctx.print(0,0,"Press Space to fly!");
        if self.player.y as i32 > SCREEN_HEIGHT {
            self.mode = GameMode::End
        }
    }
}
//为状态实现trait
impl GameState for State {
    //tick函数在GameLoop中游戏每帧率调用，将其和游戏状态实现关联
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.home_menu(ctx),
            GameMode::Playing => self.play(ctx),
            GameMode::End => self.dead(ctx)
        }
    }
}
pub fn common_func(c_self:&mut State,ctx:&mut BTerm){
    ctx.print_centered(9,"(P).Play the Game");
    ctx.print_centered(12,"(Q).Quit the Game");

    if let Some(key) = ctx.key{
        match key {
            VirtualKeyCode::P => c_self.restart(),
            VirtualKeyCode::Q => ctx.quitting = true,
            _ => {}
        }
    }
}
fn main() -> BError {
    //构建游戏实例
    let context = BTermBuilder::new()
        .with_font("../resources/flappy32.png", 32, 32)
        .with_simple_console(SCREEN_WIDTH, SCREEN_HEIGHT, "../resources/flappy32.png")
        .with_fancy_console(SCREEN_WIDTH, SCREEN_HEIGHT, "../resources/flappy32.png")
        .with_title("Hello Minimal Bracket World")
        .with_tile_dimensions(16,16)
        .build()?;

    main_loop(context, State::new())
}
