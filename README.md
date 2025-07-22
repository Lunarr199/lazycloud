# > lazycloud ☁️

A lazy CLI sync tool powered by [`rclone`](https://github.com/rclone/rclone). Supports named profiles, auto-syncing in the background, and interval-based watchers.

---

## > Expectations
This is a wrapper (built over) for [rclone](https://github.com/rclone/rclone), and its main task is to make syncing commands shorter, and doesn't need to memorize (for lazy people like me), and there are some extra features to make your life easier. However, you still need to set up rclone like everyone else before using it (I know you are lazy, but it's easy c'mon).

## > Features
- Easy profile-based config
- One-off or scheduled syncs
- Watch profiles at intervals

## > Install
Find the correct binary version in the [Releases](https://github.com/lunar1um/lazycloud/releases) and move them to somewhere in your PATH.

Run `lazycloud --help` to check.

## > Usage
### Initialize
```bash
lazycloud init
```
which creates a default config (`config.toml`) that looks like this:

```toml
# lazycloud config

# [[sync]]
# name = "" # name of the profile
# from = "" # has to be full path
# to = "" # example gdrive:projects
# mode = "replace" # or "mirror", "copy", "move"
# flags = "--progress" # any flags that are normally supported in rclone
```

Note: every path has to be full path.

### Sync commands
1. Sync a single profile
```bash
lazycloud sync profile "[profile_name]"
```
2. Sync all available profiles
```bash
lazycloud sync all
```

### Watch commands
1. Watch a profile in the background (running like a daemon)
```bash
lazycloud watch [interval] profile "[profile_name]"
```
2. Watch all available profiles
```bash
lazycloud watch [interval] all
```
3. As this runs in the background, to stop it:
```bash
lazycloud stop profile "[profile_name]"
```
or use this to stop all profiles:
```bash
lazycloud stop all
```
4. List all running profiles
```bash
lazycloud status
```

### Other commands
1. Show all available profiles
```bash
lazycloud list
```

## Planned
- Start-up service (you can probably do this by yourself)
- TUI mode (I'm still wondering if this is a good idea)
