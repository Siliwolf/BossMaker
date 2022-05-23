pub mod editor
{
    use egui::*;

    use crate::{file::file::*, mcvalues::mcvalues::{*, colors::Colors}, graphics::backend::*};
    use macroquad::prelude::*;

    pub struct WindowsOpen
    {
        pub general_data: bool,
        pub equipment: bool,
        pub abilities: bool,
        pub drops: bool
    }

    impl WindowsOpen {
        pub fn new() -> WindowsOpen
        {
            WindowsOpen { general_data: false, equipment: false, abilities: false, drops: false }
        }
    }

    pub fn save_project(project: &Project)
    {
        let project_data = &project.data;

        save_data(project_data.clone(), &project.path)
    }

    pub fn draw_editor(project: &mut Project, egui_ctx: &Context, output_data: &mut OutputData, cur_effect: &mut Effect, cur_enchant: &mut Enchantment, cur_custom_name_color: &mut Colors, windows_open: &mut WindowsOpen, cur_ability: &mut BossAbility, cur_drop: &mut BossDrop)
    {
        let mut general_data_open = windows_open.general_data;
        let mut equipment_open = windows_open.equipment;
        let mut abilities_open = windows_open.abilities;
        let mut drops_open = windows_open.drops;

        egui::Window::new("Select Windows to Open")
            .min_width(screen_width())
            .min_height(screen_height() / 2.0)
            .title_bar(true)
            .collapsible(true)
            .resizable(false)
            .anchor(Align2::LEFT_TOP, [0.0, 0.0])
            .vscroll(true)
            .show(egui_ctx, |ui| {
                if ui.button("Back to main menu").clicked()
                {
                    output_data.state = MAIN_SCREEN;
                }
                ui.label("");
                ui.checkbox(&mut general_data_open, "General Data");
                ui.checkbox(&mut equipment_open, "Equipment");
                ui.checkbox(&mut abilities_open, "Abilities");
                ui.checkbox(&mut drops_open, "Drops");
            });
        *windows_open = WindowsOpen {general_data: general_data_open, equipment: equipment_open, abilities: abilities_open, drops: drops_open};

        if general_data_open
        {
            egui::Window::new("General Data")
                .min_width(screen_width())
                .min_height(screen_height() / 2.0)
                .title_bar(true)
                .collapsible(true)
                .resizable(true)
                .vscroll(true)
                .show(egui_ctx, |ui| {

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

        if equipment_open
        {
            egui::Window::new("Equipment")
                .min_width(screen_width())
                .min_height(screen_height() / 2.0)
                .title_bar(true)
                .collapsible(true)
                .resizable(true)
                .vscroll(true)
                .show(egui_ctx, |ui| {

                    if project.data != load_data(project.path.as_str())
                    {
                        save_project(project);
                    }

                    //Enchantment
                    ui.label("Specify Current Enchantment:");
                    ui.text_edit_singleline(&mut cur_enchant.id);
                    if !Enchantment::ench_list().contains(&cur_enchant.id)
                    {
                        ui.label(RichText::new(format!("{} is not a valid enchantement id", cur_enchant.id)).color(Color32::RED));
                    }
                    
                    ui.label("Enchantment Level:");
                    ui.add(egui::DragValue::new(&mut cur_enchant.lvl).clamp_range(std::ops::RangeInclusive::new(1.0 as f64, 255.0 as f64)));

                    ui.label("");

                    //Mainhand
                    ui.label("Mainhand:");

                    ui.label("Item type (all lowercase with underscores):");
                    ui.text_edit_singleline(&mut project.data.mainhand.item_type);
                    
                    if ui.button("Add current enchantment to Mainhand").clicked()
                    {
                        project.data.mainhand.enchantments.push(cur_enchant.clone());
                    }

                    ui.menu_button("Mainhand Enchantments list, click on one to remove it", |ui|
                    {
                        for enchant in &project.clone().data.mainhand.enchantments
                        {
                            if ui.button(format!("{}, {}", enchant.id, enchant.lvl)).clicked()
                            {
                                let proj2 = project.clone();
                                
                                let index = proj2.data.mainhand.enchantments.iter().position(|r| r == &enchant.clone()).unwrap();

                                project.data.mainhand.enchantments.remove(index);
                            }
                        }
                    });

                    ui.label("");

                    //Offhand
                    ui.label("Offhand:");

                    ui.label("Item type (all lowercase with underscores):");
                    ui.text_edit_singleline(&mut project.data.offhand.item_type);
                    
                    if ui.button("Add current enchantment to Offhand").clicked()
                    {
                        project.data.offhand.enchantments.push(cur_enchant.clone());
                    }

                    ui.menu_button("Offhand Enchantments list, click on one to remove it", |ui|
                    {
                        for enchant in &project.clone().data.offhand.enchantments
                        {
                            if ui.button(format!("{}, {}", enchant.id, enchant.lvl)).clicked()
                            {
                                let proj2 = project.clone();
                                
                                let index = proj2.data.offhand.enchantments.iter().position(|r| r == &enchant.clone()).unwrap();

                                project.data.offhand.enchantments.remove(index);
                            }
                        }
                    });

                    ui.label("");

                    //Head
                    ui.label("Head:");

                    ui.label("Item type (all lowercase with underscores):");
                    ui.text_edit_singleline(&mut project.data.helmet.item_type);
                    
                    if ui.button("Add current enchantment to Head").clicked()
                    {
                        project.data.helmet.enchantments.push(cur_enchant.clone());
                    }

                    ui.menu_button("Head Enchantments list, click on one to remove it", |ui|
                    {
                        for enchant in &project.clone().data.helmet.enchantments
                        {
                            if ui.button(format!("{}, {}", enchant.id, enchant.lvl)).clicked()
                            {
                                let proj2 = project.clone();
                                
                                let index = proj2.data.helmet.enchantments.iter().position(|r| r == &enchant.clone()).unwrap();

                                project.data.helmet.enchantments.remove(index);
                            }
                        }
                    });

                    ui.label("");

                    //Chestplate
                    ui.label("Chestplate:");

                    ui.label("Item type (all lowercase with underscores):");
                    ui.text_edit_singleline(&mut project.data.chestplate.item_type);
                    
                    if ui.button("Add current enchantment to Chestplate").clicked()
                    {
                        project.data.chestplate.enchantments.push(cur_enchant.clone());
                    }

                    ui.menu_button("Chestplate Enchantments list, click on one to remove it", |ui|
                    {
                        for enchant in &project.clone().data.chestplate.enchantments
                        {
                            if ui.button(format!("{}, {}", enchant.id, enchant.lvl)).clicked()
                            {
                                let proj2 = project.clone();
                                
                                let index = proj2.data.chestplate.enchantments.iter().position(|r| r == &enchant.clone()).unwrap();

                                project.data.chestplate.enchantments.remove(index);
                            }
                        }
                    });

                    ui.label("");

                    //Leggings
                    ui.label("Leggings:");

                    ui.label("Item type (all lowercase with underscores):");
                    ui.text_edit_singleline(&mut project.data.leggings.item_type);
                    
                    if ui.button("Add current enchantment to Leggings").clicked()
                    {
                        project.data.leggings.enchantments.push(cur_enchant.clone());
                    }

                    ui.menu_button("Leggings Enchantments list, click on one to remove it", |ui|
                    {
                        for enchant in &project.clone().data.leggings.enchantments
                        {
                            if ui.button(format!("{}, {}", enchant.id, enchant.lvl)).clicked()
                            {
                                let proj2 = project.clone();
                                
                                let index = proj2.data.leggings.enchantments.iter().position(|r| r == &enchant.clone()).unwrap();

                                project.data.leggings.enchantments.remove(index);
                            }
                        }
                    });

                    ui.label("");

                    //Boots
                    ui.label("Boots:");

                    ui.label("Item type (all lowercase with underscores):");
                    ui.text_edit_singleline(&mut project.data.boots.item_type);
                    
                    if ui.button("Add current enchantment to Boots").clicked()
                    {
                        project.data.boots.enchantments.push(cur_enchant.clone());
                    }

                    ui.menu_button("Boots Enchantments list, click on one to remove it", |ui|
                    {
                        for enchant in &project.clone().data.boots.enchantments
                        {
                            if ui.button(format!("{}, {}", enchant.id, enchant.lvl)).clicked()
                            {
                                let proj2 = project.clone();
                                
                                let index = proj2.data.boots.enchantments.iter().position(|r| r == &enchant.clone()).unwrap();

                                project.data.boots.enchantments.remove(index);
                            }
                        }
                    });
            });
        }

        if abilities_open
        {
            egui::Window::new("Abilities")
                .min_width(screen_width())
                .min_height(screen_height() / 2.0)
                .title_bar(true)
                .collapsible(true)
                .resizable(true)
                .vscroll(true)
                .show(egui_ctx, |ui| {
                    if project.data != load_data(project.path.as_str())
                    {
                        save_project(project);
                    }

                    egui::ComboBox::from_label("Ability Type")
                    .selected_text(format!("{:?}", cur_ability.ability_type))
                    .show_ui(ui, |ui|
                    {
                        ui.selectable_value(&mut cur_ability.ability_type, BossAbilityType::Lighting, "Lightning");
                        ui.selectable_value(&mut cur_ability.ability_type, BossAbilityType::Summon, "Summon");
                    });

                    ui.label("Recurring delay in ticks (1 second = 20 ticks)");
                    ui.add(egui::DragValue::new(&mut cur_ability.delay));

                    egui::ComboBox::from_label("Ability Location")
                    .selected_text(format!("{:?}", cur_ability.location))
                    .show_ui(ui, |ui|
                    {
                        ui.selectable_value(&mut cur_ability.location, BossAbilityLocationType::AtSelf, "At Self");
                        ui.selectable_value(&mut cur_ability.location, BossAbilityLocationType::NearestPlayer, "Nearest Player");
                    });

                    ui.label("Config Value 1:");
                    ui.text_edit_singleline(&mut cur_ability.config[0]);
                    ui.label("Config Value 2:");
                    ui.text_edit_singleline(&mut cur_ability.config[1]);
                    ui.label("Config Value 3:");
                    ui.text_edit_singleline(&mut cur_ability.config[2]);
                    ui.label("Config Value 4:");
                    ui.text_edit_singleline(&mut cur_ability.config[3]);
                    ui.label("Config Value 5:");
                    ui.text_edit_singleline(&mut cur_ability.config[4]);
                    ui.label("Config Value 6:");
                    ui.text_edit_singleline(&mut cur_ability.config[5]);
                    ui.label("Config Value 7:");
                    ui.text_edit_singleline(&mut cur_ability.config[6]);
                    ui.label("Config Value 8:");
                    ui.text_edit_singleline(&mut cur_ability.config[7]);
                    
                    if ui.button("Add Current Ability").clicked()
                    {
                        project.data.abilities.push(cur_ability.clone());

                        *cur_ability = BossAbility {ability_type: BossAbilityType::Summon, 
                                                    delay: 0, 
                                                    location: BossAbilityLocationType::AtSelf, 
                                                    config: [String::new(), String::new(), 
                                                    String::new(), String::new(), 
                                                    String::new(), String::new(), 
                                                    String::new(), String::new()]};  
                    }

                    ui.menu_button("Abilities list, click on one to remove it", |ui|
                        {
                            for ability in &project.clone().data.abilities
                            {
                                if ui.button(format!("{:?}, {:?}", ability.ability_type, ability.config)).clicked()
                                {
                                    let proj2 = project.clone();
                                    
                                    let index = proj2.data.abilities.iter().position(|r| r == &ability.clone()).unwrap();

                                    project.data.abilities.remove(index);
                                }
                            }
                        });

                });
        }

        if drops_open
        {
            egui::Window::new("Drops")
                .min_width(screen_width())
                .min_height(screen_height() / 2.0)
                .title_bar(true)
                .collapsible(true)
                .resizable(true)
                .vscroll(true)
                .show(egui_ctx, |ui| {
                    if project.data != load_data(project.path.as_str())
                    {
                        save_project(project);
                    }

                    egui::ComboBox::from_label("Drop Type")
                    .selected_text(format!("{:?}", project.data.drop_type))
                    .show_ui(ui, |ui|
                    {
                        ui.selectable_value(&mut project.data.drop_type, BossDropType::OnlyOne, "Only One");
                        ui.selectable_value(&mut project.data.drop_type, BossDropType::IndividualProbabilities, "Individual Probabilities");
                    });

                    ui.label("Item Name (no need for this to be vanilla, \nthis is just a placeholder for me to\nhook up to custom items");

                    ui.text_edit_singleline(&mut cur_drop.item);

                    ui.label("Drop Chance:");
                    ui.add(egui::Slider::new(&mut cur_drop.chance, std::ops::RangeInclusive::new(0.0, 1.0)));

                    if ui.button("Add Current Drop to Boss").clicked()
                    {
                        project.data.drops.push(cur_drop.clone());

                        *cur_drop = BossDrop{item: "".to_string(), chance: 0.5};
                    }

                    ui.menu_button("Drop items list, click on one to remove it", |ui|
                    {
                        for drop in &project.clone().data.drops
                        {
                            if ui.button(format!("{:?}, {:?}", drop.item, drop.chance)).clicked()
                            {
                                let proj2 = project.clone();
                                
                                let index = proj2.data.drops.iter().position(|r| r == &drop.clone()).unwrap();

                                project.data.drops.remove(index);
                            }
                        }
                    });
                });
        }
    }
}