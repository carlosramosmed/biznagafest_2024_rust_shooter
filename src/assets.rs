use crate::engine::TextureID;

pub const TEXTURES_PATHS: &[(TextureID, &str)] = &[
    // soldiers
    (TextureID::SoldierAlive1, "../../assets/sprites/npc/soldier/idle/0.png"),
    (TextureID::SoldierAlive2, "../../assets/sprites/npc/soldier/idle/1.png"),
    (TextureID::SoldierAlive3, "../../assets/sprites/npc/soldier/idle/2.png"),
    (TextureID::SoldierAlive4, "../../assets/sprites/npc/soldier/idle/3.png"),
    (TextureID::SoldierAlive5, "../../assets/sprites/npc/soldier/idle/4.png"),
    (TextureID::SoldierAlive6, "../../assets/sprites/npc/soldier/idle/5.png"),
    (TextureID::SoldierAlive7, "../../assets/sprites/npc/soldier/idle/6.png"),
    (TextureID::SoldierAlive8, "../../assets/sprites/npc/soldier/idle/7.png"),

    (TextureID::SoldierDead1,   "../../assets/sprites/npc/soldier/death/0.png"),
    (TextureID::SoldierDead2,  "../../assets/sprites/npc/soldier/death/1.png"),
    (TextureID::SoldierDead3,  "../../assets/sprites/npc/soldier/death/2.png"),
    (TextureID::SoldierDead4,  "../../assets/sprites/npc/soldier/death/3.png"),
    (TextureID::SoldierDead5,  "../../assets/sprites/npc/soldier/death/4.png"),
    (TextureID::SoldierDead6, "../../assets/sprites/npc/soldier/death/5.png"),
    (TextureID::SoldierDead7,  "../../assets/sprites/npc/soldier/death/6.png"),
    (TextureID::SoldierDead8,  "../../assets/sprites/npc/soldier/death/7.png"),

    (TextureID::SoldierWalking1, "../../assets/sprites/npc/soldier/walk/0.png"),
    (TextureID::SoldierWalking2, "../../assets/sprites/npc/soldier/walk/1.png"),
    (TextureID::SoldierWalking3, "../../assets/sprites/npc/soldier/walk/2.png"),
    (TextureID::SoldierWalking4, "../../assets/sprites/npc/soldier/walk/3.png"),

    (TextureID::SoldierShooting1, "../../assets/sprites/npc/soldier/attack/0.png"),
    (TextureID::SoldierShooting2, "../../assets/sprites/npc/soldier/attack/1.png"),

    // soldier pain
    (TextureID::SoldierPain, "../../assets/sprites/npc/soldier/pain/0.png"),
    
    // walls
    (TextureID::Wall,          "../../assets/textures/1.png"),

    // weapon
    (TextureID::WeaponIdle,    "../../assets/sprites/weapon/shotgun/0.png"),
    (TextureID::WeaponShoot,   "../../assets/sprites/weapon/shotgun/1.png"),
    (TextureID::WeaponReload1,  "../../assets/sprites/weapon/shotgun/2.png"),
    (TextureID::WeaponReload2, "../../assets/sprites/weapon/shotgun/3.png"),
    (TextureID::WeaponReload3, "../../assets/sprites/weapon/shotgun/4.png"),
    (TextureID::WeaponReload4, "../../assets/sprites/weapon/shotgun/5.png"),

    (TextureID::PainScreen, "../../assets/textures/blood_screen.png"),
    (TextureID::GameOver, "../../assets/textures/game_over.png"),

    (TextureID::Zero,  "../../assets/textures/digits/0.png"),
    (TextureID::One,   "../../assets/textures/digits/1.png"),
    (TextureID::Two,   "../../assets/textures/digits/2.png"),
    (TextureID::Three, "../../assets/textures/digits/3.png"), 
    (TextureID::Four,  "../../assets/textures/digits/4.png"),
    (TextureID::Five,  "../../assets/textures/digits/5.png"),
    (TextureID::Six,   "../../assets/textures/digits/6.png"),
    (TextureID::Seven, "../../assets/textures/digits/7.png"), 
    (TextureID::Eight, "../../assets/textures/digits/8.png"), 
    (TextureID::Nine,  "../../assets/textures/digits/9.png"),
];