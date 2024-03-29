#!/usr/bin/env sh
set -e
# Automatically added by dh_installsystemd/12.10ubuntu1
if [ -d /run/systemd/system ]; then
        systemctl --system daemon-reload >/dev/null || true
fi
# End automatically added section
# Automatically added by dh_installsystemd/12.10ubuntu1
if [ "$1" = "remove" ]; then
        if [ -x "/usr/bin/deb-systemd-helper" ]; then
                deb-systemd-helper mask 'dauth-directory.service' >/dev/null || true
        fi
fi

if [ "$1" = "purge" ]; then
        if [ -x "/usr/bin/deb-systemd-helper" ]; then
                deb-systemd-helper purge 'dauth-directory.service' >/dev/null || true
                deb-systemd-helper unmask 'dauth-directory.service' >/dev/null || true
        fi
fi
# End automatically added section
