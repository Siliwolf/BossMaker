use boss_maker::graphics::backend::*;
use boss_maker::file::file::*;
use boss_maker::themes::themes::*;

#[macroquad::main("Craftz Boss Maker")]
async fn main()
{

    let mut state: i8 = MAIN_SCREEN;
    let mut theme: i8 = 0;

    //Debug, remove before shipping
    let _dirresult = std::env::set_current_dir("./testfolder");

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
        path: String::new(),
        data: Boss::new()
    };

    //Main Loop    
    loop {
        let d = draw_graphics(state, theme, roboto, dir.as_str(), &mut cur_boss_name, &mut cur_proj).await;
        theme = d.theme;
        state = d.state;
        // thread::sleep(std::time::Duration::from_millis(50));
    }
}

/*
let d = draw_graphics(state, theme).await;
        theme = d.theme;
*/