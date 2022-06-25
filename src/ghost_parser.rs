use std::collections::HashMap;
use crate::ghost::Ghost;
use crate::ghost::GhostFrame;
use crate::ghost::Vector3;
use crate::ghost::BulletType;
use crate::ghost::DiscardAbility;
use crate::ghost::TriggerEvent;

pub fn parse(ghost: &mut Ghost, buffer: &String) -> Result<(), &'static str> {
    if let Some((first_frame, remaining_frames)) = buffer.split_once("$") {
        // ghost header: {2/{forced id}/LEVEL NAME/total time/0.033333333$}
        let mut idx = 0;
        for field in first_frame.split("/") {
            match idx {
                0 => { if field != "2" {return Err("ghost first header field should always be '2'")}},
                1 => { ghost.forced_ghost_id = field.parse::<i64>().unwrap() },
                2 => { ghost.level_name = field.to_string() },
                3 => { ghost.total_time = field.parse::<f64>().unwrap() },
                4 => { if field != "0.03333" { return Err("5th header field should always be 0.03333")}},
                _ => {
                    return Err("unknown field [value: {field}] at index {idx}");
                }
            }
            idx += 1;
        }
        
        let file_frames: Vec<&str> = remaining_frames.split("|").collect();
        return parse_frames(ghost, &file_frames);
    } else {
        return Err("Failed to split between initial ghost frame and remaining ghost frames; exiting.");
    }
}

// "frame" serialization
// note: position, angle, and camera are relative to the last frame, except for frame 0, where they're initial values
// a{time difference*10,000 since last frame} e.g. a416 = 0.0416s since last tick
// b{position} = b{x,y,z} = b1,2,3
// c{facing direction}
// d{camera up/down angle}
// e{triggerEvent} int, nonzero
// f (playShotAnimation) true if present
// g (grounded) true if present
// h (ziplining) true if present
// i (stomping) true if present
// j{bulletId} int, nonzero
// k{bulletHitPos} = {x,y,z}, cannot be {0,0,0} - i hope that positional precision prevents that from ever occuring...
// | -> start of new frame

pub fn parse_frames(ghost: &mut Ghost, frames: &Vec<&str>) -> Result<(), &'static str> {
    let field_chars = ["a", "b", "c", "d", "e" , "f", "g", "h", "i", "j", "k"];
    let mut prev_frame: Option<GhostFrame> = None;
    let mut idx = 0;
    for frame in frames {
        // set up new frame
        let mut ghost_frame = GhostFrame::new();
        ghost_frame.index = idx;

        // Copy over cumulative values from previous frame if it exists.
        if let Some(prev) = prev_frame {
            ghost_frame.cumulative_time = prev.cumulative_time;
            ghost_frame.pos = prev.pos;
            ghost_frame.facing_angle = prev.facing_angle;
            ghost_frame.camera_pitch = prev.camera_pitch;
        }

        // Pull the fields out of the current frame.
        let mut fields: HashMap<String, String> = HashMap::new();

        for field_char in field_chars {
            // yeehaw
            if frame.contains(field_char) {
                let field_start_idx = frame.find(field_char).unwrap();
                let field_untruncated = frame.get(field_start_idx+1..).unwrap();
                if field_untruncated.contains(char::is_alphabetic) {
                    let truncation_idx = field_untruncated.find(char::is_alphabetic).unwrap();
                    let field_truncated = field_untruncated.get(..truncation_idx).unwrap();
                    fields.insert(
                        field_char.to_string(),
                        field_truncated.to_string()
                    );
                } else {
                    // nothing to truncate
                    fields.insert(
                        field_char.to_string(),
                        field_untruncated.to_string()
                    );
                }
            }
        }

        // parse the {field label, field} pairs and update current frame accordingly.
        for (field_label, field) in fields {
            match field_label.as_str() {
                "a" => {
                    let frame_time = field.parse::<i64>().unwrap();
                    ghost_frame.frame_time = (frame_time as f64) / 10000.0}
                "b" => {ghost_frame.pos_change = Vector3::from_string(field.as_str()).unwrap()}
                // camera stuff uses a weird serialization thing
                // it can sometimes result in "-" being serialized for a very small negative number
                "c" => { ghost_frame.facing_angle_change = if field == "-" {0.0} else {field.parse::<f64>().unwrap()} }
                "d" => { ghost_frame.camera_pitch_change = if field == "-" {0.0} else {field.parse::<f64>().unwrap()} }
                "e" => { 
                    let event_id = field.parse::<i64>().unwrap();
                    ghost_frame.trigger_event = match event_id {
                        1 => {TriggerEvent::Jump}
                        3 => {TriggerEvent::Land}
                        10..=20 => { TriggerEvent::Bullet(BulletType::from_trigger_event(event_id).unwrap()) }
                        1000..=1190 => {TriggerEvent::Discard(DiscardAbility::from_trigger_event(event_id).unwrap())}
                        2000 => {TriggerEvent::BulletHit}
                        _ => return Err("Spurious trigger event ID: {event_id}")
                    }
                }
                "f" => { ghost_frame.play_shot_animation = true}
                "g" => { ghost_frame.grounded = true }
                "h" => { ghost_frame.ziplining = true }
                "i" => { ghost_frame.stomping = true }
                "j" => { ghost_frame.bullet_id = field.parse::<i64>().unwrap() }
                "k" => { ghost_frame.bullet_hit_pos = Vector3::from_string(field.as_str()).unwrap()}
                _ => return Err("Unmatched field in frame [{frame}]. This should be impossible!")
            }            
        }
        // update cumulative values with new differentials
        ghost_frame.cumulative_time += ghost_frame.frame_time;
        ghost_frame.pos.add(ghost_frame.pos_change);
        ghost_frame.facing_angle += ghost_frame.facing_angle_change;
        ghost_frame.camera_pitch += ghost_frame.camera_pitch_change;
        // append, update, iterate.
        ghost.ghost_frames.push(ghost_frame);
        prev_frame = Some(ghost_frame);
        idx += 1;
    }
    Ok(())
}