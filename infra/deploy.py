import argparse
import logging
import os
import shutil
import subprocess

from pathlib import Path

from fabric import Connection

log = logging.getLogger(__name__)

hosts = "colte1.local, colte2.local"


def build_dauth_services(target):
    """Build rust services from source via cargo"""
    if target == "debug":
        cmd = ["cargo", "build"]
    elif target == "release":
        cmd = ["cargo", "build", "--release"]
    else:
        raise ValueError("Invalid target mode: {}".format(target))

    log.info("Running build command: %s", cmd)
    subprocess.run(cmd, check=True, cwd="../services")


def package_dauth_service(target, package_name="dauth_0.1.0~dev_amd64.deb"):
    """Package the dauth service per external nfpm.yaml config file"""
    with open("../services/nfpm-dauth.yaml") as f:
        nfpm_config = f.read()

    # Update the config file TARGET placeholder with the appropriate target
    nfpm_config = nfpm_config.replace(r"${TARGET}", target)
    log.debug("Running nfpm with config: \n %s", nfpm_config)

    subprocess.run(
        [
            "nfpm",
            "package",
            "--config",
            "/dev/stdin",
            "--packager",
            "deb",
            "--target",
            package_name,
        ],
        check=True,
        cwd="../services",
        input=nfpm_config.encode("utf8"),
    )

    package_path = Path("../services", package_name)
    log.info("Package created at: %s", package_path.absolute())
    return package_path


def package_dauth_directory_service(
    target, package_name="dauth-directory_0.1.0~dev_amd64.deb"
):
    """Package the dauth directory service per external nfpm.yaml config file"""
    with open("../services/nfpm-directory.yaml") as f:
        nfpm_config = f.read()

    # Update the config file TARGET placeholder with the appropriate target
    nfpm_config = nfpm_config.replace(r"${TARGET}", target)
    log.debug("Running nfpm with config: \n %s", nfpm_config)

    subprocess.run(
        [
            "nfpm",
            "package",
            "--config",
            "/dev/stdin",
            "--packager",
            "deb",
            "--target",
            package_name,
        ],
        check=True,
        cwd="../services",
        input=nfpm_config.encode("utf8"),
    )

    package_path = Path("../services", package_name)
    log.info("Package created at: %s", package_path.absolute())
    return package_path


def deploy_package(package_path, host):
    """Transfer and install the provided package on the host"""

    package_name = package_path.name

    result = Connection(host).put(package_path, remote="/tmp/", preserve_mode=False)
    result = Connection(host).sudo(
        "dpkg --force-confdef --force-confold -i /tmp/{}".format(package_name)
    )


def setup_open5gs_meson_directory():
    """Run the meson build configure step, which will fetch external dependencies"""
    subprocess.run("meson setup ./build", shell=True, check=True, cwd="../open5gs")


def build_open5gs_packages(fast_build=False):
    """Builds our open5gs deb packages from source via dpkg-buildpkg"""
    command = [
        "dpkg-buildpackage",
        "-us",
        "-uc",
        "--build=binary",
        "--compression-level=1",
        "--compression=gzip",
    ]

    if fast_build:
        command += ["--no-pre-clean", "--no-post-clean"]

    subprocess.run(command, check=True, cwd="../open5gs")

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
    """Deploys all open5gs packages to the indicated host"""

    # Build the package list programatically to more easily update
    components = [
        "amf",
        "ausf",
        "bsf",
        "nrf",
        "nssf",
        "pcf",
        "smf",
        "udm",
        "udr",
        "upf",
        "sgwc",
        "sgwu",
        "hss",
        "mme",
        "pcrf",
    ]
    version = "2.4.7"
    architecture = "amd64"

    # Explicitly include the common package, although it is not a core network component.
    packages = ["open5gs-dauth-common_{}_{}.deb".format(version, architecture)]

    for component in components:
        packages.append(
            "open5gs-dauth-{}_{}_{}.deb".format(component, version, architecture)
        )

    connection = Connection(host)
    for package in packages:
        deb_path = Path(open5gs_package_directory, package).absolute()
        log.info("Deploying deb: %s to host %s", deb_path, host)
        connection.put(deb_path, remote="/tmp/", preserve_mode=False)
        connection.sudo(
            "dpkg --force-confnew --force-overwrite -i /tmp/{}".format(deb_path.name)
        )

def build_ueransim(fast_build=False):
    """Builds our custom ueransim binaries from source"""

    if fast_build:
        release_type = "-DCMAKE_BUILD_TYPE=Debug"
        release_dir = "cmake-build-debug"
    else:
        release_type = "-DCMAKE_BUILD_TYPE=Release"
        release_dir = "cmake-build-release"

    cmake_configure_command = [
        "cmake",
        release_type,
        "-G",
        "Ninja",
        "-B",
        release_dir,
    ]

    subprocess.run(cmake_configure_command, check=True, cwd="../ueransim")

    cmake_build_command = [
        "cmake",
        "--build",
        release_dir,
        "--target",
        "all",
    ]

    subprocess.run(cmake_build_command, check=False, cwd="../ueransim")

    output_directory = Path("../ueransim-bin")
    output_directory.mkdir(exist_ok=True, parents=True)
    for component in ["nr-ue", "nr-gnb"]:
        log.info(f"Copying ueransim binary {component} to {output_directory}")
        shutil.copy(
            Path("../ueransim") / Path(release_dir) / Path(component),
            output_directory / Path(component)
            )

def deploy_ueransim(ueransim_binary_directory, host):
    """Deploys all ueransim binaries to the indicated host"""

    # Build the package list programatically to more easily update
    components = ["nr-ue", "nr-gnb"]

    connection = Connection(host)
    for component in components:
        binary_path = Path(ueransim_binary_directory, component).absolute()
        log.info("Deploying binary: %s to host %s", binary_path, host)

        if '@' in host:
            name, ip = host.split("@", 2)
            connection.run(f"mkdir -p /home/{name}/ueransim/")
            connection.put(binary_path, remote=f"/home/{name}/ueransim/", preserve_mode=False)
            connection.run(f"chmod a+x /home/{name}/ueransim/{component}")
        else:
            connection.run("mkdir -p /home/vagrant/ueransim/")
            connection.put(binary_path, remote="/home/vagrant/ueransim/", preserve_mode=False)
            connection.run(f"chmod a+x /home/vagrant/ueransim/{component}")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="deploy dauth in a test environment")
    parser.add_argument(
        "-b",
        "--build-dauth",
        action="store_true",
        help="Build dauth from source",
    )
    parser.add_argument(
        "--build-open5gs",
        action="store_true",
        help="Build open5gs from source",
    )
    parser.add_argument(
        "--build-ueransim",
        action="store_true",
        help="Build ueransim from source",
    )

    parser.add_argument(
        "-d",
        "--deploy-dauth",
        action="store_true",
        help="Deploy an already built version of dauth",
    )
    parser.add_argument(
        "--deploy-dauth-directory",
        action="store_true",
        help="Deploy the dauth directory service",
    )
    parser.add_argument(
        "--deploy-open5gs",
        action="store_true",
        help="Deploy an already built version of open5gs",
    )
    parser.add_argument(
        "--deploy-ueransim",
        action="store_true",
        help="Deploy the built ueransim binaries",
    )

    parser.add_argument(
        "-o",
        "--dest-host",
        action="append",
        default=[],
        help="Specify a hostname to deploy onto",
    )

    parser.add_argument(
        "-f",
        "--fast-debug",
        action="store_true",
        help="Do a less peformant but faster to complete debug build",
    )

    parser.add_argument(
        "-v",
        "--verbose",
        action="store_true",
        help="Enable verbose output",
    )

    args = parser.parse_args()

    try:
        import colorlog

        handler = colorlog.StreamHandler()
        handler.setFormatter(
            colorlog.ColoredFormatter(
                "%(log_color)s%(levelname)s(%(name)s)%(reset)s: %(message)s"
            )
        )
        log = colorlog.getLogger(__name__)
        if args.verbose:
            log.setLevel(logging.DEBUG)
        else:
            log.setLevel(logging.INFO)
        log.addHandler(handler)
    except Exception as e:
        if args.verbose:
            logging.basicConfig(level=logging.DEBUG)
        else:
            logging.basicConfig(level=logging.INFO)

        log = logging.getLogger(__name__)
        log.info(
            "System does not support colored logging due to exception:", exc_info=True
        )
        log.info("Continuing operation with standard logging")

    log.debug("Proceeding with args: %s", args)

    if not (
        args.build_dauth
        or args.build_open5gs
        or args.deploy_dauth
        or args.deploy_open5gs
        or args.deploy_dauth_directory
        or args.build_ueransim
        or args.deploy_ueransim
    ):
        log.error("No action specified!")

    if args.deploy_dauth_directory and len(args.dest_host) > 1:
        log.error("Cannot deploy the directory when multiple hosts are specified")
        raise NotImplementedError(
            "No way to differentiate which host receives the directory and which do not."
        )

    # Default values
    dauth_package_path = None
    dauth_directory_path = None

    if args.fast_debug:
        cargo_target = "debug"
        log.warn("Doing a debug build! Don't use for any performance testing")
        open5gs_fast_unclean_build = True
        log.warn(
            "Building incrementally and may contain state from previous builds if unclean"
        )
    else:
        cargo_target = "release"
        open5gs_fast_unclean_build = False

    if args.build_dauth:
        log.info("Building dauth")
        build_dauth_services(target=cargo_target)
        log.info("Building dauth packages")
        dauth_package_path = package_dauth_service(target=cargo_target)
        dauth_directory_path = package_dauth_directory_service(target=cargo_target)

    if args.build_open5gs:
        log.info("Building open5gs")
        log.info("Configuring the meson build")
        setup_open5gs_meson_directory()
        log.warning("Building and packaging open5gs may take a while : /")
        build_open5gs_packages(fast_build=open5gs_fast_unclean_build)

    if args.build_ueransim:
        log.info("Building ueransim")
        build_ueransim(fast_build=args.fast_debug)

    if args.deploy_dauth:
        if dauth_package_path is None:
            log.info("Building dauth package")
            dauth_package_path = package_dauth_service(target=cargo_target)
        log.info("Deploying dauth package")
        if len(args.dest_host) == 0:
            log.error("Specified deploy but no deploy destinations provided")
        for host in args.dest_host:
            deploy_package(dauth_package_path, host)

    if args.deploy_dauth_directory:
        if dauth_directory_path is None:
            log.info("Building dauth directory package")
            dauth_directory_path = package_dauth_directory_service(target=cargo_target)
        log.info("Deploying dauth directory package")
        if len(args.dest_host) == 0:
            log.error("Specified deploy but no deploy destinations provided")
        assert len(args.dest_host) == 1
        deploy_package(dauth_directory_path, args.dest_host[0])

    if args.deploy_open5gs:
        log.info("Deploying open5gs packages")
        if len(args.dest_host) == 0:
            log.error("Specified deploy but no deploy destinations provided")
        for host in args.dest_host:
            deploy_open5gs_5gc_packages(Path("../open5gs-debs"), host)

            if '@' in host:
                name, ip = host.split("@", 2)
                Connection(host).sudo(f"/home/{name}/scripts/open5gs-ip-config.py {ip}")
            else:
                Connection(host).sudo("/home/vagrant/scripts/open5gs-ip-config.py")


    if args.build_ueransim:
        log.info("Deploying ueransim")
        if len(args.dest_host) == 0:
            log.error("Specified deploy but no deploy destinations provided")
        for host in args.dest_host:
            deploy_ueransim("../ueransim-bin", host)
