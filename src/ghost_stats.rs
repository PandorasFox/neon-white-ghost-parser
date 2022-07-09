use crate::ghost::Ghost;
use crate::ghost::GhostFrame;
use crate::ghost::Vector3;
use std::io::{self, Write};
/*
use crate::ghost::BulletType;
use crate::ghost::DiscardAbility;
use crate::ghost::TriggerEvent;
*/

pub trait GhostStatCollector {
    fn collect(&mut self, frame: &GhostFrame);
    fn emit (&self) -> String;
}

pub struct FrameTimeStats {
    pub num_frames: i64,
    pub min_frame_time: f64,
    pub max_frame_time: f64,
    pub frame_time_sum: f64
}

impl FrameTimeStats {
    pub fn new() -> FrameTimeStats {
        FrameTimeStats {
            num_frames: 0,
            min_frame_time: 0.0,
            max_frame_time: 0.0,
            frame_time_sum: 0.0,
        }
    }
}

impl GhostStatCollector for FrameTimeStats {
    fn collect(&mut self, frame: &GhostFrame) {
        if frame.index == 0 {return};
        if frame.frame_time == 0.0 {return}; // why does this happen aside from frame 0
        self.num_frames += 1;
        self.frame_time_sum += frame.frame_time;
        if frame.frame_time > self.max_frame_time {
            self.max_frame_time = frame.frame_time;
        }
        if frame.index == 1 || frame.frame_time < self.min_frame_time {
            self.min_frame_time = frame.frame_time;
        }
    }

    fn emit(&self) -> String {
        let avg_frametime = self.frame_time_sum / self.num_frames as f64;
        format!("Frame Time stats:\n\
        Cumulative frametime: {}\n\
        Frametime min/average/max: {} / {avg_frametime} / {}\n", self.frame_time_sum, self.min_frame_time, self.max_frame_time)
    }
}

pub struct VelocityStats {
    pub last_pos: Vector3,
    pub last_y_vel: f64,
    pub last_lat_vel: f64,

    pub max_upwards_y_vel: f64,
    pub max_downward_y_vel: f64,
    pub max_lateral_vel: f64,
    pub avg_lateral_vel: f64,

    pub max_accel_in_one_frame: f64,
    pub num_jumps: i64,
}

impl VelocityStats {
    pub fn new() -> VelocityStats {
        VelocityStats {
            last_pos: Vector3::zero(),
            last_y_vel: 0.0,
            last_lat_vel: 0.0,

            max_upwards_y_vel: 0.0,
            max_downward_y_vel: 0.0,
            max_lateral_vel: 0.0,
            avg_lateral_vel: 0.0,

            max_accel_in_one_frame: 0.0,
            num_jumps: 0,
        }
    } 
}

impl GhostStatCollector for VelocityStats {
    fn collect(&mut self, frame: &GhostFrame) {
        if frame.index != 0 && frame.frame_time != 0.0 {
            let y_diff = frame.pos.y - self.last_pos.y;
            let lat_diff = ((frame.pos.x - self.last_pos.x).powf(2.0) + (frame.pos.z - self.last_pos.z).powf(2.0)).sqrt();

            // compute velocity based on frametime
            // TODO: figure out a better way to map position difference per frametime to current velocity
            // time to go spelunking in .updateVelocity again.....
            let y_velocity = y_diff / frame.frame_time;
            let lat_velocity = lat_diff / frame.frame_time;
            if lat_velocity > self.max_lateral_vel {
                self.max_lateral_vel = lat_velocity;
            }
            
            if y_velocity > self.max_upwards_y_vel {
                self.max_upwards_y_vel = y_velocity;
            } else if y_velocity < self.max_downward_y_vel {
                self.max_downward_y_vel = y_velocity;
            }

            // check changes in velocity! we want to track greatest changes too.
            let velocity_diff = lat_velocity - self.last_lat_vel;
            if velocity_diff > self.max_accel_in_one_frame {
                self.max_accel_in_one_frame = velocity_diff;
            }

            if (self.last_y_vel == 0.0 ) && y_velocity > 0.0 {
                self.num_jumps += 1;
            }

            self.last_lat_vel = lat_velocity;
            self.last_y_vel = y_velocity;
        }
        self.last_pos = frame.pos;
    }

    fn emit(&self) -> String {
        let num_jumps = self.num_jumps;
        let max_accel_in_one_frame = self.max_accel_in_one_frame;
        let max_y_vel = self.max_upwards_y_vel;
        let min_y_vel = self.max_downward_y_vel;
        let max_lat_vel = self.max_lateral_vel;
        format!("Velocity stats\n\
                 Num jumps: {num_jumps}\n\
                 Max acceleration in one frame: {max_accel_in_one_frame}\n\
                 Max lateral velocity: {max_lat_vel}\n\
                 Maximum upwards velocity: {max_y_vel}\n\
                 Maximum downwards velocity: {min_y_vel}\n\
                 [Max water velocity is 33.75, max walking velocity is 18.75]\n")
    }
}

pub fn collect_stats(ghost: &Ghost) {
    // todo: have a list of frame consumers by Trait, iterate!
    let mut velocity_consumer = VelocityStats::new();
    let mut frametime_consumer = FrameTimeStats::new();
    for frame in &ghost.ghost_frames {
        velocity_consumer.collect(frame);
        frametime_consumer.collect(frame);
    }

    let level_name = &ghost.level_name;
    let ghost_time = &ghost.total_time;

    println!("Ghost stats for {level_name}:");
    println!("Ghost time: {ghost_time}\n");
    io::stdout().write_all(frametime_consumer.emit().as_bytes()).unwrap();
    println!();
    io::stdout().write_all(velocity_consumer.emit().as_bytes()).unwrap();
}