pub mod backend
{
    use egui::*;
    use macroquad::prelude::*;

    use crate::{mcvalues::mcvalues::{*, mobs::moblist}, themes::*, file::{file::*, self}};

    pub struct OutputData
    {
        pub theme: i8,
        pub state: i8
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

    pub async fn draw_graphics(state: i8, theme: i8, macroquad_font: Font, directory: &str, cur_boss_name: &mut String, cur_proj: &mut Project) -> OutputData
    {
        set_theme(theme);

        let mut output_data = OutputData
        {
            theme,
            state
        };

        clear_background(WHITE);

        //Egui Drawing, external function calls
        egui_macroquad::ui(|egui_ctx| {
            //egui::Window::new()
            if state == MAIN_SCREEN
            {
                draw_main_screen(&egui_ctx, directory, cur_boss_name, &mut output_data, cur_proj);
            }

            if state == EDITOR
            {
                draw_editor(cur_proj, egui_ctx);
            }
        });

        // Draw things before egui
        
        if theme == 0
        {
            clear_background(Color::new(0.14, 0.18, 0.2, 1.0));
        }
        else if theme == 1 {
            clear_background(Color::new(0.65, 0.65, 0.65, 1.0));
        }
        else if theme == 2 {
            clear_background(Color::new(0.0, 0.0, 0.6, 1.0));
        }
        else if theme == 3 {
            clear_background(Color::new(0.7, 0.65, 0.1, 0.7));
        }

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

    fn draw_editor(project: &mut Project, egui_ctx: &Context)
    {
        egui::Window::new("general_data")
            .min_width(screen_width())
            .min_height(screen_height() / 2.0)
            .title_bar(false)
            .anchor(Align2::LEFT_CENTER, [0.0, screen_height() / -8.0])
            .resizable(false)
            .show(egui_ctx, |ui| {

            ui.text_edit_singleline(&mut project.data.base_type);
            if !moblist().contains(&project.data.base_type)
            {
                ui.label(RichText::new(format!("{} is not a valid mob type", project.data.base_type)).color(Color32::RED));
            }

        });
    }

}
