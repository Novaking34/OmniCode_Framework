#!/usr/bin/env python3
"""Host-supervised boot ladder. Each fallback starts a fresh QEMU guest.

Guest RAM is deliberately reset at each transition; serial evidence is retained
by the host. This is not an in-place firmware/CPU-mode transition.
"""

# BLOCK 1: METADATA — bounded, strictly descending hosted recovery
import argparse
from pathlib import Path
import queue
import subprocess
import sys
import tempfile
import threading
import time

# BLOCK 2: OPENING — rung contracts and command construction
RUNGS = ("uefi", "multiboot", "stage2", "bios")
READY = {
    "uefi": "MILLENIUMOS UEFI OS ONLINE :: PROCEED",
    "multiboot": "MILLENIUMOS MULTIBOOT ONLINE :: PROCEED",
    "stage2": "MILLENIUMOS STAGE 2 RUNTIME :: PROCEED",
    "bios": "MILLENIUMOS SAFE MODE :: PROCEED",
}
REQUEST = "[CASCADE] HOST_REQUEST next="


def next_rung(current, line):
    """Accept only an exact request for the immediately lower rung."""
    if not line.startswith(REQUEST):
        return None
    index = RUNGS.index(current)
    if index + 1 == len(RUNGS) or line != REQUEST + RUNGS[index + 1]:
        raise RuntimeError(f"Invalid cascade request from {current}: {line}")
    return RUNGS[index + 1]


def qemu_command(qemu, images, rung, ovmf=None, graphical=False):
    command = [str(qemu), "-no-reboot", "-monitor", "none", "-net", "none"]
    if rung == "uefi":
        if not ovmf or not Path(ovmf).is_file():
            raise RuntimeError("UEFI requires a readable OVMF firmware image")
        command += ["-bios", str(ovmf), "-drive",
                    f"format=raw,file=fat:rw:{images / 'esp'}"]
    elif rung == "multiboot":
        command += ["-kernel", str(images / "omnicode_multiboot.elf")]
    else:
        image = "omnicode_bios_dual.img" if rung == "stage2" else "omnicode_bios.img"
        command += ["-drive", f"format=raw,file={images / image}"]
    if not graphical:
        command += ["-display", "none"]
    return command + ["-serial", "stdio"]


def stop_guest(process):
    if process.poll() is None:
        process.terminate()
        try:
            process.wait(timeout=3)
        except subprocess.TimeoutExpired:
            process.kill()
            process.wait()


# BLOCK 3: BODY — one child at a time, retained evidence, bounded audit waits
def supervise(args, initial_command=None):
    rung = args.rung
    args.images = args.images.resolve()
    # QEMU's FAT backend treats ':' in a path as a protocol separator.
    # Stage only the small EFI payload into a unique colon-free directory.
    import shutil
    with tempfile.TemporaryDirectory(prefix="omnicode-boot-") as scratch:
        staged = Path(scratch)
        firmware = args.images / "esp/EFI/BOOT/BOOTX64.EFI"
        if rung == "uefi" and initial_command is None:
            (staged / "esp/EFI/BOOT").mkdir(parents=True)
            shutil.copyfile(firmware, staged / "esp/EFI/BOOT/BOOTX64.EFI")
        log_path = args.log
        log_path.parent.mkdir(parents=True, exist_ok=True)
        with log_path.open("w", encoding="utf-8") as log:
            def emit(line):
                print(line, flush=True)
                log.write(line + "\n")
                log.flush()

            while True:
                emit(f"[HOST] START rung={rung}")
                command = initial_command or qemu_command(
                    args.qemu, staged if rung == "uefi" else args.images,
                    rung, args.ovmf, args.graphical,
                )
                initial_command = None
                events = queue.Queue(maxsize=256)
                process = subprocess.Popen(
                    command, stdout=subprocess.PIPE, text=True,
                    errors="replace", bufsize=1,
                )

                def receive(stream):
                    try:
                        for line in stream:
                            events.put(line.rstrip("\r\n"))
                    finally:
                        events.put(None)

                reader = threading.Thread(
                    target=receive, args=(process.stdout,), daemon=True,
                )
                reader.start()
                deadline = time.monotonic() + args.timeout
                ready = False
                fallback = None
                try:
                    while True:
                        if not ready and time.monotonic() >= deadline:
                            raise RuntimeError(f"Boot readiness timed out at {rung}")
                        try:
                            line = events.get(timeout=0.1)
                        except queue.Empty:
                            continue
                        if line is None:
                            status = process.wait()
                            if status or not ready:
                                raise RuntimeError(
                                    f"QEMU exited at {rung}: status={status}, ready={ready}"
                                )
                            return 0
                        emit(line)
                        fallback = next_rung(rung, line)
                        if fallback:
                            break
                        if line == READY[rung]:
                            ready = True
                            emit(f"[HOST] READY rung={rung}")
                            if args.exit_after_ready:
                                return 0
                finally:
                    stop_guest(process)
                    # The bounded queue can retain output after a transition.
                    # Drain it while joining so the reader never leaks a child.
                    while reader.is_alive():
                        try:
                            events.get(timeout=0.1)
                        except queue.Empty:
                            pass
                        reader.join(timeout=0.01)
                    process.stdout.close()
                emit(f"[HOST] TRANSITION {rung} -> {fallback}; guest RAM reset")
                rung = fallback


# BLOCK 4: CLOSING — CLI errors fail closed and interrupt cleans up the child
def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--rung", choices=RUNGS, required=True)
    parser.add_argument("--images", type=Path, required=True)
    parser.add_argument("--qemu", default="qemu-system-x86_64")
    parser.add_argument("--ovmf", type=Path)
    parser.add_argument("--graphical", action="store_true")
    parser.add_argument("--exit-after-ready", action="store_true")
    parser.add_argument("--timeout", type=float, default=30)
    parser.add_argument("--log", type=Path)
    parser.add_argument("command", nargs=argparse.REMAINDER)
    args = parser.parse_args()
    if args.log is None:
        args.log = args.images / "logs" / f"host-{time.time_ns()}.log"
    command = args.command
    if command[:1] == ["--"]:
        command = command[1:]
    try:
        return supervise(args, command or None)
    except (OSError, RuntimeError) as error:
        print(f"[HOST] ERROR {error}", file=sys.stderr)
        return 1
    except KeyboardInterrupt:
        return 130


if __name__ == "__main__":
    sys.exit(main())
