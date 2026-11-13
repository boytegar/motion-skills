use motion_line_art_animation::video::Reel;

fn main() -> std::process::ExitCode {
    let _reel = Reel::load(".");
    println!("Line Art Animation fframes engine ready.");
    std::process::ExitCode::SUCCESS
}
