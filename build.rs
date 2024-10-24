use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    // Get the output directory where Cargo will place build artifacts
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("texture_id.rs");
    let mut f = File::create(&dest_path).unwrap();

    // Start writing the enum definition
    writeln!(
        f,
        "#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]\npub enum TextureID {{"
    )
    .unwrap();

    // Helper function to generate numbered variants
    fn generate_variants(f: &mut File, base_name: &str, count: usize) {
        for i in 1..=count {
            writeln!(f, "    {}{},", base_name, i).unwrap();
        }
    }

    // Write non-numbered variants
    let simple_variants = [
        "Wall",
        "WeaponIdle",
        "WeaponShoot",
        "SoldierPain",
        "PainScreen",
        "GameOver",
        "Zero", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine",
    ];

    for variant in &simple_variants {
        writeln!(f, "    {},", variant).unwrap();
    }

    // Generate numbered variants
    generate_variants(&mut f, "WeaponReload", 4);
    generate_variants(&mut f, "SoldierAlive", 8);
    generate_variants(&mut f, "SoldierDead", 8);
    generate_variants(&mut f, "SoldierWalking", 4);
    generate_variants(&mut f, "SoldierShooting", 0);
    generate_variants(&mut f, "SoldierShooting", 2);

    // Close the enum definition
    writeln!(f, "}}").unwrap();

    // Instruct Cargo to rerun the build script if the build script changes
    println!("cargo:rerun-if-changed=build.rs");
}
