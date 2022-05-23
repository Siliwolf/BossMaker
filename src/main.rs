use boss_maker::editor::editor::WindowsOpen;
use boss_maker::graphics::backend::*;
use boss_maker::file::file::*;
use boss_maker::mcvalues::mcvalues::{Effect, Enchantment, BossAbility, BossAbilityType, BossAbilityLocationType, BossDrop};
use boss_maker::mcvalues::mcvalues::colors::Colors;
use boss_maker::themes::themes::*;use macroquad::prelude::*;
use rand::*;
extern crate rand;

#[macroquad::main("Craftz Boss Maker")]
async fn main()
{

    let mut state: i8 = MAIN_SCREEN;
    let mut theme: i8 = 0;

    let mut dir = String::new();

    if check_dir()
    {
        dir = get_dir().unwrap();
    }
    else
    {
        println!("{}\n{}", get_dir().err().unwrap(), std::env::current_dir().unwrap().to_str().unwrap().to_string());
    }

    //Set style
    egui_macroquad::cfg(|egui_context| {
        dark_theme(egui_context);
    });

    let roboto = macroquad::prelude::load_ttf_font_from_bytes(include_bytes!("resources\\roboto.ttf")).unwrap();

    //Draw loop variables
    let mut cur_boss_name = String::new();
    let mut cur_proj: Project = Project
    {
        path: "speed".to_string(),
        data: Boss::new()
    };
    let mut cur_effect: Effect = Effect
    {
        id: String::new(),
        lvl: 1
    };
    let mut cur_custom_name_color = Colors::White;
    let mut dot_loc: BgDots = BgDots { blue: rand_pos(), red: rand_pos(), green: rand_pos(), yellow: rand_pos(), yellow_right: false, yellow_up: false, blue_up: true, blue_right: true, green_right: false, green_up: true, red_right: false, red_up: false};
    let mut cur_enchant: Enchantment = Enchantment { id: String::new(), lvl: 1 };
    let mut windows_open = WindowsOpen::new();
    let mut cur_ability = BossAbility {ability_type: BossAbilityType::Summon, delay: 0, location: BossAbilityLocationType::AtSelf, config: [String::new(), String::new(), String::new(), String::new(), String::new(), String::new(), String::new(), String::new()]};
    let mut cur_boss_drop: BossDrop = BossDrop { item: "".to_string(), chance: 0.5 };

    //Main Loop    
    loop {
        let d = draw_graphics(state, theme, roboto, dir.as_str(), &mut cur_boss_name, &mut cur_proj, &mut cur_effect, &mut cur_custom_name_color, &mut dot_loc, &mut cur_enchant, &mut windows_open, &mut cur_ability, &mut cur_boss_drop).await;
        theme = d.theme;
        state = d.state;
        // thread::sleep(std::time::Duration::from_millis(50));
    }
}

fn rand_pos() -> [f32; 2]
{
    [rand::thread_rng().gen_range(0.0..screen_width()), rand::thread_rng().gen_range(0.0..screen_height())]
}
/*
let d = draw_graphics(state, theme).await;
        theme = d.theme;
*/