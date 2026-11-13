use motion_cut_paper::video::Reel;

fn main() -> std::process::ExitCode {
    let _reel = Reel::load(".");
    println!("CutPaper fframes engine ready.");
    std::process::ExitCode::SUCCESS
}
