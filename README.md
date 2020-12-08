# Wicker

Wicker is a WIP shell geared towards web developers. It is based on the concept
of sessions. A session is like an object, with its own implementation of several
methods:

- `read`: Reads a file.
- `write`: Writes a file.
- `delete`: Deletes a file.
- `cd`: Changes the active directory for the current session.

Each of these functions has a correspodning command, which runs that function in
the current active session. There can be an arbitrary amount of sessions, but only
one active session. At the moment, only two types of sessions are available:

- Local sessions. A local session implements read, write, and delete using operations
  on the local filesystem.
- Web sessions. A web session implements read, write, and delete using HTTP methods:
  GET For read, PUT/UPDATE for write, and DELETE for delete.

As well, each instance of the shell has a current working directory, which is joined with any
being read, written, or deleted.

## Commands

Only a few commands are implemented:
- `session-create`: Creates a new session.
- `session-switch`: Switches to a new session.
- `read`: Uses the current session to read a file.

However, Wicker is still in dev, and ideally soon the following commands will be implemented:
- `write`: Uses the current session to write a file.
- `delete`: Uses the current session to delete a file.
- `session-list`: Lists all existing sessions.

Feel free to PR and add a command.

## API / Coding Style

TBA
