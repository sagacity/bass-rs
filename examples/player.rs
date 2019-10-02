use bass_rs::*;
use std::io::Read;
use std::time::Duration;

fn main() {
    println!("{:?}", get_device_info());

    let result = unsafe { BASS_Init(4, 44100, 0, 0, 0) };
    assert_eq!(result, 1);

    let mut audio_file = std::fs::File::open("/Users/rjacobs/sandbox/neon/data/music.ogg").unwrap();
    let mut audio_buffer = Vec::new();
    audio_file.read_to_end(&mut audio_buffer).unwrap();

    let stream_handle = unsafe {
        let ptr = audio_buffer.as_ptr();
        BASS_StreamCreateFile(1, ptr, 0, audio_buffer.len() as u64, 0)
    };
    let play_result = unsafe { BASS_ChannelPlay(stream_handle, 0) };
    assert_eq!(play_result, 1);

    std::thread::sleep(Duration::from_millis(15400));

    // Loop the music three times, restarting each loop from 1 second from the beginning
    let mut loop_count = 0;
    while loop_count < 3 {
        let bytes_pos = unsafe { BASS_ChannelGetPosition(stream_handle, 0) };
        let seconds_pos = unsafe { BASS_ChannelBytes2Seconds(stream_handle, bytes_pos) };
        println!("{} / {}", bytes_pos, seconds_pos);
        std::thread::sleep(Duration::from_millis(400));

        if seconds_pos > 5.0 {
            loop_count = loop_count + 1;

            // Set the new position to '1 second from start'
            let new_pos = unsafe { BASS_ChannelSeconds2Bytes(stream_handle, 1.0 )};
            unsafe { BASS_ChannelSetPosition(stream_handle, new_pos, 0) };
        }
    }

    let free_stream_result = unsafe { BASS_StreamFree(stream_handle) };
    assert_eq!(free_stream_result, 1);

    let free_result = unsafe { BASS_Free() };
    assert_eq!(free_result, 1);
}
