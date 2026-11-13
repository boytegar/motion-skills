use motion_riso::video::Reel;

fn main() -> std::process::ExitCode {
    let _reel = Reel::load(".");
    println!("Riso fframes engine ready.");
    std::process::ExitCode::SUCCESS
}
