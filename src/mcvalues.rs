pub mod mcvalues
{
    use serde::{Serialize, Deserialize};

    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct Effect
    {
        pub id: String,
        pub lvl: i8
    }

    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct BossAbility
    {
        pub ability_type: String,
        pub delay: i32,
        pub location: String,
        pub config: Vec<String>
    }

    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct BossEquipmentItem
    {
        pub item_type: String,
        pub enchantments: Vec<Enchantment>
    }

    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct BossDrop
    {
        pub item: String,
        pub chance: f32
    }

    #[derive(Serialize, Deserialize, Clone, Debug)]
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

        pub fn moblist() -> [String; 77]
        {
            [AXOLOTL.to_string(), BAT.to_string(), CAT.to_string(), CHICKEN.to_string(), COD.to_string(), COW.to_string(), DONKEY.to_string(), FOX.to_string(), 
            GLOW_SQUID.to_string(), HORSE.to_string(), MOOSHROOM.to_string(), MULE.to_string(), OCELOT.to_string(), PARROT.to_string(), PIG.to_string(), PUFFERFISH.to_string(), RABBIT.to_string(), 
            SALMON.to_string(), SHEEP.to_string(), SKELETON_HORSE.to_string(), SNOW_GOLEM.to_string(), SQUID.to_string(), STRIDER.to_string(), TROPICAL_FISH.to_string(), TURTLE.to_string(), 
            VILLAGER.to_string(), WANDERING_TRADER.to_string(), BEE.to_string(), CAVE_SPIDER.to_string(), DOLPHIN.to_string(), ENDERMAN.to_string(), GOAT.to_string(), IRON_GOLEM.to_string(), 
            LLAMA.to_string(), PANDA.to_string(), PIGLIN.to_string(), POLAR_BEAR.to_string(), SPIDER.to_string(), TRADER_LLAMA.to_string(), WOLF.to_string(), ZOMBIFIED_PIGLIN.to_string(), 
            BLAZE.to_string(), CHICKEN_JOCKEY.to_string(), CREEPER.to_string(), DROWNED.to_string(), ELDER_GUARDIAN.to_string(), ENDERMITE.to_string(), EVOKER.to_string(), GHAST.to_string(), 
            GUARDIAN.to_string(), HOGLIN.to_string(), HUSK.to_string(), MAGMA_CUBE.to_string(), PHANTOM.to_string(), PIGLIN_BRUTE.to_string(), PILLAGER.to_string(), RAVAGER.to_string(), 
            SHULKER.to_string(), SILVERFISH.to_string(), SKELETON.to_string(), SKELETON_HORSEMAN.to_string(), SLIME.to_string(), SPIDER_JOCKEY.to_string(), STRAY.to_string(), 
            VEX.to_string(), VINDICATOR.to_string(), WITCH.to_string(), WITHER_SKELETON.to_string(), ZOGLIN.to_string(), ZOMBIE.to_string(), ZOMBIE_VILLAGER.to_string(), ENDERDRAGON.to_string(), 
            WITHER.to_string(), GIANT.to_string(), ZOMBIE_HORSE.to_string(), KILLER_BUNNY.to_string(), ILLUSIONER.to_string(), ]
        }
    }


    pub const HORIZONTAL_SPACE: &str = "                                                                                                                                                                                                                                      ";
}