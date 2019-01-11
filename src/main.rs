use just_core::kernel::InstalledPackages;
use just_core::kernel::PackageShims;
use just_core::result::BoxedResult;
use semver::Version;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "install")]
struct Opt {
    #[structopt(long = "package")]
    pub package: Option<String>,
}

fn uninstall(pkg_name: &str) -> BoxedResult<()> {
    use just_core::kernel::Kernel;
    use just_core::manifest::ManifestFiles;
    use log::error;

    let mut kernel = Kernel::load();
    if let Some(version) = remove_package(&pkg_name, &mut kernel.packages) {
        match ManifestFiles::scan(&kernel).load_manifest(pkg_name) {
            Some(manifest) => {
                kernel.remove_shims(&manifest.package)?;

                println!(
                    "Package {}-{} was successfully uninstalled",
                    pkg_name, version
                );
            }
            None => error!("No Package with name {} found", pkg_name),
        }

        Ok(())
    } else {
        println!("Package {} is not installed", pkg_name);

        Ok(())
    }
}

fn remove_package(pkg_name: &str, packages: &mut InstalledPackages) -> Option<Version> {
    if packages.is_installed(pkg_name, None) {
        packages.remove_package(pkg_name)
    } else {
        None
    }
}

fn main() {
    let opt: Opt = Opt::from_args();
    if let Some(pkg_name) = opt.package {
        uninstall(&pkg_name).ok();
    }
}
