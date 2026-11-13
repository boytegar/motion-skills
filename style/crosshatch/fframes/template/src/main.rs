use motion_crosshatch::video::Reel;

fn main() -> std::process::ExitCode {
    let _reel = Reel::load(".");
    println!("Crosshatch fframes engine ready.");
    std::process::ExitCode::SUCCESS
}
