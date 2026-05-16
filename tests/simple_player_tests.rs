mod common;

use avaudio::prelude::*;

#[test]
fn simple_player_surface() -> Result<(), Box<dyn std::error::Error>> {
    let audio_path = common::artifacts_dir()?.join("simple-player-test.aiff");
    common::make_test_audio(&audio_path)?;

    let player = AudioSimplePlayer::create_from_path(&audio_path)?;
    assert!(player.prepare_to_play());
    assert!(player.duration() > 0.0);

    player.set_volume(0.4);
    player.set_pan(0.1);
    player.set_rate(1.05);
    player.set_current_time(0.0);
    player.set_number_of_loops(0);

    assert!((player.volume() - 0.4).abs() < 0.001);
    assert!((player.pan() - 0.1).abs() < 0.001);
    assert!((player.rate() - 1.05).abs() < 0.001);
    assert_eq!(player.number_of_loops(), 0);
    Ok(())
}

#[test]
#[ignore = "requires audio output hardware"]
fn simple_player_playback() -> Result<(), Box<dyn std::error::Error>> {
    let audio_path = common::artifacts_dir()?.join("simple-player-playback.aiff");
    common::make_test_audio(&audio_path)?;
    let player = AudioSimplePlayer::create_from_path(&audio_path)?;
    assert!(player.play());
    player.stop();
    Ok(())
}
