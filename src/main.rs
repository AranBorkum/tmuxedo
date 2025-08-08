use crate::{tmuxedo::source_all_tmuxedo_files, tpm::run_tpm};

mod tmux;
mod tmuxedo;
mod tpm;

fn main() {
    source_all_tmuxedo_files();
    run_tpm();
}
