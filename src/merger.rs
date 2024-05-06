use std::process::Command;

pub fn merge_mp4_image(
    image_file_path: String,
    input_video_path: &str,
    video_file_path: String,
) -> bool {
    match Command::new("ffmpeg")
        .args([
            "-loop",
            "1",
            "-framerate",
            "30",
            "-t",
            "5",
            "-i",
            &image_file_path,
            "-i",
            &input_video_path,
            "-filter_complex",
            "[1][0]concat=n=2:v=1:a=0",
            &video_file_path,
        ])
        .output()
    {
        Ok(_) => true,
        Err(message) => {
            println!("{}", message.to_string());
            false
        }
    }
}
