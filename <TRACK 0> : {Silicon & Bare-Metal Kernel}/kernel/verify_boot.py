#!/usr/bin/env python3
"""QEMU boot regressions: full RAM integrity and actual hosted fallback."""

# BLOCK 1: METADATA — host-only tests; no learning or AI runtime
import argparse
import json
from pathlib import Path
import queue
import re
import shutil
import subprocess
import sys
import tempfile
import threading
import time

from host_boot import READY, next_rung, qemu_command

# BLOCK 2: OPENING — bounded QMP sessions and firmware discovery
KERNEL = Path(__file__).resolve().parent
TRACK4 = (KERNEL.parent.parent /
          "<TRACK 4> : {Variable Storage & Episodic Memory}" /
          "scripture/indexes/parallel/ordinal/parallel-ordinal-index.bin")


def firmware():
    for name in ("/usr/share/ovmf/OVMF.fd", "/usr/share/qemu/OVMF.fd",
                 "/usr/share/OVMF/OVMF_CODE_4M.fd"):
        if Path(name).is_file():
            return Path(name)
    return None


class Guest:
    def __init__(self, command, log, scratch):
        self.log = log
        self.scratch = scratch
        # Serial goes to its own file; QMP exclusively owns stdio.
        command = command[:-2] + ["-serial", f"file:{log}", "-qmp", "stdio", "-S"]
        self.errors = (scratch / "qemu.stderr").open("w")
        self.process = subprocess.Popen(
            command, stdin=subprocess.PIPE, stdout=subprocess.PIPE,
            stderr=self.errors, text=True,
        )
        self.events = queue.Queue()

        def receive():
            for line in self.process.stdout:
                self.events.put(json.loads(line))
            self.events.put(None)

        self.reader = threading.Thread(target=receive, daemon=True)
        self.reader.start()
        try:
            greeting = self.events.get(timeout=5)
            if not greeting or "QMP" not in greeting:
                raise RuntimeError(f"Missing QMP greeting: {greeting!r}; " +
                                   (scratch / "qemu.stderr").read_text())
            self.command("qmp_capabilities")
            self.command("cont")
        except BaseException:
            self.close()
            raise

    def command(self, name, arguments=None):
        request = {"execute": name}
        if arguments:
            request["arguments"] = arguments
        self.process.stdin.write(json.dumps(request) + "\n")
        self.process.stdin.flush()
        deadline = time.monotonic() + 5
        while True:
            event = self.events.get(timeout=max(.01, deadline - time.monotonic()))
            if event is None:
                raise RuntimeError("QEMU exited before QMP response: " +
                                   (self.scratch / "qemu.stderr").read_text())
            if "error" in event:
                raise RuntimeError(str(event["error"]))
            if "return" in event:
                return event["return"]
            if time.monotonic() > deadline:
                raise RuntimeError("QMP response timed out")

    def await_marker(self, marker):
        deadline = time.monotonic() + 15
        while time.monotonic() < deadline:
            text = self.log.read_text(errors="replace") if self.log.exists() else ""
            if marker in text:
                return
            if self.process.poll() is not None:
                break
            time.sleep(.03)
        raise RuntimeError(f"Missing boot marker {marker!r}; inspect {self.log}")

    def read_memory(self, address, size):
        path = self.scratch / "memory.bin"
        result = self.command("human-monitor-command", {
            "command-line": f'pmemsave {address:#x} {size} "{path}"',
        })
        if result:
            raise RuntimeError(f"QMP memory dump failed: {result}")
        return path.read_bytes()

    def close(self):
        if self.process.poll() is None:
            self.process.terminate()
            try:
                self.process.wait(timeout=3)
            except subprocess.TimeoutExpired:
                self.process.kill()
                self.process.wait()
        self.reader.join(timeout=2)
        self.process.stdin.close()
        self.process.stdout.close()
        self.errors.close()


# BLOCK 3: BODY — success markers alone cannot certify memory or transitions
def check_table(actual, baseline):
    if actual != baseline:
        changed = sum(a != b for a, b in zip(actual, baseline))
        raise RuntimeError(f"Track 4 integrity failed: {changed} changed bytes, "
                           f"length {len(actual)} !=/== {len(baseline)}")


def normal_boot(images, rung, ovmf, log, scratch):
    command = qemu_command("qemu-system-x86_64", images, rung, ovmf)
    guest = Guest(command, log, scratch)
    try:
        guest.await_marker(READY[rung])
        guest.command("stop")
        if rung != "bios":
            baseline = TRACK4.read_bytes()
            check_table(guest.read_memory(0x20000, len(baseline)), baseline)
            address = {"stage2": 0x18000, "multiboot": 0x108000,
                       "uefi": 0x200000}[rung]
            count = int.from_bytes(guest.read_memory(address, 4), "little")
            if not 1 <= count <= 64:
                raise RuntimeError(f"Invalid {rung} recorder count: {count}")
            if rung == "multiboot":
                # Also prove the original embedded source remains pristine.
                symbols = subprocess.check_output([
                    "nm", "-n", str(images / "omnicode_multiboot.elf")
                ], text=True)
                match = re.search(r"^([0-9a-f]+) r track4_table_start$",
                                  symbols, re.MULTILINE)
                if not match:
                    raise RuntimeError("Missing embedded table symbol")
                check_table(guest.read_memory(int(match[1], 16), len(baseline)),
                            baseline)
        print(f"PASS {rung}: boot" +
              (", full table and flight recorder verified" if rung != "bios" else ""),
              flush=True)
    finally:
        guest.close()


def cascade_artifacts(target):
    """Fault variants live only in the audit's temporary directory."""
    (target / "esp/EFI/BOOT").mkdir(parents=True)
    subprocess.run([
        "clang", "-target", "x86_64-unknown-windows", "-fuse-ld=lld-link",
        "-nostdlib", "-Wl,-subsystem:efi_application", "-Wl,-entry:EfiMain",
        "-DUEFI_TRIPWIRE_SIMULATION=1", str(KERNEL / "uefi/main.c"),
        "-o", str(target / "esp/EFI/BOOT/BOOTX64.EFI"),
    ], check=True)
    obj = target / "multiboot.o"
    subprocess.run([
        "nasm", "-DCASCADE_SIMULATION=1", "-f", "elf32",
        "-I" + str(TRACK4.parent) + "/", str(KERNEL / "multiboot/boot.asm"),
        "-o", str(obj),
    ], check=True)
    subprocess.run([
        "ld", "-m", "elf_i386", "-T", str(KERNEL / "multiboot/linker.ld"),
        str(obj), "-o", str(target / "omnicode_multiboot.elf"),
    ], check=True)
    stage2 = target / "stage2.bin"
    subprocess.run([
        "nasm", "-DCASCADE_SIMULATION=1", "-f", "bin",
        str(KERNEL / "bios/stage2/stage2.asm"), "-o", str(stage2),
    ], check=True)
    mbr = (KERNEL / "images/omnicode_bios.img").read_bytes()
    payload = stage2.read_bytes()
    if len(payload) > 2048:
        raise RuntimeError("Cascade Stage 2 exceeds sector allocation")
    table = TRACK4.read_bytes()
    (target / "omnicode_bios.img").write_bytes(mbr)
    (target / "omnicode_bios_dual.img").write_bytes(
        mbr + payload.ljust(2048, b"\0") + table.ljust(122 * 512, b"\0"))


def verify(images):
    logs = images / "logs"
    logs.mkdir(parents=True, exist_ok=True)
    ovmf = firmware()
    with tempfile.TemporaryDirectory(prefix="omnicode-audit-") as temporary:
        scratch = Path(temporary)
        for rung in ("bios", "stage2", "multiboot", "uefi"):
            if rung == "uefi":
                if ovmf is None:
                    print("SKIP UEFI and full cascade: OVMF firmware unavailable")
                    continue
                staged = scratch / "normal"
                (staged / "esp/EFI/BOOT").mkdir(parents=True)
                shutil.copyfile(images / "BOOTX64.EFI",
                                staged / "esp/EFI/BOOT/BOOTX64.EFI")
            else:
                staged = images
            normal_boot(staged, rung, ovmf, logs / f"{rung}.log", scratch)

        # Negative regression: a correct header/anchor must not conceal damage.
        corrupted = bytearray(TRACK4.read_bytes())
        corrupted[0x5B60] ^= 1
        try:
            check_table(corrupted, TRACK4.read_bytes())
        except RuntimeError:
            print("PASS integrity oracle rejects corruption beyond header")
        else:
            raise RuntimeError("Integrity oracle accepted table corruption")

        for current, requested in (("uefi", "bios"), ("multiboot", "uefi"),
                                   ("bios", "multiboot")):
            try:
                next_rung(current, "[CASCADE] HOST_REQUEST next=" + requested)
            except RuntimeError:
                continue
            raise RuntimeError("Supervisor accepted a skipped/upward transition")
        print("PASS cascade rejects skipped/upward transitions")

        if ovmf:
            target = scratch / "cascade"
            cascade_artifacts(target)
            transcript = logs / "host_cascade.log"
            result = subprocess.run([
                sys.executable, str(KERNEL / "host_boot.py"),
                "--rung", "uefi", "--images", str(target), "--ovmf", str(ovmf),
                "--exit-after-ready", "--log", str(transcript),
            ], text=True, stdout=subprocess.PIPE, stderr=subprocess.STDOUT,
                timeout=65)
            if result.returncode:
                raise RuntimeError("Hosted cascade failed:\n" + result.stdout)
            text = transcript.read_text()
            expected = ["[HOST] START rung=uefi",
                        "[HOST] TRANSITION uefi -> multiboot",
                        "[HOST] START rung=multiboot",
                        "[HOST] TRANSITION multiboot -> stage2",
                        "[HOST] START rung=stage2",
                        "[HOST] TRANSITION stage2 -> bios",
                        "[HOST] START rung=bios", "[HOST] READY rung=bios"]
            cursor = 0
            for marker in expected:
                found = text.find(marker, cursor)
                if found < 0:
                    raise RuntimeError(f"Missing ordered cascade event: {marker}")
                cursor = found + len(marker)
            print("PASS hosted cascade: UEFI -> Multiboot -> Stage 2 -> BIOS ready")


# BLOCK 4: CLOSING — failures propagate to the build script
if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--images", type=Path, default=KERNEL / "images")
    arguments = parser.parse_args()
    try:
        verify(arguments.images.resolve())
    except (OSError, RuntimeError, subprocess.SubprocessError, queue.Empty) as error:
        print(f"FAIL {error}", file=sys.stderr)
        sys.exit(1)
