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
### Configuration
```bash
lazycloud init
```
which creates a default config (`config.toml`) that looks something like this:

```toml
# lazycloud config

# [[sync]]
# name = "" # name of the profile
# from = "" # has to be full path
# to = "" # example gdrive:projects
# mode = "replace" # or "mirror", "copy", "move"
# flags = "--progress" # any flags that are normally supported in rclone
```

#### Name (`name`)
The name of the profile that will be the unique identifier of it throughout the usage of this cli.

#### From and To (`from` & `to`)
Self-explanatory, just make sure they are full path.

#### Modes (`mode`)
There are currently 4 modes (more to be added)
- mirror: copies new/ changed files to the destination, and **delete files from destination that don't exist in source**.
- replace: same as `mirror`, but also delete files that **match exclude patterns**.
- copy: copy without deleting anything
- move: move files to destination and **delete them from source**.

#### Flags (`flags`)
Any additional flags that you normally use in rclone. 

#### Example:
```toml
[[sync]]
name = "projects"
from = "/home/[computer-name]/Projects" # note it can be file or folder
to = "gdrive:Projects" # a folder named "Projects" on Google Drive
mode = "replace"
flags = "--progress" # display progress for easy tracking
```
<img width="447" height="518" alt="beautiful illustration" src="https://github.com/user-attachments/assets/102f3b0f-3ff1-42f7-88ba-45339759ec26" />

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
