# Argon Standard Library Reference

## Math Functions

| Function | Description | Example |
|----------|-------------|---------|
| `abs(n)` | Absolute value | `abs(-42)` → `42` |
| `max(a, b)` | Maximum of two numbers | `max(3, 7)` → `7` |
| `min(a, b)` | Minimum of two numbers | `min(3, 7)` → `3` |
| `rand()` | Random number (0-999999) | `rand()` → `374600` |
| `rand_int(min, max)` | Random integer in range | `rand_int(1, 100)` → `42` |

## String Functions

| Function | Description | Example |
|----------|-------------|---------|
| `len(s)` | Length of string | `len("hello")` → `5` |
| `substr(s, start, len)` | Substring | `substr("hello", 1, 3)` → `"ell"` |
| `trim(s)` | Remove whitespace | `trim("  hi  ")` → `"hi"` |
| `to_upper(s)` | Uppercase | `to_upper("hello")` → `"HELLO"` |
| `to_lower(s)` | Lowercase | `to_lower("HELLO")` → `"hello"` |
| `contains(s, sub)` | Check substring | `contains("hello", "ell")` → `true` |
| `starts_with(s, prefix)` | Check prefix | `starts_with("hello", "he")` → `true` |
| `ends_with(s, suffix)` | Check suffix | `ends_with("hello", "lo")` → `true` |
| `replace(s, from, to)` | Replace substring | `replace("hello", "l", "x")` → `"hexxo"` |
| `split(s, delim)` | Split into array | `split("a,b,c", ",")` → `["a","b","c"]` |
| `join(arr, delim)` | Join array | `join(["a","b"], "-")` → `"a-b"` |
| `char_at(s, idx)` | Character at index | `char_at("hello", 1)` → `"e"` |
| `index_of(s, sub)` | Find substring index | `index_of("hello", "l")` → `2` |
| `repeat(s, n)` | Repeat string | `repeat("ab", 3)` → `"ababab"` |
| `reverse(s)` | Reverse string | `reverse("hello")` → `"olleh"` |

## Array Functions

| Function | Description | Example |
|----------|-------------|---------|
| `len(arr)` | Array length | `len([1,2,3])` → `3` |
| `push(arr, val)` | Add to end | `push([1,2], 3)` → `[1,2,3]` |
| `pop(arr)` | Remove from end | `pop([1,2,3])` → `3` |
| `shift(arr)` | Remove from start | `shift([1,2,3])` → `1` |
| `reverse(arr)` | Reverse array | `reverse([1,2,3])` → `[3,2,1]` |
| `sort(arr)` | Sort array | `sort([3,1,2])` → `[1,2,3]` |
| `slice(arr, start, end)` | Slice array | `slice([0,1,2,3], 1, 3)` → `[1,2]` |
| `range(start, end, step?)` | Generate range | `range(0, 5)` → `[0,1,2,3,4]` |
| `contains(arr, val)` | Check if contains | `contains([1,2,3], 2)` → `true` |
| `find_index(arr, val)` | Find index | `find_index([1,2,3], 2)` → `1` |

## Type Functions

| Function | Description | Example |
|----------|-------------|---------|
| `typeof(val)` | Get type name | `typeof(42)` → `"int"` |
| `is_null(val)` | Check if null | `is_null(null)` → `true` |
| `is_int(val)` | Check if integer | `is_int(42)` → `true` |
| `is_string(val)` | Check if string | `is_string("hi")` → `true` |
| `is_array(val)` | Check if array | `is_array([1,2])` → `true` |

## Conversion Functions

| Function | Description | Example |
|----------|-------------|---------|
| `int(val)` | Convert to integer | `int("123")` → `123` |
| `str(val)` | Convert to string | `str(456)` → `"456"` |
| `toString(val)` | Convert to string | `toString(true)` → `"true"` |
| `parseInt(s)` | Parse integer | `parseInt("42")` → `42` |

## File I/O Functions

| Function | Description | Example |
|----------|-------------|---------|
| `readFile(path)` | Read file content | `readFile("data.txt")` |
| `writeFile(path, content)` | Write to file | `writeFile("out.txt", "hello")` |
| `fileExists(path)` | Check file exists | `fileExists("data.txt")` → `true` |

## Date/Time Functions

| Function | Description | Example |
|----------|-------------|---------|
| `timestamp()` | Unix timestamp (seconds) | `timestamp()` → `1704067200` |
| `timestamp_ms()` | Unix timestamp (milliseconds) | `timestamp_ms()` → `1704067200000` |
| `date_now()` | Current date string | `date_now()` → `"2024-01-01"` |
| `now()` | Alias for timestamp() | `now()` → `1704067200` |

## Crypto Functions

| Function | Description | Example |
|----------|-------------|---------|
| `uuid()` | Generate unique ID | `uuid()` → `"a1b2c3d4-..."` |
| `bcrypt_hash(pwd)` | Hash password | `bcrypt_hash("secret")` |
| `bcrypt_verify(pwd, hash)` | Verify password | `bcrypt_verify("secret", hash)` → `true` |
| `jwt_sign(payload, secret)` | Create JWT token | `jwt_sign("data", "key")` |
| `jwt_verify(token, secret)` | Verify JWT | `jwt_verify(token, "key")` → `"data"` |

## Environment Functions

| Function | Description | Example |
|----------|-------------|---------|
| `env(key, default?)` | Get env variable | `env("PATH", "")` |
| `get_args()` | Get program arguments | `get_args()` → `["arg1", "arg2"]` |
| `sleep(ms)` | Pause execution | `sleep(1000)` |
| `exit(code?)` | Exit program | `exit(0)` |

## Debug Functions

| Function | Description | Example |
|----------|-------------|---------|
| `print(val)` | Print to stdout | `print("hello")` |
| `debug(val)` | Debug print | `debug(myvar)` |
| `assert(cond, msg?)` | Assert condition | `assert(x > 0, "x must be positive")` |

---

## Networking Functions

| Function | Description |
|----------|-------------|
| `argon_listen(port)` | Start TCP listener |
| `argon_accept(listener_id)` | Accept connection |
| `argon_socket_read(socket_id)` | Read from socket |
| `argon_socket_write(socket_id, data)` | Write to socket |
| `argon_socket_close(socket_id)` | Close socket |

---

## Type Reference

Argon supports the following types:

- `null` - Null value
- `int` - 64-bit integer
- `bool` - Boolean (true/false)
- `string` - String
- `array` - Dynamic array of any values
- `struct` - Named struct with fields
- `function` - Function reference
