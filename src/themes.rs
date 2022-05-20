pub mod themes
{
    use egui::*;

    pub fn dark_theme(ctx: &egui::Context)
    {
        let mut style: egui::Style = (*ctx.style()).clone();

        style.visuals.extreme_bg_color = egui::Color32::from_rgb(90, 90, 90);

        style.visuals.window_rounding = Rounding
        {
            ..Default::default()
        };

        //Set font
        let mut fonts = FontDefinitions::default();

        fonts.font_data.insert("roboto".to_owned(),
        FontData::from_static(include_bytes!("resources\\roboto.ttf")).tweak(FontTweak { scale: 1.4, y_offset_factor: -0.22, y_offset: 0.0 }));

        fonts.families.get_mut(&FontFamily::Proportional).unwrap()
            .insert(0, "roboto".to_owned());

        fonts.families.get_mut(&FontFamily::Monospace).unwrap()
            .push("roboto".to_owned());

        ctx.set_fonts(fonts);

        //No more set font
        style.wrap = Some(true);

        style.visuals.dark_mode = true;

        style.visuals.window_shadow = epaint::Shadow
        {
            extrusion: 0.0,
            ..Default::default()
        };
        
        style.visuals.widgets.noninteractive.bg_fill = Color32::from_rgb(35, 35, 35);

        style.visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(60, 60, 60);

        style.visuals.override_text_color = Some(egui::Color32::from_rgb(255, 255, 255));

        ctx.set_style(style);
    }

    pub fn light_theme(ctx: &egui::Context)
    {
        let mut style: egui::Style = (*ctx.style()).clone();

        style.visuals.extreme_bg_color = egui::Color32::from_rgb(200, 200, 200);

        style.visuals.window_rounding = Rounding
        {
            ..Default::default()
        };

        //Set font
        let mut fonts = FontDefinitions::default();

        fonts.font_data.insert("roboto".to_owned(),
        FontData::from_static(include_bytes!("resources\\roboto.ttf")).tweak(FontTweak { scale: 1.4, y_offset_factor: -0.22, y_offset: 0.0 }));

        fonts.families.get_mut(&FontFamily::Proportional).unwrap()
            .insert(0, "roboto".to_owned());

        fonts.families.get_mut(&FontFamily::Monospace).unwrap()
            .push("roboto".to_owned());

        ctx.set_fonts(fonts);

        //No more set font
        style.wrap = Some(true);

        style.visuals.dark_mode = false;

        style.visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(235, 235, 235);

        style.visuals.window_shadow = epaint::Shadow
        {
            extrusion: 0.0,
            ..Default::default()
        };
        
        style.visuals.widgets.noninteractive.bg_fill = Color32::from_rgb(140, 140, 140);

        style.visuals.override_text_color = Some(egui::Color32::from_rgb(0, 0, 0));

        ctx.set_style(style);
    }

    pub fn blue_theme(ctx: &egui::Context)
    {
        let mut style: egui::Style = (*ctx.style()).clone();

        style.visuals.extreme_bg_color = egui::Color32::from_rgb(90, 90, 90);

        style.visuals.window_rounding = Rounding
        {
            ..Default::default()
        };

        //Set font
        let mut fonts = FontDefinitions::default();

        fonts.font_data.insert("roboto".to_owned(),
        FontData::from_static(include_bytes!("resources\\roboto.ttf")).tweak(FontTweak { scale: 1.4, y_offset_factor: -0.22, y_offset: 0.0 }));

        fonts.families.get_mut(&FontFamily::Proportional).unwrap()
            .insert(0, "roboto".to_owned());

        fonts.families.get_mut(&FontFamily::Monospace).unwrap()
            .push("roboto".to_owned());

        ctx.set_fonts(fonts);

        //No more set font
        style.wrap = Some(true);

        style.visuals.dark_mode = true;

        style.visuals.window_shadow = epaint::Shadow
        {
            extrusion: 0.0,
            ..Default::default()
        };
        
        style.visuals.widgets.noninteractive.bg_fill = Color32::from_rgb(50, 102, 117);

        style.visuals.widgets.inactive.bg_fill = Color32::from_rgb(42, 67, 74);

        style.visuals.override_text_color = Some(egui::Color32::from_rgb(255, 255, 255));

        ctx.set_style(style);
    }

    pub fn sandy_theme(ctx: &egui::Context)
    {
        let mut style: egui::Style = (*ctx.style()).clone();

        style.visuals.extreme_bg_color = egui::Color32::from_rgb(90, 90, 90);

        style.visuals.window_rounding = Rounding
        {
            ..Default::default()
        };

        //Set font
        let mut fonts = FontDefinitions::default();

        fonts.font_data.insert("roboto".to_owned(),
        FontData::from_static(include_bytes!("resources\\roboto.ttf")).tweak(FontTweak { scale: 1.4, y_offset_factor: -0.22, y_offset: 0.0 }));

        fonts.families.get_mut(&FontFamily::Proportional).unwrap()
            .insert(0, "roboto".to_owned());

        fonts.families.get_mut(&FontFamily::Monospace).unwrap()
            .push("roboto".to_owned());

        ctx.set_fonts(fonts);

        //No more set font
        style.wrap = Some(true);

        style.visuals.dark_mode = true;

        style.visuals.window_shadow = epaint::Shadow
        {
            extrusion: 0.0,
            ..Default::default()
        };
        
        style.visuals.widgets.noninteractive.bg_fill = Color32::from_rgb(196, 191, 139);

        style.visuals.widgets.inactive.bg_fill = Color32::from_rgb(137, 132, 85);

        style.visuals.override_text_color = Some(egui::Color32::from_rgb(0, 0, 0));

        ctx.set_style(style);
    }
}

pub fn set_theme(theme: i8)
{
    match theme {
        0 => egui_macroquad::cfg(|egui_context| {
            themes::dark_theme(egui_context);
        }),

        1 => egui_macroquad::cfg(|egui_context| {
            themes::light_theme(egui_context);
        }),

        2 => egui_macroquad::cfg(|egui_context| {
            themes::blue_theme(egui_context);
        }),

        3 => egui_macroquad::cfg(|egui_context| {
            themes::sandy_theme(egui_context);
        }),

        _ => egui_macroquad::cfg(|egui_context|
        {
            themes::dark_theme(egui_context);
        })
    }
}