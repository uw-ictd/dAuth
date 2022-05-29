#!/usr/bin/env sh

set -e
# Automatically added by dh_installsystemd/12.10ubuntu1
if [ -d /run/systemd/system ] && [ "$1" = remove ]; then
        deb-systemd-invoke stop 'dauth-directory.service' >/dev/null || true
fi
# End automatically added section
