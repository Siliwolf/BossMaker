pub mod backend
{
    use egui::*;
    use macroquad::prelude::*;
    use crate::{mcvalues::mcvalues::{*, colors::Colors}, themes::*, file::{file::*, self}, editor::{self, editor::WindowsOpen}};

    pub struct OutputData
    {
        pub theme: i8,
        pub state: i8
    }

    pub struct BgDots
    {
        pub blue: [f32; 2],
        pub red: [f32; 2],
        pub green: [f32; 2],
        pub yellow: [f32; 2],
        pub blue_up: bool,
        pub red_up: bool,
        pub green_up: bool,
        pub yellow_up: bool,
        pub blue_right: bool,
        pub red_right: bool,
        pub green_right: bool,
        pub yellow_right: bool
    }

    pub const MAIN_SCREEN: i8 = 0;
    pub const EDITOR: i8 = 1;

    #[derive(Debug, PartialEq)]
    enum Themes
    {
        Dark,
        Light,
        Blue,
        Sandy
    }

    /*
        THEMES:
        0: Dark
        1: Light
        2: Blue
        3: Sandy
    */

    pub async fn draw_graphics(state: i8, theme: i8, macroquad_font: Font, directory: &str, cur_boss_name: &mut String, cur_proj: &mut Project, cur_effect: &mut Effect, cur_custom_name_color: &mut Colors, dot_loc: &mut BgDots, cur_enchantment: &mut Enchantment, windows_open: &mut WindowsOpen, cur_ability: &mut BossAbility, cur_boss_drop: &mut BossDrop) -> OutputData
    {
        set_theme(theme);

        let mut output_data = OutputData
        {
            theme,
            state
        };

        clear_background(Color { r: 0.1, g: 0.1, b: 0.2, a: 1.0 });

        //Egui Drawing, external function calls
        egui_macroquad::ui(|egui_ctx| {
            //egui::Window::new()
            if state == MAIN_SCREEN
            {
                draw_main_screen(&egui_ctx, directory, cur_boss_name, &mut output_data, cur_proj);
            }

            if state == EDITOR
            {
                editor::editor::draw_editor(cur_proj, egui_ctx, &mut output_data, cur_effect, cur_enchantment, cur_custom_name_color, windows_open, cur_ability, cur_boss_drop);
            }
        });

        // Draw things before egui

        //Background
        
        draw_bg(dot_loc);
        
        if state == MAIN_SCREEN
        {
            let mut text_size = ((screen_width() * 0.01) * (screen_height() * 0.01)) as u16;

            if text_size > 96
            {
                text_size = 96;
            }
    
            let text_params: TextParams = TextParams { font: macroquad_font, font_size: text_size, font_scale: 1.0, font_scale_aspect: 1.0, color: Color::new(0.45, 0.90, 0.60, 1.0) };
            draw_text_ex("Craftz Boss Maker", 10.0, screen_height() / 10.0, text_params);
        }

        egui_macroquad::draw();
         
        // Draw things after egui

        let text_params: TextParams = TextParams { font: macroquad_font, font_size: 16, font_scale: 1.0, font_scale_aspect: 1.0, color: WHITE };
        draw_text_ex(format!("Fps: {}", get_fps()).as_str(), 0.0, screen_height() * 0.98, text_params);
        
        next_frame().await;
    
        output_data
    }


    #[inline]
    fn draw_main_screen(egui_ctx: &Context, dir: &str, cur_boss_name: &mut String, output_data: &mut OutputData, cur_proj: &mut Project)
    {
        let mut theme = output_data.theme;

        egui::Window::new("projects")
            .title_bar(false)
            .resizable(false)
            .min_height(1000.0)
            .anchor(Align2::RIGHT_CENTER, [0.0, 0.0])
            .show(egui_ctx, |ui| {
                ui.label("Recent Projects:");

                //Project stuff
                let files = std::fs::read_dir(format!("{}\\data", dir)).unwrap();

                for file in files
                {
                    if ui.button(file.as_ref().unwrap().file_name().to_str().unwrap().to_string()).clicked()
                    {
                        cur_proj.path = file.as_ref().unwrap().path().to_str().unwrap().to_string();
                        cur_proj.data = load_data(file.as_ref().unwrap().path().to_str().unwrap());

                        output_data.state = EDITOR;

                        println!("Loaded project with data {:?}", load_data(file.as_ref().unwrap().path().to_str().unwrap()));
                    }
                }

                ui.label(HORIZONTAL_SPACE);
            });
               
        egui::Window::new("context_bar")
            .min_width(screen_width())
            .min_height(screen_height() / 2.0)
            .title_bar(false)
            .anchor(Align2::LEFT_CENTER, [0.0, screen_height() / -8.0])
            .resizable(false)
            .show(egui_ctx, |ui| {
                

                ui.label("Context Actions:");
                ui.label("                                                     ");

                //Theme stuff
                ui.label("Select Theme:");

                let mut current_theme: Themes = Themes::Dark;

                if theme == 0
                {
                    current_theme = Themes::Dark;
                }
                else if theme == 1
                {
                    current_theme = Themes::Light;
                }
                else if theme == 2
                {
                    current_theme = Themes::Blue;
                }
                else if theme == 3
                {
                    current_theme = Themes::Sandy;
                }

                egui::ComboBox::from_label("")
                .selected_text(format!("{:?}", current_theme))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut current_theme, Themes::Dark, "Dark");
                    ui.selectable_value(&mut current_theme, Themes::Light, "Light");
                    ui.selectable_value(&mut current_theme, Themes::Blue, "Blue");
                    ui.selectable_value(&mut current_theme, Themes::Sandy, "Sandy");


                if current_theme == Themes::Dark
                {
                    theme = 0;
                }
                else if current_theme == Themes::Light
                {
                    theme = 1;
                }
                else if current_theme == Themes::Blue
                {
                    theme = 2;
                }
                else if current_theme == Themes::Sandy
                {
                    theme = 3;
                }
                else
                {
                    theme = 0;
                }


                });

                   
                //New boss stuff
                ui.label("New boss name:");
                ui.text_edit_singleline(cur_boss_name);

                if ui.button("Create New Boss").clicked()
                {
                    //New project
                    if cur_boss_name != ""
                    {
                        let path = format!("{}\\data\\{}.json", dir, cur_boss_name, );
                        println!("Creating new project: {}", path);

                        file::file::save_data(Boss::new(), &path);
                    }
                }

                // add_space(ui);


            });

            output_data.theme = theme;
    }
    
    fn draw_bg(dot_loc: &mut BgDots)
    {
        use ::rand::*;

        let speed = 130.0;

        let rand_fac = thread_rng().gen_range(0.8..1.2);

        //Blue dot
        if dot_loc.blue_up
        {
            dot_loc.blue[1] += -speed * get_frame_time() * rand_fac;
        }
        else
        {
            dot_loc.blue[1] += speed * get_frame_time() * rand_fac;
        }

        if dot_loc.blue_right
        {
            dot_loc.blue[0] += speed * get_frame_time() * rand_fac;
        }
        else
        {
            dot_loc.blue[0] += -speed * get_frame_time() * rand_fac;
        }

        if dot_loc.blue[0] >= screen_width()
        {
            dot_loc.blue_right = false;
        }
        else if dot_loc.blue[0] <= 0.0
        {
            dot_loc.blue_right = true;
        }

        if dot_loc.blue[1] >= screen_height()
        {
            dot_loc.blue_up = true;
        }
        else if dot_loc.blue[1] <= 0.0
        {
            dot_loc.blue_up = false;
        }

        

        //Red dot
        if dot_loc.red_up
        {
            dot_loc.red[1] += -speed * get_frame_time() * rand_fac;
        }
        else
        {
            dot_loc.red[1] += speed * get_frame_time() * rand_fac;
        }

        if dot_loc.red_right
        {
            dot_loc.red[0] += speed * get_frame_time() * rand_fac;
        }
        else
        {
            dot_loc.red[0] += -speed * get_frame_time() * rand_fac;
        }

        if dot_loc.red[0] >= screen_width()
        {
            dot_loc.red_right = false;
        }
        else if dot_loc.red[0] <= 0.0
        {
            dot_loc.red_right = true;
        }

        if dot_loc.red[1] >= screen_height()
        {
            dot_loc.red_up = true;
        }
        else if dot_loc.red[1] <= 0.0
        {
            dot_loc.red_up = false;
        }

        

        //Green dot
        if dot_loc.green_up
        {
            dot_loc.green[1] += -speed * get_frame_time() * rand_fac;
        }
        else
        {
            dot_loc.green[1] += speed * get_frame_time() * rand_fac;
        }

        if dot_loc.green_right
        {
            dot_loc.green[0] += speed * get_frame_time() * rand_fac;
        }
        else
        {
            dot_loc.green[0] += -speed * get_frame_time() * rand_fac;
        }

        if dot_loc.green[0] >= screen_width()
        {
            dot_loc.green_right = false;
        }
        else if dot_loc.green[0] <= 0.0
        {
            dot_loc.green_right = true;
        }

        if dot_loc.green[1] >= screen_height()
        {
            dot_loc.green_up = true;
        }
        else if dot_loc.green[1] <= 0.0
        {
            dot_loc.green_up = false;
        }

        //Yellow dot
        if dot_loc.yellow_up
        {
            dot_loc.yellow[1] += -speed * get_frame_time() * rand_fac;
        }
        else
        {
            dot_loc.yellow[1] += speed * get_frame_time() * rand_fac;
        }

        if dot_loc.yellow_right
        {
            dot_loc.yellow[0] += speed * get_frame_time() * rand_fac;
        }
        else
        {
            dot_loc.yellow[0] += -speed * get_frame_time() * rand_fac;
        }

        if dot_loc.yellow[0] >= screen_width()
        {
            dot_loc.yellow_right = false;
        }
        else if dot_loc.yellow[0] <= 0.0
        {
            dot_loc.yellow_right = true;
        }

        if dot_loc.yellow[1] >= screen_height()
        {
            dot_loc.yellow_up = true;
        }
        else if dot_loc.yellow[1] <= 0.0
        {
            dot_loc.yellow_up = false;
        }

        draw_line(dot_loc.blue[0], dot_loc.blue[1], dot_loc.green[0], dot_loc.green[1], 10.0, WHITE);
        draw_line(dot_loc.red[0], dot_loc.red[1], dot_loc.green[0], dot_loc.green[1], 10.0, WHITE);
        draw_line(dot_loc.yellow[0], dot_loc.yellow[1], dot_loc.green[0], dot_loc.green[1], 10.0, WHITE);
        draw_line(dot_loc.blue[0], dot_loc.blue[1], dot_loc.red[0], dot_loc.red[1], 10.0, WHITE);
        draw_line(dot_loc.blue[0], dot_loc.blue[1], dot_loc.yellow[0], dot_loc.yellow[1], 10.0, WHITE);
        draw_line(dot_loc.red[0], dot_loc.red[1], dot_loc.yellow[0], dot_loc.yellow[1], 10.0, WHITE);

        draw_line(dot_loc.red[0], dot_loc.red[1], 0.0, 0.0, 10.0, WHITE);
        draw_line(dot_loc.green[0], dot_loc.green[1], screen_width(), 0.0, 10.0, WHITE);
        draw_line(dot_loc.blue[0], dot_loc.blue[1], screen_width(), screen_height(), 10.0, WHITE);
        draw_line(dot_loc.yellow[0], dot_loc.yellow[1], 0.0, screen_height(), 10.0, WHITE);

        draw_circle(dot_loc.green[0], dot_loc.green[1], screen_height() / 32.0, GREEN);
        draw_circle(dot_loc.green[0], dot_loc.green[1], screen_height() / 64.0, DARKGREEN);

        draw_circle(dot_loc.red[0], dot_loc.red[1], screen_height() / 32.0, RED);
        draw_circle(dot_loc.red[0], dot_loc.red[1], screen_height() / 64.0, DARKBROWN);

        draw_circle(dot_loc.blue[0], dot_loc.blue[1], screen_height() / 32.0, BLUE);
        draw_circle(dot_loc.blue[0], dot_loc.blue[1], screen_height() / 64.0, DARKBLUE);

        draw_circle(dot_loc.yellow[0], dot_loc.yellow[1], screen_height() / 32.0, YELLOW);
        draw_circle(dot_loc.yellow[0], dot_loc.yellow[1], screen_height() / 64.0, ORANGE);       
    }

}