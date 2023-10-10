# Command-Line Help for `octoprint-cli`

This document contains the help content for the `octoprint-cli` command-line program.

**Command Overview:**

* [`octoprint-cli`↴](#octoprint-cli)
* [`octoprint-cli completion`↴](#octoprint-cli-completion)
* [`octoprint-cli version`↴](#octoprint-cli-version)
* [`octoprint-cli system`↴](#octoprint-cli-system)
* [`octoprint-cli system commands`↴](#octoprint-cli-system-commands)
* [`octoprint-cli job`↴](#octoprint-cli-job)
* [`octoprint-cli job command`↴](#octoprint-cli-job-command)
* [`octoprint-cli job command start`↴](#octoprint-cli-job-command-start)
* [`octoprint-cli job command cancel`↴](#octoprint-cli-job-command-cancel)
* [`octoprint-cli job command restart`↴](#octoprint-cli-job-command-restart)
* [`octoprint-cli job command pause`↴](#octoprint-cli-job-command-pause)
* [`octoprint-cli job command pause pause`↴](#octoprint-cli-job-command-pause-pause)
* [`octoprint-cli job command pause resume`↴](#octoprint-cli-job-command-pause-resume)
* [`octoprint-cli job command pause toggle`↴](#octoprint-cli-job-command-pause-toggle)
* [`octoprint-cli printer`↴](#octoprint-cli-printer)
* [`octoprint-cli printer printhead`↴](#octoprint-cli-printer-printhead)
* [`octoprint-cli printer printhead jog`↴](#octoprint-cli-printer-printhead-jog)
* [`octoprint-cli printer printhead home`↴](#octoprint-cli-printer-printhead-home)
* [`octoprint-cli printer printhead feedrate`↴](#octoprint-cli-printer-printhead-feedrate)
* [`octoprint-cli printer tool`↴](#octoprint-cli-printer-tool)
* [`octoprint-cli printer tool command`↴](#octoprint-cli-printer-tool-command)
* [`octoprint-cli printer tool command target`↴](#octoprint-cli-printer-tool-command-target)
* [`octoprint-cli printer tool command offset`↴](#octoprint-cli-printer-tool-command-offset)
* [`octoprint-cli printer tool command select`↴](#octoprint-cli-printer-tool-command-select)
* [`octoprint-cli printer tool command extrude`↴](#octoprint-cli-printer-tool-command-extrude)
* [`octoprint-cli printer tool command flowrate`↴](#octoprint-cli-printer-tool-command-flowrate)
* [`octoprint-cli printer sd`↴](#octoprint-cli-printer-sd)
* [`octoprint-cli printer sd command`↴](#octoprint-cli-printer-sd-command)
* [`octoprint-cli printer sd command init`↴](#octoprint-cli-printer-sd-command-init)
* [`octoprint-cli printer sd command refresh`↴](#octoprint-cli-printer-sd-command-refresh)
* [`octoprint-cli printer sd command release`↴](#octoprint-cli-printer-sd-command-release)
* [`octoprint-cli printer bed`↴](#octoprint-cli-printer-bed)
* [`octoprint-cli printer bed command`↴](#octoprint-cli-printer-bed-command)
* [`octoprint-cli printer bed command target`↴](#octoprint-cli-printer-bed-command-target)
* [`octoprint-cli printer bed command offset`↴](#octoprint-cli-printer-bed-command-offset)

## `octoprint-cli`

Octoprint API in CLI

**Usage:** `octoprint-cli [OPTIONS] [COMMAND]`

###### **Subcommands:**

* `completion` — Generate shell completion
* `version` — Retrieve information regarding server and API version.
* `system` — system commands
* `job` — Retrieve information about the current job (if there is one).
* `printer` — Retrieves the current state of the printer.

###### **Options:**

* `--profile <PROFILE>` — Profile to use. default: no profile
* `--base-url <BASE_URL>` — Override the base url
* `-t`, `--auth-token <AUTH_TOKEN>` — Authorization token



## `octoprint-cli completion`

Generate shell completion

**Usage:** `octoprint-cli completion --generate <generator>`

###### **Options:**

* `--generate <GENERATOR>`

  Possible values: `bash`, `elvish`, `fish`, `powershell`, `zsh`




## `octoprint-cli version`

Retrieve information regarding server and API version.

**Usage:** `octoprint-cli version [OPTIONS]`

###### **Options:**

* `-f`, `--format <OUTPUT_FORMAT>` — Output format (default: toml or table)

  Possible values: `json`, `toml`, `yaml`




## `octoprint-cli system`

system commands

**Usage:** `octoprint-cli system [COMMAND]`

###### **Subcommands:**

* `commands` — Retrieves all configured system commands



## `octoprint-cli system commands`

Retrieves all configured system commands

**Usage:** `octoprint-cli system commands [OPTIONS] [action]`

###### **Arguments:**

* `<ACTION>`

###### **Options:**

* `-f`, `--format <OUTPUT_FORMAT>` — Output format (default: toml or table)

  Possible values: `json`, `toml`, `yaml`




## `octoprint-cli job`

Retrieve information about the current job (if there is one).

**Usage:** `octoprint-cli job [OPTIONS] [COMMAND]`

###### **Subcommands:**

* `command` — Send a job command. Default: start

###### **Options:**

* `-f`, `--format <OUTPUT_FORMAT>` — Output format (default: toml or table)

  Possible values: `json`, `toml`, `yaml`




## `octoprint-cli job command`

Send a job command. Default: start

**Usage:** `octoprint-cli job command [OPTIONS] [COMMAND]`

###### **Subcommands:**

* `start` — Starts the print of the currently selected file.
* `cancel` — Cancels the current print job.
* `restart` — Restart the print of the currently selected file from the beginning.
* `pause` — Pauses/resumes/toggles the current print job.

###### **Options:**

* `-f`, `--format <OUTPUT_FORMAT>` — Output format (default: toml or table)

  Possible values: `json`, `toml`, `yaml`




## `octoprint-cli job command start`

Starts the print of the currently selected file. For selecting a file, see Issue a file command. If a print job is already active, a 409 Conflict will be returned.

**Usage:** `octoprint-cli job command start`



## `octoprint-cli job command cancel`

Cancels the current print job. If no print job is active (either paused or printing), a 409 Conflict will be returned.

**Usage:** `octoprint-cli job command cancel`



## `octoprint-cli job command restart`

Restart the print of the currently selected file from the beginning. There must be an active print job for this to work and the print job must currently be paused. If either is not the case, a 409 Conflict will be returned.

    Equivalent to issuing a `cancel` command while paused, directly followed by a `start` command.


**Usage:** `octoprint-cli job command restart`



## `octoprint-cli job command pause`

Pauses/resumes/toggles the current print job. Accepts one optional additional parameter action specifying which action to take. 

**Usage:** `octoprint-cli job command pause [COMMAND]`

###### **Subcommands:**

* `pause` — Pauses the current job if it’s printing, does nothing if it’s already paused.
* `resume` — Resumes the current job if it’s paused, does nothing if it’s printing.
* `toggle` — Toggles the pause state of the job, pausing it if it’s printing and resuming it if it’s currently paused.



## `octoprint-cli job command pause pause`

Pauses the current job if it’s printing, does nothing if it’s already paused.

**Usage:** `octoprint-cli job command pause pause`



## `octoprint-cli job command pause resume`

Resumes the current job if it’s paused, does nothing if it’s printing.

**Usage:** `octoprint-cli job command pause resume`



## `octoprint-cli job command pause toggle`

Toggles the pause state of the job, pausing it if it’s printing and resuming it if it’s currently paused.

**Usage:** `octoprint-cli job command pause toggle`



## `octoprint-cli printer`

Retrieves the current state of the printer. Returned information includes:
  -  temperature information
  -  sd state
  -  general printer state
Temperature information can also be made to include the printer’s temperature history by supplying the history query parameter. The amount of data points to return here can be limited using the limit query parameter.
Clients can specific a list of attributes to not return in the response (e.g. if they don’t need it) via the exclude query parameter.

**Usage:** `octoprint-cli printer [OPTIONS] [COMMAND]`

###### **Subcommands:**

* `printhead` — Print head commands allow jogging and homing the print head in all three axes.
* `tool` — Query the tool temperature data and also include the temperature history but limit it to two entries.
* `sd` — Retrieves the current state of the printer’s SD card.
* `bed` — Retrieves the current temperature data (actual, target and offset).

###### **Options:**

* `-i`, `--input <INPUT_FILE>` — Read the data from file ('-' for stdin)
* `-t`, `--template` — (option) Generate an input template
* `-l`, `--limit <LIMIT>` — (option) only the last n data points from the printer’s temperature history will be returned.
* `-y`, `--history` — (option) include history information in the response too.
* `-e`, `--exclude <EXCLUDE>` — (option) An optional list of fields to exclude from the response

  Possible values: `temperature`, `sd`, `state`

* `-f`, `--format <OUTPUT_FORMAT>` — Output format (default: toml or table)

  Possible values: `json`, `toml`, `yaml`




## `octoprint-cli printer printhead`

Print head commands allow jogging and homing the print head in all three axes.

**Usage:** `octoprint-cli printer printhead [OPTIONS] [COMMAND]`

###### **Subcommands:**

* `jog` — Jogs the print head (relatively) by a defined amount in one or more axes
* `home` — Homes the print head in all of the given axes.
* `feedrate` — Changes the feedrate factor to apply to the movements of the axes.

###### **Options:**

* `-f`, `--format <OUTPUT_FORMAT>` — Output format (default: toml or table)

  Possible values: `json`, `toml`, `yaml`




## `octoprint-cli printer printhead jog`

Jogs the print head (relatively) by a defined amount in one or more axes

**Usage:** `octoprint-cli printer printhead jog [OPTIONS]`

###### **Options:**

* `--x <X>` — (option) Amount/coordinate to jog print head on x axis.
* `--y <Y>` — (option) Amount/coordinate to jog print head on y axis.
* `--z <Z>` — (option) Amount/coordinate to jog print head on z axis.
* `--absolute` — (option) Whether to move relative to current position or to absolute position
* `--speed <SPEED>` — (option) Speed at which to move.



## `octoprint-cli printer printhead home`

Homes the print head in all of the given axes.

**Usage:** `octoprint-cli printer printhead home [OPTIONS]`

###### **Options:**

* `--axes <AXES>` — (option) A list of axes which to home.



## `octoprint-cli printer printhead feedrate`

Changes the feedrate factor to apply to the movements of the axes.

**Usage:** `octoprint-cli printer printhead feedrate --factor <factor>`

###### **Options:**

* `--factor <FACTOR>` — The new feedrate factor.



## `octoprint-cli printer tool`

Query the tool temperature data and also include the temperature history but limit it to two entries.

**Usage:** `octoprint-cli printer tool [OPTIONS] [COMMAND]`

###### **Subcommands:**

* `command` — Tool commands allow setting the temperature and temperature offsets for the printer’s tools (hotends).

###### **Options:**

* `-f`, `--format <OUTPUT_FORMAT>` — Output format (default: toml or table)

  Possible values: `json`, `toml`, `yaml`




## `octoprint-cli printer tool command`

Tool commands allow setting the temperature and temperature offsets for the printer’s tools (hotends), selecting the current tool and extruding/retracting from the currently selected tool.

**Usage:** `octoprint-cli printer tool command [COMMAND]`

###### **Subcommands:**

* `target` — Sets the given target temperature on the printer’s tools.
* `offset` — Sets the given temperature offset on the printer’s tools.
* `select` — Selects the printer’s current tool.
* `extrude` — Extrudes the given amount of filament from the currently selected tool.
* `flowrate` — Changes the flow rate factor to apply to extrusion of the tool.



## `octoprint-cli printer tool command target`

Sets the given target temperature on the printer’s tools.

**Usage:** `octoprint-cli printer tool command target [OPTIONS]`

###### **Options:**

* `--targets-tool0 <TARGETS-TOOL0>`



## `octoprint-cli printer tool command offset`

Sets the given temperature offset on the printer’s tools.

**Usage:** `octoprint-cli printer tool command offset [OPTIONS]`

###### **Options:**

* `--offsets-tool0 <OFFSETS-TOOL0>`



## `octoprint-cli printer tool command select`

Selects the printer’s current tool.

**Usage:** `octoprint-cli printer tool command select --tool <tool>`

###### **Options:**

* `--tool <TOOL>` — Tool to select



## `octoprint-cli printer tool command extrude`

Extrudes the given amount of filament from the currently selected tool.

**Usage:** `octoprint-cli printer tool command extrude [OPTIONS] --amount <amount>`

###### **Options:**

* `--amount <AMOUNT>` — The amount of filament to extrude in mm.
* `--speed <SPEED>` — (option) Speed at which to extrude.



## `octoprint-cli printer tool command flowrate`

Changes the flow rate factor to apply to extrusion of the tool.

**Usage:** `octoprint-cli printer tool command flowrate --factor <factor>`

###### **Options:**

* `--factor <FACTOR>` — The new flowrate factor.



## `octoprint-cli printer sd`

Retrieves the current state of the printer’s SD card.

**Usage:** `octoprint-cli printer sd [OPTIONS] [COMMAND]`

###### **Subcommands:**

* `command` — SD commands allow initialization, refresh and release of the printer’s SD card (if available).

###### **Options:**

* `-f`, `--format <OUTPUT_FORMAT>` — Output format (default: toml or table)

  Possible values: `json`, `toml`, `yaml`




## `octoprint-cli printer sd command`

SD commands allow initialization, refresh and release of the printer’s SD card (if available).

**Usage:** `octoprint-cli printer sd command [OPTIONS] [COMMAND]`

###### **Subcommands:**

* `init` — Initializes the printer’s SD card, making it available for use. This also includes an initial retrieval of the list of files currently stored on the SD card, so after issuing that command a retrieval of the files on SD card will return a successful result.
* `refresh` — Refreshes the list of files stored on the printer’s SD card.
* `release` — Releases the SD card from the printer. The reverse operation to init. After issuing this command, the SD card won’t be available anymore, hence and operations targeting files stored on it will fail.

###### **Options:**

* `-f`, `--format <OUTPUT_FORMAT>` — Output format (default: toml or table)

  Possible values: `json`, `toml`, `yaml`




## `octoprint-cli printer sd command init`

Initializes the printer’s SD card, making it available for use. This also includes an initial retrieval of the list of files currently stored on the SD card, so after issuing that command a retrieval of the files on SD card will return a successful result.

**Usage:** `octoprint-cli printer sd command init`



## `octoprint-cli printer sd command refresh`

Refreshes the list of files stored on the printer’s SD card.

**Usage:** `octoprint-cli printer sd command refresh`



## `octoprint-cli printer sd command release`

Releases the SD card from the printer. The reverse operation to init. After issuing this command, the SD card won’t be available anymore, hence and operations targeting files stored on it will fail.

**Usage:** `octoprint-cli printer sd command release`



## `octoprint-cli printer bed`

Retrieves the current temperature data (actual, target and offset).

**Usage:** `octoprint-cli printer bed [OPTIONS] [COMMAND]`

###### **Subcommands:**

* `command` — Bed commands allow setting the temperature and temperature offsets for the printer’s heated bed.

###### **Options:**

* `-f`, `--format <OUTPUT_FORMAT>` — Output format (default: toml or table)

  Possible values: `json`, `toml`, `yaml`




## `octoprint-cli printer bed command`

Bed commands allow setting the temperature and temperature offsets for the printer’s heated bed.

**Usage:** `octoprint-cli printer bed command [OPTIONS] [COMMAND]`

###### **Subcommands:**

* `target` — Sets the given target temperature on the printer’s bed.
* `offset` — Sets the given temperature offset on the printer’s bed.

###### **Options:**

* `-f`, `--format <OUTPUT_FORMAT>` — Output format (default: toml or table)

  Possible values: `json`, `toml`, `yaml`




## `octoprint-cli printer bed command target`

Sets the given target temperature on the printer’s bed.

**Usage:** `octoprint-cli printer bed command target --temperature <temperature>`

###### **Options:**

* `--temperature <TEMPERATURE>` — Target temperature to set. A value of 0 will turn the heater off.



## `octoprint-cli printer bed command offset`

Sets the given temperature offset on the printer’s bed.

**Usage:** `octoprint-cli printer bed command offset --offsets <offsets>`

###### **Options:**

* `--offsets <OFFSETS>` — Offset(s) to set



<hr/>

<small><i>
    This document was generated automatically by
    <a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>

