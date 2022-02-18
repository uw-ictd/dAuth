#!/usr/bin/env sh

set -e
# Automatically added by dh_installsystemd/12.10ubuntu1
if [ "$1" = "configure" ] || [ "$1" = "abort-upgrade" ] || [ "$1" = "abort-deconfigure" ] || [ "$1" = "abort-remove" ] ; then
        # This will only remove masks created by d-s-h on package removal.
        deb-systemd-helper unmask 'dauth.service' >/dev/null || true

        # was-enabled defaults to true, so new installations run enable.
        if deb-systemd-helper --quiet was-enabled 'dauth.service'; then
                # Enables the unit on first installation, creates new
                # symlinks on upgrades if the unit file has changed.
                deb-systemd-helper enable 'dauth.service' >/dev/null || true
        else
                # Update the statefile to add new symlinks (if any), which need to be
                # cleaned up on purge. Also remove old symlinks.
                deb-systemd-helper update-state 'dauth.service' >/dev/null || true
        fi
fi
# End automatically added section
# Automatically added by dh_installsystemd/12.10ubuntu1
if [ "$1" = "configure" ] || [ "$1" = "abort-upgrade" ] || [ "$1" = "abort-deconfigure" ] || [ "$1" = "abort-remove" ] ; then
        if [ -d /run/systemd/system ]; then
                systemctl --system daemon-reload >/dev/null || true
                if [ -n "$2" ]; then
                        _dh_action=restart
                else
                        _dh_action=start
                fi
                deb-systemd-invoke $_dh_action 'dauth.service' >/dev/null || true
        fi
fi
# End automatically added section
