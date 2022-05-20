pub mod backend
{
    use egui::*;
    use macroquad::prelude::*;

    use crate::{mcvalues::mcvalues::*, themes::set_theme};

    pub struct OutputData
    {
        pub theme: i8,
    }

    pub const MAIN_SCREEN: i8 = 0;

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

    pub async fn draw_graphics(state: i8, theme: i8, macroquad_font: Font, directory: &str) -> OutputData
    {
        set_theme(theme);

        let mut output_data = OutputData
        {
            theme
        };

        clear_background(WHITE);

        //Egui Drawing, external function calls
        egui_macroquad::ui(|egui_ctx| {
            //egui::Window::new()
            if state == MAIN_SCREEN
            {
                draw_main_screen(&egui_ctx, &mut output_data.theme);
            }
        });

        // Draw things before egui
        
        if state == MAIN_SCREEN
        {
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
    fn draw_main_screen(egui_ctx: &Context, theme: &mut i8)
    {
        egui::Window::new("projects")
            .title_bar(false)
            .resizable(false)
            .min_height(1000.0)
            .vscroll(true)
            .fixed_pos([screen_width() * 0.6, screen_height() * 0.2])
            .show(egui_ctx, |ui| {
                ui.label("Recent Projects:");

                //Project stuff

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


                if ui.button("Create New Boss").clicked()
                {
                    //New project
                }

                //Theme stuff
                ui.label("Select Theme:");

                let mut current_theme: Themes = Themes::Dark;

                if *theme == 0
                {
                    current_theme = Themes::Dark;
                }
                else if *theme == 1
                {
                    current_theme = Themes::Light;
                }
                else if *theme == 2
                {
                    current_theme = Themes::Blue;
                }
                else if *theme == 3
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
                    *theme = 0;
                }
                else if current_theme == Themes::Light
                {
                    *theme = 1;
                }
                else if current_theme == Themes::Blue
                {
                    *theme = 2;
                }
                else if current_theme == Themes::Sandy
                {
                    *theme = 3;
                }
                else
                {
                    *theme = 0;
                }


                });

                // add_space(ui);


            });
    }

}
