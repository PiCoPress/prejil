# Prejil
Prejil is string interpreter what is written in Rust, and it will support as library Rust and C.
Also there is an official CLI tool.

# Definition
- `seek` is `skip` then gather the between chars, and stop.
- `skip` is just look a target and stop.

# Development plans - TODO
- pure cursor-based streaming
  -
  - seek
    -
    - ✅️`fn seek_char(char) -> String` ::: seek a char ahead
    - ✅️`fn seek_char_back(char) -> String` ::: seek a char backward
    - ❌️`fn seek_string(&str) -> String` ::: seek multiple chars ahead
    - ❌️`fn seek_string_back(&str) -> String` ::: seek multiple chars backward
  - skip
    -
    - ❌️`fn skip_char(char) -> String` ::: skip a char ahead
    - ❌️`fn skip_char_back(char) -> String` ::: skip a char backward
    - ❌️`fn skip_string(&str) -> String` ::: skip multiple chars ahead
    - ❌️`fn skip_string_back(&str) -> String` ::: skip multiple chars backward
  - status
    - 
    - ❌️`fn get_current_byte() -> u8` ::: show current byte at cursor

# Plans of development plans - Abstracted TODO
- library
  - 
  - Add rules to the kind of seek_* and skip_* function which supports it, and the rules make the streaming more powerful.
  - Create rule-supported functions like the above. 
  - Pure-based replace/delete/insert
  - Rule-based replace/delete/insert
  - Multi cursor + Multi threading
  - Register a code at rule
  - CFormat-like parse
  - implement library working on C
- CLI
  - 
  - It will work as library on shell(powershell/bash etc...)
