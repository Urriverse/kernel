#!/usr/bin/env python3

import os
import subprocess
import sys
import termios
import tomllib
import tty
from pathlib import Path

GREEN   = "\033[0;32m"
BOLD    = "\033[1m"
RED     = "\033[0;31m"
NC      = "\033[0m"
CLEAR   = "\033[H\033[2J"
INVERSE = "\033[7m"

SCRIPT_DIR   = Path(__file__).resolve().parent          # etc/bin/
PROJECT_ROOT = SCRIPT_DIR.parent.parent                 # kernel/
PROFILES_DIR = PROJECT_ROOT / "etc" / "profiles"
DO_SCRIPT    = SCRIPT_DIR / "do"                        # etc/bin/do


def load_profiles() -> list[dict]:
    if not PROFILES_DIR.is_dir():
        print(f"{RED}error:{NC} {PROFILES_DIR} not found.", file=sys.stderr)
        sys.exit(1)

    profiles = []
    for entry in sorted(PROFILES_DIR.iterdir()):
        if entry.suffix != ".toml":
            continue
        name = entry.stem
        desc = ""
        try:
            with entry.open("rb") as f:
                data = tomllib.load(f)
            desc = data.get("profile", {}).get("description", "").strip()
        except Exception:
            desc = "(invalid TOML)"
        profiles.append({"name": name, "description": desc})

    if not profiles:
        print(f"{RED}error:{NC} no profiles in {PROFILES_DIR}.", file=sys.stderr)
        sys.exit(1)
    return profiles


class RawTerminal:
    def __enter__(self):
        self.fd = sys.stdin.fileno()
        self.old = termios.tcgetattr(self.fd)
        tty.setraw(self.fd)
        return self

    def __exit__(self, *args):
        termios.tcsetattr(self.fd, termios.TCSADRAIN, self.old)

    def get_key(self) -> str:
        c = sys.stdin.read(1)
        if c == '\x1b':                     # Escape sequence
            n = sys.stdin.read(1)
            if n == '[':
                m = sys.stdin.read(1)
                if m == 'A': return 'UP'
                if m == 'B': return 'DOWN'
            return c
        if c in ('\r', '\n'):
            return 'ENTER'
        return c


def draw_menu(profiles: list[dict], selected: int):
    lines = [
        CLEAR,
        "\r\n",
        "\r\n",
        "\r\n",
        f"  {GREEN}Select kernel build profile:{NC}\r\n",
        "\r\n",
    ]
    for i, p in enumerate(profiles):
        if i == selected:
            prefix = f"  {INVERSE}>"
        else:
            prefix = "   "
        name = f"{BOLD}{p['name']:<12}{NC}"
        desc = p['description']
        lines.append(f"{prefix} {name}  {desc}\r\n")
    lines.append("\r\n  ↑/↓: navigate   Enter: confirm   Q: quit\r\n")

    sys.stdout.write("".join(lines))
    sys.stdout.flush()


def interactive_select(profiles: list[dict]) -> str:
    idx = 0
    try:
        with RawTerminal() as term:
            draw_menu(profiles, idx)
            while True:
                key = term.get_key()
                if key == 'UP':
                    idx = (idx - 1) % len(profiles)
                    draw_menu(profiles, idx)
                elif key == 'DOWN':
                    idx = (idx + 1) % len(profiles)
                    draw_menu(profiles, idx)
                elif key == 'ENTER':
                    break
                elif key.lower() == 'q':
                    sys.stdout.write(CLEAR + "Aborted.\r\n")
                    sys.stdout.flush()
                    sys.exit(130)
    except Exception:
        # Restore terminal before propagating the error
        sys.stdout.write("\r\n")
        raise
    return profiles[idx]["name"]


def main():
    if not DO_SCRIPT.is_file() or not os.access(DO_SCRIPT, os.X_OK):
        print(
            f"{RED}error:{NC} cannot execute `{DO_SCRIPT}`.\n"
            f"Please ensure it exists and has the executable bit set (chmod +x etc/bin/do).",
            file=sys.stderr,
        )
        sys.exit(1)

    profiles = load_profiles()
    chosen = interactive_select(profiles)

    sys.stdout.write(CLEAR)
    sys.stdout.flush()

    cmd = [str(DO_SCRIPT), "config", chosen]
    try:
        subprocess.run(cmd, check=True)
    except subprocess.CalledProcessError as e:
        print(f"{RED}error:{NC} `do config` failed with exit code {e.returncode}",
              file=sys.stderr)
        sys.exit(e.returncode)


if __name__ == "__main__":
    main()
