### Safe Change
CLI app to safely execute unsafe code (which might cause unaccessibility), by reverting changes if no response recieved.
### It work in following manner
1. It backups the to be modified file. (ie. conf files)
2. executes your command. (will change configuration)
3. create a lockfile (you'll been given it's path).
4. initiate a timer of specified timer(or default 30sec).
5. The lockfile need to be deleted within timeout.
6. If lockfile exists after timeout, the changes will revert.
7. On another case, nothing happens,(ie. your change applied).

#### Didn't understand??, try this
```mermaid
graph
A(Safe Execute) --> B(Backup file) --> C(Execute specified Command) --> D(Create lockfile) --> E(initiate Timer) -- User can delete file withing this period --> F{Time Over}
F -- File deleted --> G(Nothing happen)
F -- File not deleted --> H(Revert the change)
H --{If command Specified}--> I(Post revert command)
```

### Usage:
```sh
> safe_change --help
CLI app to safely execute unsafe code (which might cause unaccessibility), by reverting changes if no response recieved

Usage: safe_change [OPTIONS] <COMMAND> <FILE_PATH> [REVERT_COMMAND]

Arguments:
  <COMMAND>         Command to execute safely
  <FILE_PATH>       File which encounter changes (ie. to backup)
  [REVERT_COMMAND]  Command to execute after revert [default: ]

Options:
  -t, --timer <TIMER>  timer after to revert changes [default: 30]
  -h, --help           Print help
```
