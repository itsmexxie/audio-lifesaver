# Audio Lifesaver
Mute all audio & pause all mpris-based players when you disconnect your headphones.

Written in Rust with minimal Rust knowledge 😎

## Installation
Copy the `other/audio-lifesaver.service` file into either:

```bash
/etc/systemd/user
~/.config/systemd/user
```

Replace the path in the `ExecStart` with the path to where you installed the binary, then execute these two commands. First command runs the service once, second command registers the service to run it at startup.

```bash
systemctl --user start audio-lifesaver.service
systemctl --user enable audio-lifesaver.service
```
