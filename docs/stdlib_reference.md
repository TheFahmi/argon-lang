# Argon Standard Library Reference

## Math Functions

| Function | Description | Example |
|----------|-------------|---------|
| `abs(n)` | Absolute value | `abs(-42)` → `42` |
| `max(a, b)` | Maximum of two numbers | `max(3, 7)` → `7` |
| `min(a, b)` | Minimum of two numbers | `min(3, 7)` → `3` |
| `rand()` | Random number (0-999999) | `rand()` → `374600` |
| `randInt(min, max)` | Random integer in range | `randInt(1, 100)` → `42` |

## String Functions

| Function | Description | Example |
|----------|-------------|---------|
| `len(s)` | Length of string | `len("hello")` → `5` |
| `substr(s, start, len)` | Substring | `substr("hello", 1, 3)` → `"ell"` |
| `trim(s)` | Remove whitespace | `trim("  hi  ")` → `"hi"` |
| `toUpper(s)` | Uppercase | `toUpper("hello")` → `"HELLO"` |
| `toLower(s)` | Lowercase | `toLower("HELLO")` → `"hello"` |
| `contains(s, sub)` | Check substring | `contains("hello", "ell")` → `true` |
| `startsWith(s, prefix)` | Check prefix | `startsWith("hello", "he")` → `true` |
| `endsWith(s, suffix)` | Check suffix | `endsWith("hello", "lo")` → `true` |
| `replace(s, from, to)` | Replace substring | `replace("hello", "l", "x")` → `"hexxo"` |
| `split(s, delim)` | Split into array | `split("a,b,c", ",")` → `["a","b","c"]` |
| `join(arr, delim)` | Join array | `join(["a","b"], "-")` → `"a-b"` |
| `charAt(s, idx)` | Character at index | `charAt("hello", 1)` → `"e"` |
| `indexOf(s, sub)` | Find substring index | `indexOf("hello", "l")` → `2` |
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
| `findIndex(arr, val)` | Find index | `findIndex([1,2,3], 2)` → `1` |

## Type Functions

| Function | Description | Example |
|----------|-------------|---------|
| `typeof(val)` | Get type name | `typeof(42)` → `"int"` |
| `isNull(val)` | Check if null | `isNull(null)` → `true` |
| `isInt(val)` | Check if integer | `isInt(42)` → `true` |
| `isString(val)` | Check if string | `isString("hi")` → `true` |
| `isArray(val)` | Check if array | `isArray([1,2])` → `true` |

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
| `timestampMs()` | Unix timestamp (milliseconds) | `timestampMs()` → `1704067200000` |
| `dateNow()` | Current date string | `dateNow()` → `"2024-01-01"` |
| `now()` | Alias for timestamp() | `now()` → `1704067200` |

## Crypto Functions

| Function | Description | Example |
|----------|-------------|---------|
| `uuid()` | Generate unique ID | `uuid()` → `"a1b2c3d4-..."` |
| `bcryptHash(pwd)` | Hash password | `bcryptHash("secret")` |
| `bcryptVerify(pwd, hash)` | Verify password | `bcryptVerify("secret", hash)` → `true` |
| `jwtSign(payload, secret)` | Create JWT token | `jwtSign("data", "key")` |
| `jwtVerify(token, secret)` | Verify JWT | `jwtVerify(token, "key")` → `"data"` |

## Environment Functions

| Function | Description | Example |
|----------|-------------|---------|
| `env(key, default?)` | Get env variable | `env("PATH", "")` |
| `getArgs()` | Get program arguments | `getArgs()` → `["arg1", "arg2"]` |
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
| `argonListen(port)` | Start TCP listener |
| `argonAccept(listener_id)` | Accept connection |
| `argonSocketRead(socket_id)` | Read from socket |
| `argonSocketWrite(socket_id, data)` | Write to socket |
| `argonSocketClose(socket_id)` | Close socket |

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
