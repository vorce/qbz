---
name: Bug report
about: Report something that is not working as expected
title: "[Bug] "
labels: bug
assignees: ""
---

## Description

A clear and concise description of the problem.

## Steps to reproduce

1.
2.
3.

## Expected behavior

What you expected to happen.

## Actual behavior

What actually happened.

## Environment

- QBZ version:
- Installation method: <!-- AppImage / .deb / .rpm / AUR / Tarball / Flatpak / Built from source -->
- Distribution:
- Desktop environment / WM:
- Display server: <!-- Wayland / X11 -->
- GPU: <!-- e.g. NVIDIA RTX 3070, AMD RX 6600 XT, Intel Iris Xe -->
- Audio backend (PipeWire / ALSA Direct / PulseAudio):
- Audio device / DAC (if relevant):

## Logs

Logs help us diagnose problems much faster. To share them:

1. Open **Settings > Developer Mode > View Logs**
2. Click **Upload** on both the **Terminal** and **Console** tabs
3. Paste both URLs here:

- Terminal log URL:
- Console log URL:

> If QBZ crashes on startup, you can capture terminal output by running it from a terminal:
> ```
> ./QBZ*.AppImage 2>&1 | tee /tmp/qbz-crash.log
> ```
> Then attach or paste the contents of `/tmp/qbz-crash.log`.

## Screenshots

If applicable, add screenshots to help illustrate the issue.

## Additional context

Anything else that may help reproduce or understand the issue.
