use motion_sketchbook::video::Reel;

fn main() -> std::process::ExitCode {
    let _reel = Reel::load(".");
    println!("Sketchbook fframes engine ready.");
    std::process::ExitCode::SUCCESS
}
