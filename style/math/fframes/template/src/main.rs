use motion_math::video::Reel;

fn main() -> std::process::ExitCode {
    let _reel = Reel::load(".");
    println!("Math fframes engine ready.");
    std::process::ExitCode::SUCCESS
}
