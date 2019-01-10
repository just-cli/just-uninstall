use just_core::kernel::InstalledPackages;
use semver::Version;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "install")]
struct Opt {
    #[structopt(long = "package")]
    pub package: Option<String>,
}

fn uninstall(pkg_name: &str, packages: &mut InstalledPackages) -> Option<Version> {
    if packages.is_installed(pkg_name, None) {
        packages.remove_package(pkg_name)
    } else {
        None
    }
}

fn main() {
    use just_core::kernel::Kernel;

    let opt: Opt = Opt::from_args();
    if let Some(pkg_name) = opt.package {
        let mut kernel = Kernel::load();
        if let Some(version) = uninstall(&pkg_name, &mut kernel.packages) {
            println!(
                "Package {} {} was successfully uninstalled",
                pkg_name, version
            );
        } else {
            println!("Package {} is not installed", pkg_name);
        }
    }
}
