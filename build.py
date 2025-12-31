#!/usr/bin/env python3
"""
Cross-platform build script for SCP Label Maker
Automatically detects distro, installs dependencies, and builds for multiple targets
"""
import argparse
import os
import platform
import subprocess
import sys
import zipfile
import shutil
from pathlib import Path
from typing import List, Optional
try:
    from colorama import Fore, Style, init
    init(autoreset=True)
except ImportError:
    print("Installing colorama...")
    subprocess.run([sys.executable, "-m", "pip", "install", "colorama"], check=True)
    from colorama import Fore, Style, init
    init(autoreset=True)

class BuildConfig:
    """Build configuration"""
    def __init__(self):
        self.targets = []
        self.use_cross = False
        self.use_cargo_xwin = False
        self.strip_binaries = False
        self.package = False
        self.skip_deps = False
        self.clean = False

class DistroInfo:
    """Distro detection and package management"""
    def __init__(self):
        self.distro = self._detect_distro()
        self.pkg_manager = self._detect_package_manager()

    def _detect_distro(self) -> str:
        """Detect Linux distribution"""
        if platform.system() != "Linux":
            return "unknown"
        try:
            with open("/etc/os-release") as f:
                for line in f:
                    if line.startswith("ID="):
                        return line.split("=")[1].strip().strip('"')
        except:
            pass
        return "unknown"

    def _detect_package_manager(self) -> Optional[str]:
        """Detect package manager"""
        managers = {
            "pacman": ["arch", "manjaro", "endeavouros"],
            "apt": ["debian", "ubuntu", "mint", "pop"],
            "dnf": ["fedora", "rhel", "centos"],
            "zypper": ["opensuse", "suse"],
            "emerge": ["gentoo"],
        }
        for mgr, distros in managers.items():
            if self.distro in distros:
                return mgr
        for mgr in managers.keys():
            if self._command_exists(mgr):
                return mgr
        return None

    def _command_exists(self, cmd: str) -> bool:
        """Check if command exists"""
        return subprocess.run(
            ["which", cmd],
            stdout=subprocess.DEVNULL,
            stderr=subprocess.DEVNULL
        ).returncode == 0

class DependencyManager:
    """Manage system dependencies"""
    PACKAGES = {
        "pacman": {
            "mingw": "mingw-w64-gcc",
            "wayland": "wayland",
            "libxkbcommon": "libxkbcommon",
        },
        "apt": {
            "mingw": "mingw-w64",
            "wayland": "libwayland-dev",
            "libxkbcommon": "libxkbcommon-dev",
        },
        "dnf": {
            "mingw": "mingw64-gcc mingw32-gcc",
            "wayland": "wayland-devel",
            "libxkbcommon": "libxkbcommon-devel",
        },
        "zypper": {
            "mingw": "mingw64-cross-gcc",
            "wayland": "wayland-devel",
            "libxkbcommon": "libxkbcommon-devel",
        },
    }

    def __init__(self, distro_info: DistroInfo):
        self.distro_info = distro_info

    def check_rust(self) -> bool:
        """Check if Rust is installed"""
        return subprocess.run(
            ["rustc", "--version"],
            stdout=subprocess.DEVNULL,
            stderr=subprocess.DEVNULL
        ).returncode == 0

    def install_rust(self):
        """Install Rust via rustup"""
        print(f"{Fore.YELLOW}Installing Rust...")
        subprocess.run([
            "curl", "--proto", "=https", "--tlsv1.2", "-sSf",
            "https://sh.rustup.rs", "-o", "/tmp/rustup.sh"
        ], check=True)
        subprocess.run(["sh", "/tmp/rustup.sh", "-y"], check=True)
        print(f"{Fore.GREEN}✓ Rust installed")

    def check_cargo_tool(self, tool: str) -> bool:
        """Check if cargo tool is installed"""
        result = subprocess.run(
            ["cargo", tool, "--version"],
            stdout=subprocess.DEVNULL,
            stderr=subprocess.DEVNULL
        )
        return result.returncode == 0

    def install_cargo_tool(self, tool: str):
        """Install cargo tool"""
        print(f"{Fore.YELLOW}Installing {tool}...")
        subprocess.run(["cargo", "install", tool], check=True)
        print(f"{Fore.GREEN}✓ {tool} installed")

    def check_system_package(self, package_key: str) -> bool:
        """Check if system package is installed"""
        pkg_mgr = self.distro_info.pkg_manager
        if pkg_mgr not in self.PACKAGES:
            return False
        package = self.PACKAGES[pkg_mgr].get(package_key, "")
        if not package:
            return False
        if pkg_mgr == "pacman":
            result = subprocess.run(
                ["pacman", "-Qi", package.split()[0]],
                stdout=subprocess.DEVNULL,
                stderr=subprocess.DEVNULL
            )
        elif pkg_mgr == "apt":
            result = subprocess.run(
                ["dpkg", "-s", package.split()[0]],
                stdout=subprocess.DEVNULL,
                stderr=subprocess.DEVNULL
            )
        elif pkg_mgr in ["dnf", "zypper"]:
            result = subprocess.run(
                [pkg_mgr, "list", "installed", package.split()[0]],
                stdout=subprocess.DEVNULL,
                stderr=subprocess.DEVNULL
            )
        else:
            return False
        return result.returncode == 0

    def install_system_package(self, package_key: str):
        """Install system package"""
        pkg_mgr = self.distro_info.pkg_manager
        if pkg_mgr not in self.PACKAGES:
            print(f"{Fore.YELLOW}⚠ Unknown package manager, skipping {package_key}")
            return
        package = self.PACKAGES[pkg_mgr].get(package_key, "")
        if not package:
            print(f"{Fore.YELLOW}⚠ No package mapping for {package_key}")
            return
        print(f"{Fore.YELLOW}Installing {package}...")
        if pkg_mgr == "pacman":
            subprocess.run(["sudo", "pacman", "-S", "--noconfirm"] + package.split(), check=True)
        elif pkg_mgr == "apt":
            subprocess.run(["sudo", "apt", "install", "-y"] + package.split(), check=True)
        elif pkg_mgr == "dnf":
            subprocess.run(["sudo", "dnf", "install", "-y"] + package.split(), check=True)
        elif pkg_mgr == "zypper":
            subprocess.run(["sudo", "zypper", "install", "-y"] + package.split(), check=True)
        print(f"{Fore.GREEN}✓ {package} installed")

    def check_android_ndk(self) -> Optional[Path]:
        """Check if ANDROID_NDK_HOME is set and valid"""
        ndk_home = os.environ.get("ANDROID_NDK_HOME")
        if ndk_home and Path(ndk_home).is_dir():
            return Path(ndk_home)
        return None

    def setup_dependencies(self, config: BuildConfig):
        """Setup all required dependencies"""
        print(f"\n{Fore.CYAN}{'='*60}")
        print(f"{Fore.CYAN}Checking Dependencies")
        print(f"{Fore.CYAN}{'='*60}\n")
        print(f"{Fore.BLUE}Distro: {Fore.WHITE}{self.distro_info.distro}")
        print(f"{Fore.BLUE}Package Manager: {Fore.WHITE}{self.distro_info.pkg_manager or 'unknown'}\n")

        if not self.check_rust():
            print(f"{Fore.RED}✗ Rust not found")
            self.install_rust()
        else:
            print(f"{Fore.GREEN}✓ Rust installed")

        if config.use_cross:
            if not self.check_cargo_tool("cross"):
                self.install_cargo_tool("cross")
            else:
                print(f"{Fore.GREEN}✓ cross installed")

        if config.use_cargo_xwin:
            if not self.check_cargo_tool("cargo-xwin"):
                self.install_cargo_tool("cargo-xwin")
            else:
                print(f"{Fore.GREEN}✓ cargo-xwin installed")
            if not self.check_cargo_tool("xwin"):
                self.install_cargo_tool("xwin")
            print(f"{Fore.YELLOW}Setting up Windows SDK...")
            subprocess.run([
                "xwin", "--accept-license", "splat",
                "--output", str(Path.home() / ".xwin")
            ], check=True)
        else:
            print(f"{Fore.GREEN}✓ xwin installed")

        if any("windows" in t for t in config.targets) and not config.use_cargo_xwin:
            if not self.check_system_package("mingw"):
                self.install_system_package("mingw")
            else:
                print(f"{Fore.GREEN}✓ MinGW installed")

        if any("linux" in t for t in config.targets):
            if not self.check_system_package("wayland"):
                print(f"{Fore.YELLOW}⚠ Wayland not found (optional)")
            else:
                print(f"{Fore.GREEN}✓ Wayland installed")
            if not self.check_system_package("libxkbcommon"):
                print(f"{Fore.YELLOW}⚠ libxkbcommon not found (optional)")
            else:
                print(f"{Fore.GREEN}✓ libxkbcommon installed")

        if any("android" in t for t in config.targets):
            if not self.check_android_ndk():
                print(f"{Fore.RED}✗ ANDROID_NDK_HOME not set or invalid")
                sys.exit(1)
            else:
                print(f"{Fore.GREEN}✓ Android NDK detected")

class RustBuilder:
    """Handle Rust compilation"""
    TARGET_MAP = {
        "linux-x64": "x86_64-unknown-linux-gnu",
        "linux-x64-musl": "x86_64-unknown-linux-musl",
        "linux-x86": "i686-unknown-linux-gnu",
        "linux-x86-musl": "i686-unknown-linux-musl",
        "windows-x64": "x86_64-pc-windows-gnu",
        "windows-x64-msvc": "x86_64-pc-windows-msvc",
        "windows-x86": "i686-pc-windows-gnu",
        "windows-x86-msvc": "i686-pc-windows-msvc",
        "linux-arm64": "aarch64-unknown-linux-gnu",
        "linux-arm64-musl": "aarch64-unknown-linux-musl",
        "linux-armv7": "armv7-unknown-linux-gnueabihf",
        "linux-armv7-musl": "armv7-unknown-linux-musleabihf",
        "android-arm64": "aarch64-linux-android",
        "android-armv7": "armv7-linux-androideabi",
    }

    def __init__(self, config: BuildConfig):
        self.config = config
        self._host_triple = self._get_host_triple()

    def _get_host_triple(self) -> str:
        """Get the host rustc target triple"""
        try:
            output = subprocess.check_output(['rustc', '-vV'], text=True, stderr=subprocess.DEVNULL)
            for line in output.splitlines():
                if line.startswith('host:'):
                    return line.split(' ')[1].strip()
        except (subprocess.CalledProcessError, FileNotFoundError, IndexError):
            pass
        return "unknown"

    def add_rust_targets(self, targets: List[str]):
        """Add Rust targets"""
        print(f"\n{Fore.CYAN}{'='*60}")
        print(f"{Fore.CYAN}Adding Rust Targets")
        print(f"{Fore.CYAN}{'='*60}\n")
        for target in targets:
            rust_target = self.TARGET_MAP.get(target)
            if not rust_target:
                print(f"{Fore.RED}✗ Unknown target: {target}")
                continue
            print(f"{Fore.YELLOW}Adding target {rust_target}...")
            result = subprocess.run(
                ["rustup", "target", "add", rust_target],
                capture_output=True,
                text=True
            )
            if "up to date" in result.stdout or result.returncode == 0:
                print(f"{Fore.GREEN}✓ {rust_target}")
            else:
                print(f"{Fore.RED}✗ Failed to add {rust_target}")

    def clean(self):
        """Clean build artifacts"""
        print(f"\n{Fore.CYAN}{'='*60}")
        print(f"{Fore.CYAN}Cleaning Build Artifacts")
        print(f"{Fore.CYAN}{'='*60}\n")
        print(f"{Fore.YELLOW}Running cargo clean...")
        subprocess.run(["cargo", "clean"], check=True)
        print(f"{Fore.GREEN}✓ Clean complete")

    def build_target(self, target: str):
        """Build for a specific target"""
        rust_target = self.TARGET_MAP.get(target)
        if not rust_target:
            print(f"{Fore.RED}✗ Unknown target: {target}")
            return False

        print(f"\n{Fore.CYAN}{'='*60}")
        print(f"{Fore.CYAN}Building {target}")
        print(f"{Fore.CYAN}{'='*60}\n")

        env = os.environ.copy()
        if "android" in target:
            env = self._android_env(rust_target)

        cmd = ["cargo", "build", "--release", "--target", rust_target]
        if self.config.use_cargo_xwin and "msvc" in target:
            cmd = ["cargo", "xwin", "build", "--release", "--target", rust_target]
        elif self.config.use_cross:
            cmd = ["cross", "build", "--release", "--target", rust_target]

        print(f"{Fore.BLUE}Command: {' '.join(cmd)}\n")

        result = subprocess.run(cmd, env=env)
        if result.returncode == 0:
            print(f"\n{Fore.GREEN}✓ Build successful for {target}")
            if self.config.strip_binaries and ("linux" in target or "android" in target):
                binary_path = self._get_binary_path(rust_target)
                if binary_path and binary_path.exists():
                    print(f"{Fore.YELLOW}Stripping binary...")
                    subprocess.run(["strip", str(binary_path)])
                    print(f"{Fore.GREEN}✓ Binary stripped")
            return True
        else:
            print(f"\n{Fore.RED}✗ Build failed for {target}")
            return False

    def _android_env(self, rust_target: str) -> dict:
        """Setup Android NDK environment variables"""
        ndk_home = os.environ.get("ANDROID_NDK_HOME")
        if not ndk_home:
            print(f"{Fore.RED}ANDROID_NDK_HOME is not set for Android build!")
            sys.exit(1)
        ndk_path = Path(ndk_home)
        host_tag = "linux-x86_64"
        toolchain_bin = ndk_path / "toolchains/llvm/prebuilt" / host_tag / "bin"
        env = os.environ.copy()
        if rust_target == "aarch64-linux-android":
            api_level = "21"
            env["CC"] = str(toolchain_bin / f"aarch64-linux-android{api_level}-clang")
            env["CXX"] = str(toolchain_bin / f"aarch64-linux-android{api_level}-clang++")
            env[f"CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER"] = env["CC"]
        elif rust_target == "armv7-linux-androideabi":
            api_level = "21"
            env["CC"] = str(toolchain_bin / f"armv7a-linux-androideabi{api_level}-clang")
            env["CXX"] = str(toolchain_bin / f"armv7a-linux-androideabi{api_level}-clang++")
            env[f"CARGO_TARGET_ARMV7_LINUX_ANDROEABI_LINKER"] = env["CC"]
        else:
            print(f"{Fore.YELLOW}Warning: Unknown Android target {rust_target}, no specific NDK setup applied.")
        env["AR"] = str(toolchain_bin / "llvm-ar")
        env["RANLIB"] = str(toolchain_bin / "llvm-ranlib")
        return env

    def _get_binary_path(self, rust_target: str) -> Optional[Path]:
        """Get path to built binary"""
        base = Path("target") / rust_target / "release"

        if "windows" in rust_target:
            return base / "scp-label-maker.exe"
        else:
            return base / "scp-label-maker"

    def package_builds(self):
        """Package built binaries into zip archives."""
        print(f"\n{Fore.CYAN}{'='*60}")
        print(f"{Fore.CYAN}Packaging Builds")
        print(f"{Fore.CYAN}{'='*60}\n")
        dist_dir = Path("dist")
        dist_dir.mkdir(exist_ok=True)
        assets_dir = Path("assets")
        resources_dir = Path("resources")

        for item in dist_dir.iterdir():
            if item.is_file():
                item.unlink()
            elif item.is_dir():
                shutil.rmtree(item)

        for target in self.config.targets:
            rust_target = self.TARGET_MAP.get(target)
            if not rust_target:
                continue
            binary_path = self._get_binary_path(rust_target)
            if not binary_path or not binary_path.exists():
                print(f"{Fore.YELLOW}⚠ Binary for {target} not found, skipping packaging.")
                continue

            package_name = f"scp-label-maker-{target}"
            zip_path = dist_dir / f"{package_name}.zip"

            print(f"Creating zip archive for {target} at {zip_path}...")

            with zipfile.ZipFile(zip_path, 'w', zipfile.ZIP_DEFLATED) as zipf:
                zipf.write(binary_path, binary_path.name)

                if assets_dir.is_dir():
                    for root, _, files in os.walk(assets_dir):
                        for file in files:
                            file_path = Path(root) / file
                            archive_path = file_path.relative_to(Path('.'))
                            zipf.write(file_path, archive_path)
                
                if resources_dir.is_dir():
                    for root, _, files in os.walk(resources_dir):
                        for file in files:
                            file_path = Path(root) / file
                            archive_path = file_path.relative_to(Path('.'))
                            zipf.write(file_path, archive_path)

            print(f"{Fore.GREEN}✓ Packaged {target} -> {zip_path}")
        print(f"\n{Fore.GREEN}All builds packaged in {dist_dir}/")

def main():
    parser = argparse.ArgumentParser(
        description="Cross-platform build script for SCP Label Maker",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  %(prog)s --all # Build for all targets
  %(prog)s -t linux-x64 windows-x64 # Build specific targets
  %(prog)s --all --use-cross # Use cross for all builds
  %(prog)s --all --use-cargo-xwin # Use cargo-xwin for MSVC targets
  %(prog)s --all --strip --package # Strip and package binaries
  %(prog)s --clean # Clean build artifacts
  %(prog)s --skip-deps -t linux-x64 # Skip dependency checks
Available targets:
  linux-x64, linux-x64-musl, linux-x86, linux-x86-musl
  linux-arm64, linux-arm64-musl, linux-armv7, linux-armv7-musl
  windows-x64, windows-x64-msvc, windows-x86, windows-x86-msvc
  android-arm64, android-armv7
        """
    )
    parser.add_argument(
        "-t", "--targets",
        nargs="+",
        choices=list(RustBuilder.TARGET_MAP.keys()),
        help="Specific targets to build"
    )
    parser.add_argument(
        "--all",
        action="store_true",
        help="Build for all targets"
    )
    parser.add_argument(
        "--use-cross",
        action="store_true",
        help="Use 'cross' for compilation (recommended)"
    )
    parser.add_argument(
        "--use-cargo-xwin",
        action="store_true",
        help="Use 'cargo-xwin' for MSVC targets (static Windows builds)"
    )
    parser.add_argument(
        "--strip",
        action="store_true",
        help="Strip binaries to reduce size"
    )
    parser.add_argument(
        "--package",
        action="store_true",
        help="Package binaries into dist/ folder"
    )
    parser.add_argument(
        "--skip-deps",
        action="store_true",
        help="Skip dependency installation"
    )
    parser.add_argument(
        "--clean",
        action="store_true",
        help="Clean build artifacts"
    )
    parser.add_argument(
        "--list-targets",
        action="store_true",
        help="List all available targets"
    )
    args = parser.parse_args()

    if args.list_targets:
        print(f"\n{Fore.CYAN}Available targets:")
        for target in RustBuilder.TARGET_MAP.keys():
            print(f" {Fore.GREEN}• {Fore.WHITE}{target}")
        print()
        return

    config = BuildConfig()
    config.use_cross = args.use_cross
    config.use_cargo_xwin = args.use_cargo_xwin
    config.strip_binaries = args.strip
    config.package = args.package
    config.skip_deps = args.skip_deps
    config.clean = args.clean

    if args.all:
        config.targets = list(RustBuilder.TARGET_MAP.keys())
    elif args.targets:
        config.targets = args.targets
    elif not args.clean:
        print(f"{Fore.RED}Error: Specify --all or -t <targets>")
        parser.print_help()
        sys.exit(1)

    print(f"\n{Fore.MAGENTA}{'='*60}")
    print(f"{Fore.MAGENTA} SCP Label Maker - Build Script")
    print(f"{Fore.MAGENTA}{'='*60}\n")

    if config.clean:
        builder = RustBuilder(config)
        builder.clean()
        if not config.targets:
            return

    if not config.skip_deps:
        distro_info = DistroInfo()
        dep_manager = DependencyManager(distro_info)
        dep_manager.setup_dependencies(config)

    builder = RustBuilder(config)
    builder.add_rust_targets(config.targets)

    success_count = 0
    fail_count = 0
    for target in config.targets:
        if builder.build_target(target):
            success_count += 1
        else:
            fail_count += 1

    if config.package and success_count > 0:
        builder.package_builds()

    print(f"\n{Fore.CYAN}{'='*60}")
    print(f"{Fore.CYAN}Build Summary")
    print(f"{Fore.CYAN}{'='*60}\n")
    print(f"{Fore.GREEN}Successful: {success_count}")
    print(f"{Fore.RED}Failed: {fail_count}")
    print()

    if fail_count > 0:
        sys.exit(1)

if __name__ == "__main__":
    main()