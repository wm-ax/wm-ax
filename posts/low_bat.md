# Sleep if the battery is low (Arch Linux)

After I switched to Linux, the biggest annoyance in my setup was a seeming triviality: the resulting system gave no low battery warning, but simply shut down when the battery ran out. Eventually I got around to fixing this.

The solution I'm using has two parts.  First, write a shell script to check whether the battery level falls below some threshold, and to hibernate the machine if it is.  Second, create a systemd service to run the script at regular intervals.

## 1. The shell script

According to general Unix design, "everything is a file".  This includes battery data:
```bash
> ls /sys/class/power_supply/BAT0
alarm                   device              model_name     subsystem
capacity                energy_full         power          technology
capacity_level          energy_full_design  power_now      type
charge_start_threshold  energy_now          present        uevent
charge_stop_threshold   hwmon3              serial_number  voltage_min_design
cycle_count             manufacturer        status         voltage_now
```

For example,
```bash
> cat /sys/class/power_supply/BAT0/capacity
84
```

reveals that the battery is 84% full.

To hook the battery level into a shell script, we can run the above command and store its result in a variable:
```bash
> capacity=`cat /sys/class/power_supply/BAT0/capacity`
> echo $capacity
78
```

Our goal is for the machine to sleep if the capacity falls below some threshold percentage: but we likely also want this to happen only if the battery is discharging.  How to detect this?  After unplugging the charger, you should find:
```bash
> status=`cat /sys/class/power_supply/BAT0/status`
> echo $status
Discharging
```
We also need to decide how low the battery should be allowed to fall, for example:
```bash
> THRESHOLD=5
```    
The desired logic now looks like this:
```bash
> if [[ "$status" = Discharging && $capacity -lt $THRESHOLD ]]
> then
> echo 'I should sleep!'
> else
> echo "I'm ok"
> fi
I'm ok
```
I enquoted ``$status`` because I don't know that none of its possible string values contain something syntactically problematic like whitespace.  For readability I used double brackets and '&&' in the condition, which means that the script must run with bash rather than with sh.

You can now test each of four relevant possibilities by plugging or unplugging your charger, and adjusting ``$THRESHOLD`` above or below your current battery level.  Note that the system will scale the threshold to an estimate of your battery's actual maximum capacity, rather than to its nominal factory-rated capacity.

Of course we want the machine actually to sleep, not just to log a message.  Initially I tried:
```bash
> systemctl hibernate
[pffff... ]
```
According to the [Arch docs](https://wiki.archlinux.org/index.php/Power_management/Suspend_and_hibernate), ``hibernate`` means "save the state to swap space and poweroff completely".  I'd really rather the machine just did what it does when the lid is closed, i.e., "save state to RAM and cut off power to most systems".  The command for that is ``systemctl suspend``.

When we run the script in the background, it will be helpful to have logged its behavior.  Since systemd sends stdout to syslog, we can log results with ``echo``.

We can now put the pieces together.
```bash
> if [[ "$status" = Discharging && $capacity -lt $THRESHOLD ]]
> then
> echo "battery level critical!"
> systemctl suspend
> else
> echo "checked battery level"
> fi
```

## 2. Running the script

The above sequence of shell commands will get the machine to sleep if the battery level falls below some threshold.  Instead of just typing the commands into the shell, we now want to save them as a script and then configure the machine to run the script at regular intervals.

The man pages for hier suggest where to put the script in the Linux filesystem hierarchy:

```
> man hier
[...]
/usr/local
    This is where programs which are local to the site typically go.
[...]
/usr/local/bin
    Binaries for programs local to the site.
```

I therefore saved the script as `low_bat.sh` in `/usr/local/bin`.

In Linux, there are two main ways to automate tasks: the old Unix utility `cron`, and the newer Linux-specific system and service management suite systemd.  For simple automation, both are easy to use, so I will explain each.

### 2.1 `systemd`

### 2.2 `cron`

Cron is an old Unix utility.  It is no longer included by default in all Linux flavors, so you may need first to install it.  (In Arch Linux, the package I used is `cronie`.)  The utility `cron` maintains for each user a table of files called a crontab.  Each crontab file gives a command plus a schedule to tell cron when that command should be run.  Once cron itself is running, it executes the commands on that schedule.  So to automate commands run by cron we first need to automate cron itself.  Since Arch runs systemd, installing cronie also adds a file `cronie.service` to `/lib/systemd/system`.  So to automate cron, it is enough just to enable and start it: `systemctl enable --now cronie`.

Using cron is just a matter of manipulating the files in the crontab table.  You can view the current user's crontab with `crontab -l`, and edit them with `crontab -e`.  Each file has the following format: 

[minute 0-59] [hour 0-23] [day of the month 1-31] [month 1-12] [weekday Sun-Sat 0-6] [command]

For example, the crontab 
```
01 02 03 04 05 /usr/local/bin/low_bat.sh
```
tells cron to run the `low_bat.sh` script at 2:01am on each April 3rd which happens to be a Friday.

This is obviously a bit rigid, but cron gives a few more options.

First, `*` can be used as a wildcard: so 
```
* * * * * /usr/local/bin/low_bat.sh
```
matches every hour, minute, day, month and weekday, and therefore runs the script every minute.

For custom intervals, the syntax `*/n` allows matching of every nth instance of the corresponding unit.  For example, 
```
*/5 * * * * /usr/local/bin/low_bat.sh
```
runs the script every fifth minute.

FWIW, it's also possible to list more than one specific instance of a given unit.  E.g.,```
17,19 */2 * * 00,06 /usr/local/bin/low_bat.sh
```
executes on the 17th and 19th minutes of every second hour on Saturdays and Sundays.

Having composed your crontab instruction, just save and close the file.  Since cron is already running, it will detect (using inotify) the changes and immediately begin to attempt the new instruction.

To check that the script is working, I used the systemd log files:
```
> sudo journalctl -u cronie | tail
[...] (mw) CMDOUT (/bin/sh: /usr/local/bin/low_bat.sh: Permission denied)
```
The parenthesized `mw` indicates that cron is trying to run the task as the user `mw` (who is the owner of the given cron table).  But the owner of the script file is `root` not `mw`, and file permissions do not grant execute to all users.  If, as in the present case, it's OK for any user to run this script, then one way to fix the problem is simply to grant the corresponding permission:
```
> sudo chmod o+x /usr/local/bin/low_bat.sh
```
