#!/bin/sh

flatpak build --with-appdir --allow=devel --bind-mount=/run/user/1000/doc=/run/user/1000/doc/by-app/io.github.Argentea --share=network --share=ipc --socket=fallback-x11 --device=dri --socket=wayland --filesystem=home --talk-name=org.freedesktop.portal.* --talk-name=org.a11y.Bus --bind-mount=/run/flatpak/at-spi-bus=/run/user/1000/at-spi/bus --env=AT_SPI_BUS_ADDRESS=unix:path=/run/flatpak/at-spi-bus --env=DESKTOP_SESSION=gnome --env=LANG=en_US.UTF-8 --env=WAYLAND_DISPLAY=wayland-0 --env=XDG_CURRENT_DESKTOP=GNOME --env=XDG_SESSION_DESKTOP=gnome --env=XDG_SESSION_TYPE=wayland --env=PATH=/var/home/mostafa/.cargo/bin:/var/home/mostafa/.local/bin:/var/home/mostafa/bin:/usr/local/bin:/usr/local/sbin:/usr/bin:/usr/sbin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin:/sbin:/bin:/app/bin:/usr/bin:/usr/lib/sdk/rust-stable/bin --env=LD_LIBRARY_PATH=/app/lib --env=PKG_CONFIG_PATH=/app/lib/pkgconfig:/app/share/pkgconfig:/usr/lib/pkgconfig:/usr/share/pkgconfig --share=network --bind-mount=/run/host/fonts=/usr/share/fonts --bind-mount=/run/host/fonts-cache=/usr/lib/fontconfig/cache --filesystem=/var/home/mostafa/.cache/fontconfig:ro --bind-mount=/run/host/user-fonts-cache=/var/home/mostafa/.cache/fontconfig --bind-mount=/run/host/font-dirs.xml=/var/home/mostafa/.cache/font-dirs.xml /var/home/mostafa/Projects/argentea/.flatpak/repo /usr/lib/sdk/rust-stable/bin/rust-analyzer "$@"