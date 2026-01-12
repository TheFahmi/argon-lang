# Cryo Standard Library Reference

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
| `cryoListen(port)` | Start TCP listener |
| `cryoAccept(listener_id)` | Accept connection |
| `cryoSocketRead(socket_id)` | Read from socket |
| `cryoSocketWrite(socket_id, data)` | Write to socket |
| `cryoSocketClose(socket_id)` | Close socket |

---

## Database Functions (sqlite module)

The SQLite-compatible in-memory database driver provides SQL operations.

```cryo
import "sqlite"

let db = sqliteOpenMemory();
sqliteExec(db, "CREATE TABLE users (id INTEGER, name TEXT)");
sqliteExec(db, "INSERT INTO users (name) VALUES ('Alice')");
let result = sqliteQuery(db, "SELECT * FROM users");
```

| Function | Description |
|----------|-------------|
| `sqliteOpenMemory()` | Open in-memory database |
| `sqliteExec(conn, sql)` | Execute SQL (CREATE, INSERT, UPDATE, DELETE) |
| `sqliteQuery(conn, sql)` | Execute SELECT query |
| `sqliteClose(conn)` | Close database connection |
| `sqliteGetTables(conn)` | Get list of table names |
| `sqliteGetColumns(conn, table)` | Get columns for a table |
| `sqliteTableExists(conn, table)` | Check if table exists |

---

## JWT Functions (jwt module)

JSON Web Token creation and verification.

```cryo
import "jwt"

// Create token
let payload = { sub: "user123", name: "Alice" };
payload["iat"] = getCurrentTimestamp();
payload["exp"] = getCurrentTimestamp() + 3600;

let headerB64 = base64UrlEncode(jsonStringify({ alg: "HS256", typ: "JWT" }));
let payloadB64 = base64UrlEncode(jsonStringify(payload));
let signature = hmacSha256("secret", headerB64 + "." + payloadB64);
let token = headerB64 + "." + payloadB64 + "." + base64UrlEncode(signature);

// Verify token
let result = verifyJwt(token, "secret");
if (result.valid) {
    print("User: " + result.payload["sub"]);
}
```

| Function | Description |
|----------|-------------|
| `verifyJwt(token, secret)` | Verify JWT and return JwtToken struct |
| `jwtDecode(token)` | Decode JWT payload without verification |
| `jwtIsExpired(token)` | Check if token is expired |
| `jwtGetSubject(token)` | Get subject claim from token |
| `jwtGetClaim(token, name)` | Get specific claim from token |
| `base64UrlEncode(data)` | Encode data to base64url |
| `base64UrlDecode(data)` | Decode base64url data |
| `hmacSha256(key, message)` | Generate HMAC signature |
| `jsonStringify(obj)` | Convert object to JSON string |
| `jsonParse(json)` | Parse JSON string to object |

---

## OAuth2 Functions (oauth2 module)

OAuth2 client configuration and authorization flow helpers.

```cryo
import "oauth2"

// Create provider config
let config = oauth2GitHubConfig("client-id", "client-secret", "http://localhost/callback");
let client = oauth2CreateClient(config);

// Get authorization URL
let authUrl = oauth2GetAuthorizationUrl(client, "random-state");
print("Redirect to: " + authUrl);

// After callback, set token manually
oauth2SetToken(client, "access-token", 3600, "refresh-token");
```

| Function | Description |
|----------|-------------|
| `oauth2CreateClient(config)` | Create OAuth2 client |
| `oauth2CreateConfig(...)` | Create custom OAuth2 config |
| `oauth2GoogleConfig(id, secret, redirect)` | Pre-configured Google OAuth2 |
| `oauth2GitHubConfig(id, secret, redirect)` | Pre-configured GitHub OAuth2 |
| `oauth2MicrosoftConfig(id, secret, redirect, tenant)` | Pre-configured Microsoft OAuth2 |
| `oauth2FacebookConfig(id, secret, redirect)` | Pre-configured Facebook OAuth2 |
| `oauth2DiscordConfig(id, secret, redirect)` | Pre-configured Discord OAuth2 |
| `oauth2GetAuthorizationUrl(client, state)` | Generate authorization URL |
| `oauth2SetToken(client, access, expires, refresh)` | Set tokens manually |
| `oauth2ClearToken(client)` | Clear stored tokens |
| `oauth2IsTokenValid(client)` | Check if token is still valid |
| `urlEncode(s)` | URL-encode a string |
| `joinScopes(scopes)` | Join scope array to string |

---

## Type Reference

Cryo supports the following types:

- `null` - Null value
- `int` - 64-bit integer
- `bool` - Boolean (true/false)
- `string` - String
- `array` - Dynamic array of any values
- `struct` - Named struct with fields
- `function` - Function reference

