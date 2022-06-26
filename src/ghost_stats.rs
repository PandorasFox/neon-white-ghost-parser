use crate::ghost::Ghost;
use crate::ghost::GhostFrame;
use crate::ghost::Vector3;
use crate::ghost::BulletType;
use crate::ghost::DiscardAbility;
use crate::ghost::TriggerEvent;

pub fn collect_stats(ghost: &Ghost) {
    let mut num_jumps = 0;
    let mut min_frame_time = 999.9;
    let mut max_frame_time = 0.0;
    let mut sum_of_frame_times = 0.0;

    let mut grounded_last_frame = false;

    // todo: write separate stats collectors with their own state, and dispatch to them every frame

    for frame in &ghost.ghost_frames {
        sum_of_frame_times += frame.frame_time;
        if frame.frame_time > max_frame_time {
            max_frame_time = frame.frame_time;
        }
        if frame.frame_time != 0.0 && frame.frame_time < min_frame_time {
            min_frame_time = frame.frame_time;
        }
        if grounded_last_frame && !frame.grounded {
            num_jumps += 1
        }
        grounded_last_frame = frame.grounded;
    }

    let level_name = &ghost.level_name;
    let ghost_time = &ghost.total_time;

    println!("Ghost stats for {level_name}:");
    println!("Ghost time / total frame time: {ghost_time} / {sum_of_frame_times}");
    println!("# jumps: {num_jumps}");
    println!("min frame time: {min_frame_time}");
    println!("max frame time: {max_frame_time}");
}