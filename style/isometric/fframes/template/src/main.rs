use motion_isometric::video::Reel;

fn main() -> std::process::ExitCode {
    let _reel = Reel::load(".");
    println!("Isometric fframes engine ready.");
    std::process::ExitCode::SUCCESS
}
