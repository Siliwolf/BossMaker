pub mod mcvalues
{
    use serde::{Serialize, Deserialize};

    #[derive(Serialize, Deserialize, Clone)]
    pub struct Effect
    {
        pub id: String,
        pub lvl: i8
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub struct BossAbility
    {
        pub ability_type: String,
        pub delay: i32,
        pub location: String,
        pub config: Vec<String>
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub struct BossEquipmentItem
    {
        pub item_type: String,
        pub enchantments: Vec<Enchantment>
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub struct BossDrop
    {
        pub item: String,
        pub chance: f32
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub struct Enchantment
    {
        pub id: String,
        pub lvl: i8
    }

    pub mod colors
    {
        pub const WHITE: &str = "white";
        pub const LIGHT_GRAY: &str = "light_gray";
        pub const GRAY: &str = "gray";
        pub const BLACK: &str = "black";
        pub const BROWN: &str = "brown";
        pub const RED: &str = "red";
        pub const ORANGE: &str = "orange";
        pub const YELLOW: &str = "yellow";
        pub const LIME: &str = "lime";
        pub const GREEN: &str = "green";
        pub const CYAN: &str = "cyan";
        pub const LIGHT_BLUE: &str = "light_blue";
        pub const BLUE: &str = "blue";
        pub const PURPLE: &str = "purple";
        pub const MAGENTA: &str = "magenta";
        pub const PINK: &str = "pink";
    }

    pub mod mobs
    {
        pub const AXOLOTL: &str = "axolotl";
        pub const BAT: &str = "bat";
        pub const CAT: &str = "cat";
        pub const CHICKEN: &str = "chicken";
        pub const COD: &str = "cod";
        pub const COW: &str = "cow";
        pub const DONKEY: &str = "donkey";
        pub const FOX: &str = "fox";
        pub const GLOW_SQUID: &str = "glow_squid";
        pub const HORSE: &str = "horse";
        pub const MOOSHROOM: &str = "mooshroom";
        pub const MULE: &str = "mule";
        pub const OCELOT: &str = "ocelot";
        pub const PARROT: &str = "parrot";
        pub const PIG: &str = "pig";
        pub const PUFFERFISH: &str = "pufferfish";
        pub const RABBIT: &str = "rabbit";
        pub const SALMON: &str = "salmon";
        pub const SHEEP: &str = "sheep";
        pub const SKELETON_HORSE: &str = "skeleton_horse";
        pub const SNOW_GOLEM: &str = "snow_golem";
        pub const SQUID: &str = "squid";
        pub const STRIDER: &str = "strider";
        pub const TROPICAL_FISH: &str = "tropical_fish";
        pub const TURTLE: &str = "turtle";
        pub const VILLAGER: &str = "villager";
        pub const WANDERING_TRADER: &str = "wandering_trader";
        pub const BEE: &str = "bee";
        pub const CAVE_SPIDER: &str = "cave_spider";
        pub const DOLPHIN: &str = "dolphin";
        pub const ENDERMAN: &str = "enderman";
        pub const GOAT: &str = "goat";
        pub const IRON_GOLEM: &str = "iron_golem";
        pub const LLAMA: &str = "llama";
        pub const PANDA: &str = "panda";
        pub const PIGLIN: &str = "piglin";
        pub const POLAR_BEAR: &str = "polar_bear";
        pub const SPIDER: &str = "spider";
        pub const TRADER_LLAMA: &str = "trader_llama";
        pub const WOLF: &str = "wolf";
        pub const ZOMBIFIED_PIGLIN: &str = "zombified_piglin";
        pub const BLAZE: &str = "blaze";
        pub const CHICKEN_JOCKEY: &str = "chicken_jockey";
        pub const CREEPER: &str = "creeper";
        pub const DROWNED: &str = "drowned";
        pub const ELDER_GUARDIAN: &str = "elder_guardian";
        pub const ENDERMITE: &str = "endermite";
        pub const EVOKER: &str = "evoker";
        pub const GHAST: &str = "ghast";
        pub const GUARDIAN: &str = "guardian";
        pub const HOGLIN: &str = "hoglin";
        pub const HUSK: &str = "husk";
        pub const MAGMA_CUBE: &str = "magma_cube";
        pub const PHANTOM: &str = "phantom";
        pub const PIGLIN_BRUTE: &str = "piglin_brute";
        pub const PILLAGER: &str = "pillager";
        pub const RAVAGER: &str = "ravager";
        pub const SHULKER: &str = "shulker";
        pub const SILVERFISH: &str = "silverfish";
        pub const SKELETON: &str = "skeleton";
        pub const SKELETON_HORSEMAN: &str = "skeleton_horseman";
        pub const SLIME: &str = "slime";
        pub const SPIDER_JOCKEY: &str = "spider_jockey";
        pub const STRAY: &str = "stray";
        pub const VEX: &str = "vex";
        pub const VINDICATOR: &str = "vindicator";
        pub const WITCH: &str = "witch";
        pub const WITHER_SKELETON: &str = "wither_skeleton";
        pub const ZOGLIN: &str = "zoglin";
        pub const ZOMBIE: &str = "zombie";
        pub const ZOMBIE_VILLAGER: &str = "zombie_villager";
        pub const ENDERDRAGON: &str = "enderdragon";
        pub const WITHER: &str = "wither";
        pub const GIANT: &str = "giant";
        pub const ZOMBIE_HORSE: &str = "zombie_horse";
        pub const KILLER_BUNNY: &str = "killer_bunny";
        pub const ILLUSIONER: &str = "illusioner";
    }


    pub const HORIZONTAL_SPACE: &str = "                                                                                                                                                                                                                                      ";
}