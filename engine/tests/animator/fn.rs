use super::*;

fn frame(duration: f64) -> SpriteFrame {
    SpriteFrame::new(
        Rect::from_center(Vector2D::new(0.0, 0.0), 16.0, 16.0),
        duration,
    )
}

fn animation(durations: &[f64], mode: AnimationMode) -> SpriteAnimation {
    let frames: Vec<SpriteFrame> = durations.iter().map(|d: &f64| frame(*d)).collect();
    SpriteAnimation::new(String::from("run"), frames, mode)
}

#[test]
fn a_new_animator_starts_paused_on_the_first_frame() {
    let animator: Animator = Animator::create();
    assert_eq!(
        animator.get_state(),
        AnimationState::Paused,
        "an animator must be told to play before it moves"
    );
    assert_eq!(
        animator.get_current_frame_index(),
        0,
        "and it starts at frame zero"
    );
}

#[test]
fn a_new_animator_has_no_animation_attached() {
    let animator: Animator = Animator::create();
    assert_eq!(
        animator.try_get_current_animation(),
        None,
        "nothing is loaded until play() hands over an animation"
    );
    assert_eq!(
        animator.current_frame_source(),
        None,
        "so there is no frame rectangle to report yet"
    );
}

#[test]
fn pausing_a_paused_animator_changes_nothing() {
    let mut animator: Animator = Animator::create();
    animator.pause();
    assert_eq!(
        animator.get_state(),
        AnimationState::Paused,
        "pause is idempotent on an already paused animator"
    );
}

#[test]
fn resume_moves_a_paused_animator_into_playing() {
    let mut animator: Animator = Animator::create();
    animator.resume();
    assert_eq!(
        animator.get_state(),
        AnimationState::Playing,
        "resume is what actually starts playback"
    );
}

#[test]
fn resume_leaves_an_already_playing_animator_alone() {
    let mut animator: Animator = Animator::create();
    animator.resume();
    animator.resume();
    assert_eq!(
        animator.get_state(),
        AnimationState::Playing,
        "resume must not flip a finished animation back into play"
    );
}

#[test]
fn stop_returns_the_animator_to_paused_at_frame_zero() {
    let mut animator: Animator = Animator::create();
    animator.play(animation(&[0.1, 0.1], AnimationMode::Loop));
    animator.update(0.1);
    assert_eq!(
        animator.get_current_frame_index(),
        1,
        "it advanced before the stop"
    );
    animator.stop();
    assert_eq!(
        animator.get_state(),
        AnimationState::Paused,
        "stop halts playback"
    );
    assert_eq!(
        animator.get_current_frame_index(),
        0,
        "and rewinds to the first frame"
    );
}

#[test]
fn play_attaches_the_animation_and_starts_at_frame_zero() {
    let mut animator: Animator = Animator::create();
    animator.play(animation(&[0.1, 0.2], AnimationMode::Loop));
    assert_eq!(
        animator.get_state(),
        AnimationState::Playing,
        "play starts playback"
    );
    assert_eq!(
        animator.get_current_frame_index(),
        0,
        "starting at the first frame"
    );
}

#[test]
fn play_forwards_in_the_positive_direction() {
    let mut animator: Animator = Animator::create();
    animator.play(animation(&[0.1, 0.1], AnimationMode::Loop));
    assert_eq!(
        animator.get_direction(),
        1,
        "a freshly played animation runs forwards"
    );
}

#[test]
fn a_frame_source_is_reported_once_an_animation_is_loaded() {
    let mut animator: Animator = Animator::create();
    animator.play(animation(&[0.1, 0.1], AnimationMode::Loop));
    let observed: Option<Rect> = animator.current_frame_source();
    assert!(
        observed.is_some(),
        "a loaded animation reports its current rectangle"
    );
}

#[test]
fn update_before_the_frame_duration_keeps_the_same_frame() {
    let mut animator: Animator = Animator::create();
    animator.play(animation(&[0.5, 0.5], AnimationMode::Loop));
    animator.update(0.1);
    assert_eq!(
        animator.get_current_frame_index(),
        0,
        "an update shorter than the frame does not advance"
    );
}

#[test]
fn update_past_the_frame_duration_advances() {
    let mut animator: Animator = Animator::create();
    animator.play(animation(&[0.1, 0.1], AnimationMode::Loop));
    animator.update(0.1);
    assert_eq!(
        animator.get_current_frame_index(),
        1,
        "reaching the duration rolls into the next frame"
    );
}

#[test]
fn a_looping_animation_wraps_at_the_end() {
    let mut animator: Animator = Animator::create();
    animator.play(animation(&[0.1, 0.1], AnimationMode::Loop));
    animator.update(0.1);
    animator.update(0.1);
    assert_eq!(
        animator.get_current_frame_index(),
        0,
        "after the last frame a loop returns to the first"
    );
}

#[test]
fn a_looping_animation_never_finishes() {
    let mut animator: Animator = Animator::create();
    animator.play(animation(&[0.1], AnimationMode::Loop));
    for _ in 0..10 {
        animator.update(0.1);
    }
    assert_eq!(
        animator.get_state(),
        AnimationState::Playing,
        "a loop has no end state no matter how long it runs"
    );
}

#[test]
fn a_once_animation_finishes_after_its_last_frame() {
    let mut animator: Animator = Animator::create();
    animator.play(animation(&[0.1, 0.1], AnimationMode::Once));
    animator.update(0.1);
    animator.update(0.1);
    assert_eq!(
        animator.get_state(),
        AnimationState::Finished,
        "playing through the last frame ends the animation"
    );
}

#[test]
fn a_once_animation_stops_on_the_last_frame() {
    let mut animator: Animator = Animator::create();
    animator.play(animation(&[0.1, 0.1], AnimationMode::Once));
    animator.update(0.1);
    animator.update(0.1);
    assert_eq!(
        animator.get_current_frame_index(),
        1,
        "the last frame is where a once animation comes to rest"
    );
}

#[test]
fn a_ping_pong_animation_reverses_after_the_last_frame() {
    let mut animator: Animator = Animator::create();
    animator.play(animation(&[0.1, 0.1, 0.1], AnimationMode::PingPong));
    animator.update(0.1);
    animator.update(0.1);
    assert_eq!(
        animator.get_current_frame_index(),
        2,
        "two steps land on the last frame"
    );
    assert_eq!(
        animator.get_direction(),
        1,
        "and it is still heading forwards"
    );
    animator.update(0.1);
    assert_eq!(
        animator.get_current_frame_index(),
        1,
        "the step off the end steps back rather than wrapping"
    );
    assert_eq!(
        animator.get_direction(),
        -1,
        "and the direction flips to match"
    );
}

#[test]
fn a_ping_pong_animation_comes_back_to_the_first_frame_still_heading_back() {
    let mut animator: Animator = Animator::create();
    animator.play(animation(&[0.1, 0.1, 0.1], AnimationMode::PingPong));
    for _ in 0..4 {
        animator.update(0.1);
    }
    assert_eq!(
        animator.get_current_frame_index(),
        0,
        "a full there-and-back ends on the first frame"
    );
    assert_eq!(
        animator.get_direction(),
        -1,
        "having just arrived at the first frame while going backwards, it is \
         still pointed backwards and will reverse on the next step"
    );
}

#[test]
fn a_ping_pong_animation_alternates_direction_at_each_edge() {
    let mut animator: Animator = Animator::create();
    animator.play(animation(&[0.1, 0.1, 0.1], AnimationMode::PingPong));
    for _ in 0..5 {
        animator.update(0.1);
    }
    assert_eq!(
        animator.get_direction(),
        1,
        "stepping off the first frame while going backwards reverses it again"
    );
    assert_eq!(
        animator.get_current_frame_index(),
        1,
        "and the step leaves the frame it was on"
    );
}

#[test]
fn a_ping_pong_animation_never_finishes() {
    let mut animator: Animator = Animator::create();
    animator.play(animation(&[0.1, 0.1], AnimationMode::PingPong));
    for _ in 0..8 {
        animator.update(0.1);
    }
    assert_eq!(
        animator.get_state(),
        AnimationState::Playing,
        "ping-pong has no end state"
    );
}

#[test]
fn an_animation_with_no_frames_does_not_advance() {
    let mut animator: Animator = Animator::create();
    animator.play(animation(&[], AnimationMode::Loop));
    animator.update(1.0);
    assert_eq!(
        animator.get_current_frame_index(),
        0,
        "an empty animation has nothing to step to"
    );
}

#[test]
fn a_paused_animator_ignores_updates() {
    let mut animator: Animator = Animator::create();
    animator.play(animation(&[0.1, 0.1], AnimationMode::Loop));
    animator.pause();
    animator.update(1.0);
    assert_eq!(
        animator.get_current_frame_index(),
        0,
        "a paused animator must not advance no matter how big the delta"
    );
}

#[test]
fn an_update_with_no_animation_is_a_no_op() {
    let mut animator: Animator = Animator::create();
    animator.resume();
    animator.update(1.0);
    assert_eq!(
        animator.get_state(),
        AnimationState::Playing,
        "a playing animator with no animation just does nothing"
    );
}

#[test]
fn each_frame_keeps_its_own_duration() {
    let mut animator: Animator = Animator::create();
    animator.play(animation(&[0.1, 0.5], AnimationMode::Loop));
    animator.update(0.1);
    assert_eq!(
        animator.get_current_frame_index(),
        1,
        "the short frame elapsed first"
    );
    animator.update(0.2);
    assert_eq!(
        animator.get_current_frame_index(),
        1,
        "a fifth of a second is well short of the long frame's half second"
    );
    animator.update(0.3);
    assert_eq!(
        animator.get_current_frame_index(),
        0,
        "together the two updates reach half a second, so the long frame elapses and the loop wraps"
    );
}

#[test]
fn the_default_animation_mode_is_looping() {
    let observed: AnimationMode = AnimationMode::default();
    assert_eq!(
        observed,
        AnimationMode::Loop,
        "sprite sheets repeat by default"
    );
}

#[test]
fn the_default_animation_state_is_playing() {
    let observed: AnimationState = AnimationState::default();
    assert_eq!(
        observed,
        AnimationState::Playing,
        "a struct default is not a runtime one"
    );
}
