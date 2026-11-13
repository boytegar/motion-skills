use motion_pixel::video::Reel;

fn main() -> std::process::ExitCode {
    let _reel = Reel::load(".");
    println!("Pixel fframes engine ready.");
    std::process::ExitCode::SUCCESS
}
