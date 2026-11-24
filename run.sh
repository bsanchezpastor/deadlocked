#!/usr/bin/env bash

fail() {
	echo "Not a git repository (or any of the parent directories): .git.
Do NOT download the repository as a zip file from github.com!
Please download deadlocked by cloning the Git repository: 'git clone https://github.com/avitran0/deadlocked'"
	exit 1
}

[[ -d '.git' ]] || fail

if [ ! -d /sys/module/iomem_rw ]; then
    sudo insmod kmod/iomem_rw.ko || exit 1
fi

if [ -c /dev/uinput ] && ! groups | grep -qw uinput && getent group uinput > /dev/null; then
    exec sg uinput -c "cargo run --release --bin deadlocked"
fi
cargo run --release --bin deadlocked
