use motion_bang_motion::video::Reel;

fn main() -> std::process::ExitCode {
    let _reel = Reel::load(".");
    println!("Bang Motion fframes engine ready.");
    std::process::ExitCode::SUCCESS
}
