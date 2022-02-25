import logging
import subprocess

from pathlib import Path

from fabric import Connection

log = logging.getLogger(__name__)

hosts = "colte1.local, colte2.local"


def build_dauth_manager(target):
    """ Build dauth manager from source via cargo
    """
    if target == "debug":
        cmd = ["cargo", "build", "--debug"]
    elif target == "release":
        cmd = ["cargo", "build", "--release"]
    else:
        raise ValueError("Invalid target mode: {}".format(target))

    log.info("Running build command: %s", cmd)
    subprocess.run(cmd, check=True, cwd="../manager-rs")


def package_dauth_manager(target, package_name="dauth-manager_0.0.0~dev_amd64.deb"):
    """ Package the dauth manager per external nfpm.yaml config file
    """
    with open("../manager-rs/nfpm.yaml") as f:
        nfpm_config = f.read()

    # Update the config file TARGET placeholder with the appropriate target
    nfpm_config = nfpm_config.replace(r"${TARGET}", target)
    log.debug("Running nfpm with config: \n %s", nfpm_config)

    subprocess.run(["nfpm", "package", "--config", "/dev/stdin", "--packager", "deb", "--target", package_name],
                   check=True,
                   cwd="../manager-rs",
                   input=nfpm_config.encode("utf8"))

    package_path = Path("../manager-rs", package_name)
    log.info("Package created at: %s", package_path.absolute())
    return package_path


def deploy_package(package_path, host):
    """ Transfer and install the provided package on the host
    """

    package_name = package_path.name

    result = Connection(host).put(package_path, remote="/tmp/", preserve_mode=False)
    result = Connection(host).sudo("dpkg -i /tmp/{}".format(package_name))


def build_open5gs_packages():
    """ Builds our open5gs deb packages from source via dpkg-buildpkg
    """

    subprocess.run(["dpkg-buildpackage", "-us", "-uc"],
                   check=True,
                   cwd="../open5gs")

    # Clean up packaging products
    Path("../debug-open5gs-debs").mkdir(exist_ok=True, parents=True)
    for debug_deb in Path("../").glob("open5gs-*-dbgsym*.ddeb"):
        debug_deb.replace(Path("../debug-open5gs-debs") / debug_deb.name)

    Path("../open5gs_2.3.6.dsc").unlink(missing_ok=True)
    Path("../open5gs_2.3.6_amd64.buildinfo").unlink(missing_ok=True)
    Path("../open5gs_2.3.6_amd64.changes").unlink(missing_ok=True)
    Path("../open5gs_2.3.6.tar.xz").unlink(missing_ok=True)

    Path("../open5gs-debs").mkdir(exist_ok=True, parents=True)
    for deb in Path("../").glob("open5gs*.deb"):
        deb.replace(Path("../open5gs-debs") / deb.name)

def deploy_open5gs_5gc_packages(open5gs_package_directory, host):
    """ Deploys all open5gs packages to the indicated host
    """

    # Build the package list programatically to more easily update
    components = ["amf", "ausf", "bsf", "nrf", "nssf", "pcf", "smf", "udm", "udr", "upf"]
    version = "2.3.6"
    architecture = "amd64"

    # Explicitly include the common package, although it is not a core network component.
    packages = ["open5gs-dauth-common_{}_{}.deb".format(version, architecture)]

    for component in components:
        packages.append("open5gs-dauth-{}_{}_{}.deb".format(component, version, architecture))

    connection = Connection(host)
    for package in packages:
        deb_path = Path(open5gs_package_directory, package).absolute()
        log.info("Deploying deb: %s to host %s", deb_path, host)
        connection.put(deb_path, remote="/tmp/", preserve_mode=False)
        connection.sudo("dpkg --force-confnew --force-overwrite -i /tmp/{}".format(deb_path.name))


if __name__ == "__main__":
    try:
        import colorlog

        handler = colorlog.StreamHandler()
        handler.setFormatter(
            colorlog.ColoredFormatter(
                "%(log_color)s%(levelname)s(%(name)s): %(message)s"
            )
        )
        log = colorlog.getLogger(__name__)
        log.setLevel(logging.INFO)
        log.addHandler(handler)
    except Exception as e:
        logging.basicConfig(level=logging.INFO)
        log = logging.getLogger(__name__)
        log.info(
            "System does not support colored logging due to exception:", exc_info=True
        )
        log.info("Continuing operation with standard logging")

    build_dauth_manager(target="release")
    dauth_package_path = package_dauth_manager(target="release")

    deploy_package(dauth_package_path, "colte1.local")

    build_open5gs_packages()

    deploy_open5gs_5gc_packages(Path("../open5gs-debs"), "colte1.local")
