pub mod file
{
    use serde::*;

    use crate::mcvalues::mcvalues::*;

    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct Boss
    {
        pub name: String,
        pub base_type: String,
        pub health: f32,
        pub speed: f32,
        pub aggro_through_walls: bool,
        pub effects: Vec<Effect>,
        pub custom_name_color: String,
        pub custom_name_text: String,
        pub particles_type: String,
        pub particles_number: i32,
        pub mainhand: BossEquipmentItem,
        pub offhand: BossEquipmentItem,
        pub helmet: BossEquipmentItem,
        pub chestplate: BossEquipmentItem,
        pub leggings: BossEquipmentItem,
        pub boots: BossEquipmentItem,
        pub abilities: Vec<BossAbility>,
        pub drop_type: String,
        pub drops: Vec<BossDrop>
    }

    pub struct Project
    {
        pub path: String,
        pub data: Boss
    }

    impl Boss
    {
        pub fn new() -> Boss
        {
            let boss = Boss
            {
                name: String::new(),
                base_type: mobs::ZOMBIE.to_string(),
                health: 20.0,
                speed: 1.0,
                aggro_through_walls: true,
                effects: [].to_vec(),
                custom_name_color: colors::WHITE.to_string(),
                custom_name_text: String::new(),
                particles_type: String::new(),
                particles_number: 0,
                mainhand: BossEquipmentItem { item_type: "air".to_string(), enchantments: [].to_vec() },
                offhand: BossEquipmentItem { item_type: "air".to_string(), enchantments: [].to_vec() },
                helmet: BossEquipmentItem { item_type: "air".to_string(), enchantments: [].to_vec() },
                chestplate: BossEquipmentItem { item_type: "air".to_string(), enchantments: [].to_vec() },
                leggings: BossEquipmentItem { item_type: "air".to_string(), enchantments: [].to_vec() },
                boots: BossEquipmentItem { item_type: "air".to_string(), enchantments: [].to_vec() },
                abilities: [].to_vec(),
                drop_type: String::new(),
                drops: [].to_vec()
            };

            boss
        }
    }

    pub fn check_dir() -> bool
    {
        for path in std::fs::read_dir("./").unwrap()
        {
            let f = path.unwrap();
    
            if f.file_name() != "data" && 
            f.file_name().to_str().unwrap() != find_last_char_onwards(std::env::current_exe().unwrap().to_str().unwrap(), '\\')
            {
                return false;
            }

        }

        return true
    }

    fn find_last_char_onwards(slice: &str, char: char) -> &str
    {
        let mut iter: usize = 0;
        let mut final_index: usize = 0;
        for c in slice.chars()
        {
            if c == char
            {
                final_index = iter;
            }
            
            iter += 1;
        }

        &slice[final_index + 1..]
    }

    pub fn get_dir() -> Result<String, &'static str>
    {
        let dir = std::env::current_dir().unwrap();

        let mut found_data_folder = false;

        for path in std::fs::read_dir("./").unwrap()
        {
            let f = path.unwrap();
    
            if f.file_name() != "data" && 
            f.file_name().to_str().unwrap() != find_last_char_onwards(std::env::current_exe().unwrap().to_str().unwrap(), '\\')
            {
                return Err("Invalid files at location");
            }

            if f.file_name() == "data"
            {
                found_data_folder = true;
            }
        }

        if !found_data_folder
        {
            //Create data folder
            if std::fs::create_dir("./data").is_ok()
            {
                println!("Could not create data folder\n\n\n");
                panic!()
            }
        }

        if std::fs::read_dir("./").unwrap().next().is_none()
        {
            if !std::fs::create_dir(dir.as_path().to_str().unwrap().to_owned() + "/data").is_err()
            {
                return Err("Failed to create files");
            }
        }
        
        Ok(dir.to_str().unwrap().to_string())
    }

    pub fn save_data(data: Boss, path: &str)
    {
        let j = serde_json::to_string(&data);
        
        if std::fs::write(path, j.unwrap()).is_ok()
        {
            
        }
    }

    pub fn load_data(path: &str) -> Boss
    {
        println!("Loading file {}", path);

        let s: String = std::fs::read_to_string(path).unwrap();
        let s = s.as_str();

        let data: Result<Boss, serde_json::Error> = serde_json::from_str(s);

        let boss: Boss = data.unwrap();

        boss
    }
}