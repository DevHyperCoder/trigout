# trigout - Trigger based formatter

## Usage
`trigout` ships with two binaries, `trigout` and `trigin`.

`trigout <socket name> <file name>`
> NOTE: for usage of filename, socket name MUST be provided.

Default socket name is `0`.
If filename is provided, stdout will also be copied over to the file, overwriting it.

## Configuration
Config file: `~/.config/trigout.json`
Each new socket address requires configuration in the json file

`json
[
  {
    "sock_name": "a", // Socket name to connect
    "format_str": "Hi! Date is {date}. Time is {h}:{m}:{s}",
  }
  // Other sockets
]
`

### Formatting
Use `{<var name>}` inside the string. `<var name>` can be `a-zA-Z0-9`

## Examples
This is a sample use case outlining all the features of trigout. The given example is for a status bar for [dwm](https://github.com/DevHyperCoder/dwm) (my fork).

**Configuration**
```json
[
  {
    "sock_name": "dwm",
    "format_str": "{date} | {volume}",
  }
]
```

2 [scripts](https://github.com/DevHyperCoder/dotfiles) are used, one to get the date and time formatted and one for volume

**Script**

`echo "volume=70%" | trigin dwm` -> Implementation of input

`trigout dwm /tmp/dwm-status` -> Creates a file which will be updated whenever stdout is updated

`ls /tmp/dwm-status | entr update-dwm-status` -> Use `entr` to monitor the file for changes and run the `update-dwm-status` script. 

## Potential use cases
- Status bar for a minimal window manager like dwm
- Auto compilation

## LICENSE

trigout is licensed under the GPL-3 license.
