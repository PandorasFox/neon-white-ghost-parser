#[derive(Copy, Clone)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    pub fn from_string(s: &str) -> Result<Vector3, &'static str> {
        let mut idx = 0;
        let mut result = Vector3{x:0.0, y:0.0, z:0.0};
        for field in s.split(",") {
            let val = match field.parse::<f64>() {
                Ok(v) => v,
                _ => 0.0,
            };
            match idx {
                0 => {result.x = val}
                1 => {result.y = val}
                2 => {result.z = val}
                _ => {return Err("Too many values to unpack in given string '{s}'!")}
            }
            idx += 1
        }
        return Ok(result);
    }

    pub fn add(&mut self, v: Vector3) {
        self.x += v.x;
        self.y += v.y;
        self.z += v.z;
    }
}

#[derive(Copy, Clone)]
pub enum DiscardAbility {
    Jump = 10,
    Flap = 15,
    Bomb = 20,
    Freeze = 30,
    Dash = 40,
    Stomp = 50,
    Telefrag = 60,
    Kickback = 70,
    Stun = 80,
    Consumable = 90,
    ShieldBash = 100,
    Rocket = 110,
    Resurrect = 120,
    PoisonBoost = 130,
    ZipLine = 140,
    Mine = 150,
    Rapture = 160,
    Miracle = 170,
    Backfire = 180,
    Fireball = 190,
}

impl DiscardAbility {
    pub fn from_trigger_event(i: i64) -> Result<DiscardAbility, &'static str> {
        let res = match i - 1000 {
            10 => {DiscardAbility::Jump}
            15 => {DiscardAbility::Flap}
            20 => {DiscardAbility::Bomb}
            30 => {DiscardAbility::Freeze}
            40 => {DiscardAbility::Dash}
            50 => {DiscardAbility::Stomp}
            60 => {DiscardAbility::Telefrag}
            70 => {DiscardAbility::Kickback}
            80 => {DiscardAbility::Stun}
            90 => {DiscardAbility::Consumable}
            100 => {DiscardAbility::ShieldBash}
            110 => {DiscardAbility::Rocket}
            120 => {DiscardAbility::Resurrect}
            130 => {DiscardAbility::PoisonBoost}
            140 => {DiscardAbility::ZipLine}
            150 => {DiscardAbility::Mine}
            160 => {DiscardAbility::Rapture}
            170 => {DiscardAbility::Miracle}
            180 => {DiscardAbility::Backfire}
            190 => {DiscardAbility::Fireball}
            _ => {return Err("Invalid discardAbility [{i}]")}
        };
        return Ok(res)
    }
}

#[derive(Copy, Clone)]
pub enum BulletType {
    Swipe,
    SingleShot,
    Scatter,
    Rocket,
    Mine
}

impl BulletType {
    pub fn from_trigger_event(i: i64) -> Result<BulletType, &'static str> {
        let res = match i - 10 {
            0 => BulletType::Swipe,
            1 => BulletType::SingleShot,
            2 => BulletType::Scatter,
            3 => BulletType::Rocket,
            4 => BulletType::Mine,
            _ => {return Err("Invalid bulletType [{i}]")}
        };
        return Ok(res)
    }
}

#[derive(Copy, Clone)]
pub enum TriggerEvent {
    None,
    Jump,
    // Fall,
    Land,
    Bullet(BulletType),
    Discard(DiscardAbility),
    BulletHit,
}

// trigger event
// it is either: DiscardAbility+1000
// bulletType+10
// 1 = jump
// 3 = landed
// 2000 = bulletHit
// 

#[derive(Copy, Clone)]
pub struct GhostFrame {
    pub trigger_event: TriggerEvent, 
    pub play_shot_animation: bool,
    
    pub cumulative_time: f64,
    pub frame_time: f64,

    pub pos: Vector3,
    pub pos_change: Vector3,

    pub facing_angle: f64,
    pub facing_angle_change: f64,

    pub camera_pitch: f64,
    pub camera_pitch_change: f64,
    
    pub grounded: bool,
    pub stomping: bool,
    pub ziplining: bool,
    
    pub bullet_id: i64,
    pub bullet_hit_pos: Vector3,

    pub index: i64,
}

pub struct Ghost {
    pub level_name: String,
    pub forced_ghost_id: i64, // seems to always be 0?
    pub total_time: f64,
    // pub frame_time = 0.0333..... // this one is literally just always written as 0.03333, not bothering to store it
    pub ghost_frames: Vec<GhostFrame>,
}

impl Ghost {
    pub fn new() -> Ghost {
        Ghost{
            ghost_frames: Vec::with_capacity(47000),
            level_name: String::new(),
            total_time: -1.0,
            forced_ghost_id: -1,
        }
    }
}

impl GhostFrame {
    pub fn new() -> GhostFrame {
        GhostFrame {
            cumulative_time: 0.0,
            frame_time: 0.0,
            index: -1,

            pos: Vector3{x: 0.0, y: 0.0, z: 0.0},
            pos_change: Vector3{x: 0.0, y: 0.0, z: 0.0},
            facing_angle: 0.0,
            facing_angle_change: 0.0,
            camera_pitch: 0.0,
            camera_pitch_change: 0.0,


            grounded: false,
            stomping: false,
            ziplining: false,

            trigger_event: TriggerEvent::None,
            play_shot_animation: false,
            bullet_id: 0,
            bullet_hit_pos: Vector3{x: 0.0, y: 0.0, z: 0.0},
        }
    }
}