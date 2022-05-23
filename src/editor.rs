pub mod editor
{
    use egui::*;

    use crate::{file::file::*, mcvalues::mcvalues::{*, colors::Colors}, graphics::backend::*};
    use macroquad::prelude::*;

    pub fn save_project(project: &Project)
    {
        let project_data = &project.data;

        save_data(project_data.clone(), &project.path)
    }

    pub fn draw_editor(project: &mut Project, egui_ctx: &Context, output_data: &mut OutputData, cur_effect: &mut Effect, cur_custom_name_color: &mut Colors)
    {
        egui::Window::new("General Data")
            .min_width(screen_width())
            .min_height(screen_height() / 2.0)
            .title_bar(true)
            .collapsible(false)
            .anchor(Align2::LEFT_TOP, [0.0, 0.0])
            .resizable(false)
            .vscroll(true)
            .show(egui_ctx, |ui| {

            if ui.button("Back to main menu").clicked()
            {
                output_data.state = MAIN_SCREEN;
            }
            ui.label("");

            if project.data != load_data(project.path.as_str())
            {
                save_project(project);
            }

            ui.label("Boss Type:");
            ui.text_edit_singleline(&mut project.data.base_type);
            if !mobs::moblist().contains(&project.data.base_type)
            {
                ui.label(RichText::new(format!("{} is not a valid mob type", project.data.base_type)).color(Color32::RED));
            }
            ui.label("");

            ui.label("Boss Health:");
            ui.add(egui::DragValue::new(&mut project.data.health).clamp_range(std::ops::RangeInclusive::new(0.0 as f64, 10000.0 as f64)));
            ui.label("");


            ui.label("Boss Speed (1.0 = 1 level of speed):");
            ui.add(egui::DragValue::new(&mut project.data.speed).speed(0.01).clamp_range(std::ops::RangeInclusive::new(0.0 as f64, 10.0 as f64)));
            ui.label("");

            ui.checkbox(&mut project.data.aggro_through_walls, "Aggro Through Walls");
            ui.label("");

            ui.label("Potion effects:");
            ui.label("Name of effect:");
            ui.text_edit_singleline(&mut cur_effect.id);
            if !Effect::effect_name_list().contains(&cur_effect.id)
            {
                ui.label(RichText::new(format!("{} is not a valid effect id", cur_effect.id)).color(Color32::RED));
            }

            ui.label("Effect Level:");
            ui.add(egui::DragValue::new(&mut cur_effect.lvl).speed(0.1).clamp_range(std::ops::RangeInclusive::new(1.0 as f64, 255.0 as f64)));
            if ui.button("Add Effect").clicked()
            {
                project.data.effects.push(cur_effect.clone());
            }

            ui.menu_button("Effects list, click on one to remove it", |ui|
            {
                for effect in &project.clone().data.effects
                {
                    if ui.button(format!("{}, {}", effect.id, effect.lvl)).clicked()
                    {
                        let proj2 = project.clone();
                        
                        let index = proj2.data.effects.iter().position(|r| r == &effect.clone()).unwrap();

                        project.data.effects.remove(index);
                    }
                }
            });

            ui.label("");

            let mut selected_color = *cur_custom_name_color;
            ui.label("Custom Name Color:");
            egui::ComboBox::from_label("Custom Name Color")
                .selected_text(format!("{:?}", selected_color))
                .show_ui(ui, |ui|
                {
                    ui.selectable_value(&mut selected_color, Colors::White, "White");
                    ui.selectable_value(&mut selected_color, Colors::LightGray, "Light Gray");
                    ui.selectable_value(&mut selected_color, Colors::Gray, "Gray");
                    ui.selectable_value(&mut selected_color, Colors::Black, "Black");
                    ui.selectable_value(&mut selected_color, Colors::Brown, "Brown");
                    ui.selectable_value(&mut selected_color, Colors::Red, "Red");
                    ui.selectable_value(&mut selected_color, Colors::Orange, "Orange");
                    ui.selectable_value(&mut selected_color, Colors::Yellow, "Yellow");
                    ui.selectable_value(&mut selected_color, Colors::Lime, "Lime");
                    ui.selectable_value(&mut selected_color, Colors::Green, "Green");
                    ui.selectable_value(&mut selected_color, Colors::Cyan, "Cyan");
                    ui.selectable_value(&mut selected_color, Colors::LightBlue, "Light Blue");
                    ui.selectable_value(&mut selected_color, Colors::Blue, "Blue");
                    ui.selectable_value(&mut selected_color, Colors::Purple, "Purple");
                    ui.selectable_value(&mut selected_color, Colors::Magenta, "Magenta");
                    ui.selectable_value(&mut selected_color, Colors::Pink, "Pink");
                });

            match selected_color {
                Colors::White => *cur_custom_name_color = Colors::White,
                Colors::LightGray => *cur_custom_name_color = Colors::LightGray,
                Colors::Gray => *cur_custom_name_color = Colors::Gray,
                Colors::Black => *cur_custom_name_color = Colors::Black,
                Colors::Brown => *cur_custom_name_color = Colors::Brown,
                Colors::Red => *cur_custom_name_color = Colors::Red,
                Colors::Orange => *cur_custom_name_color = Colors::Orange,
                Colors::Yellow => *cur_custom_name_color = Colors::Yellow,
                Colors::Lime => *cur_custom_name_color = Colors::Lime,
                Colors::Green => *cur_custom_name_color = Colors::Green,
                Colors::Cyan => *cur_custom_name_color = Colors::Cyan,
                Colors::LightBlue => *cur_custom_name_color = Colors::LightBlue,
                Colors::Blue => *cur_custom_name_color = Colors::Blue,
                Colors::Purple => *cur_custom_name_color = Colors::Purple,
                Colors::Magenta => *cur_custom_name_color = Colors::Magenta,
                Colors::Pink => *cur_custom_name_color = Colors::Pink,
            }

            ui.label("Custom Name:");
            ui.text_edit_singleline(&mut project.data.custom_name_text);

            ui.label("");

            ui.label("Particles Type:");
            ui.text_edit_singleline(&mut project.data.particles_type);
            ui.label("Particles Number:");
            ui.add(egui::DragValue::new(&mut project.data.particles_number).clamp_range(std::ops::RangeInclusive::new(1.0 as f64, 1000.0 as f64)));

            ui.label("");
        });
    }
}