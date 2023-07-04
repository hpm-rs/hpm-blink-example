use hpm_rt::*;

fn main() {
    cfg_if::cfg_if! {
        if #[cfg(debug_assertions)] {
            RuntimeBuilder::from_ram(Family::HPM6700_6400)
                .build()
                .unwrap();
        } else {
            let xpi_nor_cfg = XpiNorConfigurationOption::new();

            RuntimeBuilder::from_flash(Family::HPM6700_6400, xpi_nor_cfg)
                .xpi0_flash_size(8 * 1024 * 1024)
                .build()
                .unwrap();
        }
    }
    println!("cargo:rerun-if-changed=build.rs");
}
