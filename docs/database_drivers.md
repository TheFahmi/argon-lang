# Cryo Database Drivers

This document describes the database drivers available in Cryo's standard library.

---

## SQLite Driver (`stdlib/sqlite.cryo`)

A pure-Cryo implementation of an in-memory SQL database with SQLite-compatible syntax.

### Quick Start

```cryo
import "sqlite"

fn main() {
    // Open in-memory database
    let db = sqliteOpenMemory();
    
    // Create table
    sqliteExec(db, "CREATE TABLE users (id INTEGER, name TEXT, email TEXT)");
    
    // Insert data
    sqliteExec(db, "INSERT INTO users (name, email) VALUES ('Alice', 'alice@example.com')");
    sqliteExec(db, "INSERT INTO users (name, email) VALUES ('Bob', 'bob@example.com')");
    
    // Query data
    let result = sqliteQuery(db, "SELECT * FROM users");
    
    // Process results
    let i = 0;
    while (i < len(result.rows)) {
        print("User: " + result.rows[i][1]);  // name column
        i = i + 1;
    }
    
    // Close connection
    sqliteClose(db);
}
```

### API Reference

#### Connection Functions

| Function | Description | Returns |
|----------|-------------|---------|
| `sqliteOpenMemory()` | Open in-memory database | `SqliteConnection` |
| `sqliteClose(conn)` | Close database connection | `SqliteResult` |

#### Execution Functions

| Function | Description | Returns |
|----------|-------------|---------|
| `sqliteExec(conn, sql)` | Execute SQL statement (CREATE, INSERT, UPDATE, DELETE) | `SqliteResult` |
| `sqliteQuery(conn, sql)` | Execute SELECT query | `SqliteResult` |

#### Utility Functions

| Function | Description | Returns |
|----------|-------------|---------|
| `sqliteGetTables(conn)` | Get list of all table names | `array` |
| `sqliteGetColumns(conn, tableName)` | Get column names for a table | `array` |
| `sqliteTableExists(conn, tableName)` | Check if table exists | `bool` |
| `sqliteRowCount(conn, tableName)` | Get number of rows in table | `int` |

### Result Structure

```cryo
struct SqliteResult {
    success: bool,      // Whether operation succeeded
    rows: array,        // Array of rows (SELECT results)
    columns: array,     // Column names
    affected: int,      // Number of affected rows
    last_id: int,       // Last inserted auto-increment ID
    error: string       // Error message if any
}
```

### Supported SQL Syntax

#### CREATE TABLE
```sql
CREATE TABLE users (id INTEGER, name TEXT, age INTEGER)
CREATE TABLE IF NOT EXISTS users (id INTEGER, name TEXT)
```

#### DROP TABLE
```sql
DROP TABLE users
DROP TABLE IF EXISTS users
```

#### INSERT
```sql
INSERT INTO users (name, email) VALUES ('Alice', 'alice@example.com')
INSERT INTO users VALUES (1, 'Bob', 'bob@example.com')
```

#### SELECT
```sql
SELECT * FROM users
SELECT name, email FROM users
SELECT * FROM users WHERE age = 25
SELECT * FROM users LIMIT 10
```

#### UPDATE
```sql
UPDATE users SET email = 'new@example.com' WHERE name = 'Alice'
```

#### DELETE
```sql
DELETE FROM users WHERE id = 1
```

### Test Results

All 17 tests passing:
- Open in-memory database ✓
- CREATE TABLE ✓
- INSERT INTO (3 rows) ✓
- SELECT * FROM ✓
- SELECT with WHERE clause ✓
- UPDATE ✓
- DELETE ✓
- SELECT with LIMIT ✓
- CREATE TABLE IF NOT EXISTS ✓
- DROP TABLE ✓
- Utility functions ✓
- Close database ✓

---

## JWT Library (`stdlib/jwt.cryo`)

JSON Web Token creation and verification.

### Quick Start

```cryo
import "jwt"

fn main() {
    // Create a JWT token manually
    let header = { alg: "HS256", typ: "JWT" };
    let payload = { sub: "user123" };
    payload["name"] = "Alice";
    payload["role"] = "admin";
    payload["iat"] = getCurrentTimestamp();
    payload["exp"] = getCurrentTimestamp() + 3600;  // 1 hour
    
    let headerB64 = base64UrlEncode(jsonStringify(header));
    let payloadB64 = base64UrlEncode(jsonStringify(payload));
    let signingInput = headerB64 + "." + payloadB64;
    let signature = hmacSha256("my-secret-key", signingInput);
    let signatureB64 = base64UrlEncode(signature);
    
    let token = signingInput + "." + signatureB64;
    print("Token: " + token);
    
    // Verify token
    let result = verifyJwt(token, "my-secret-key");
    if (result.valid) {
        print("Valid! User: " + result.payload["sub"]);
    } else {
        print("Invalid: " + result.error);
    }
}
```

### API Reference

| Function | Description |
|----------|-------------|
| `verifyJwt(token, secret)` | Verify JWT and return JwtToken struct |
| `jwtDecode(token)` | Decode payload without verification |
| `jwtIsExpired(token)` | Check if token is expired |
| `jwtGetSubject(token)` | Get subject (sub) claim |
| `jwtGetClaim(token, name)` | Get specific claim by name |
| `base64UrlEncode(data)` | Encode to base64url |
| `base64UrlDecode(data)` | Decode from base64url |
| `hmacSha256(key, message)` | Generate HMAC signature |
| `jsonStringify(obj)` | Convert object to JSON |
| `jsonParse(json)` | Parse JSON to object |
| `getCurrentTimestamp()` | Get current Unix timestamp |

### JwtToken Structure

```cryo
struct JwtToken {
    header: any,       // Decoded header object
    payload: any,      // Decoded payload object
    signature: string, // Base64url signature
    raw: string,       // Original token string
    valid: bool,       // Whether verification passed
    error: string      // Error message if invalid
}
```

### Test Results

All 6 tests passing:
- Create JWT Token ✓
- Verify JWT Token ✓
- Verify with wrong secret (rejected) ✓
- Decode JWT Token ✓
- Check token not expired ✓
- Get specific claim ✓

---

## OAuth2 Library (`stdlib/oauth2.cryo`)

OAuth2 client configuration and authorization flow helpers.

### Quick Start

```cryo
import "oauth2"

fn main() {
    // Create GitHub OAuth2 config
    let config = oauth2GitHubConfig(
        "your-client-id",
        "your-client-secret",
        "http://localhost:3000/auth/callback"
    );
    
    // Create client
    let client = oauth2CreateClient(config);
    
    // Generate authorization URL
    let state = generateState();
    let authUrl = oauth2GetAuthorizationUrl(client, state);
    
    print("Redirect user to: " + authUrl);
    
    // After callback, manually set the token
    oauth2SetToken(client, "access-token-from-callback", 3600, "refresh-token");
    
    // Check if token is valid
    if (oauth2IsTokenValid(client)) {
        print("Token is valid!");
        print("Access Token: " + oauth2GetAccessToken(client));
    }
}
```

### Pre-configured Providers

| Function | Provider |
|----------|----------|
| `oauth2GoogleConfig(id, secret, redirect)` | Google |
| `oauth2GitHubConfig(id, secret, redirect)` | GitHub |
| `oauth2MicrosoftConfig(id, secret, redirect, tenant)` | Microsoft |
| `oauth2FacebookConfig(id, secret, redirect)` | Facebook |
| `oauth2DiscordConfig(id, secret, redirect)` | Discord |

### API Reference

| Function | Description |
|----------|-------------|
| `oauth2CreateClient(config)` | Create OAuth2 client from config |
| `oauth2CreateConfig(...)` | Create custom config |
| `oauth2GetAuthorizationUrl(client, state)` | Generate authorization redirect URL |
| `oauth2SetToken(client, access, expires, refresh)` | Set tokens manually |
| `oauth2ClearToken(client)` | Clear stored tokens |
| `oauth2GetAccessToken(client)` | Get current access token |
| `oauth2IsTokenValid(client)` | Check if token is valid (not expired) |
| `urlEncode(s)` | URL-encode a string |
| `joinScopes(scopes)` | Join scope array to space-separated string |
| `generateState()` | Generate state parameter for CSRF protection |

### Structures

```cryo
struct OAuth2Config {
    clientId: string,
    clientSecret: string,
    authorizationUrl: string,
    tokenUrl: string,
    redirectUri: string,
    scopes: array
}

struct OAuth2Token {
    accessToken: string,
    tokenType: string,      // Usually "Bearer"
    expiresIn: int,         // Seconds until expiry
    refreshToken: string,
    scope: string,
    issuedAt: int,          // Unix timestamp
    error: string
}

struct OAuth2Client {
    config: OAuth2Config,
    token: OAuth2Token
}
```

### Test Results

All 6 tests passing:
- Create Google OAuth2 Config ✓
- Create GitHub OAuth2 Config ✓
- Create OAuth2 Client ✓
- URL Encoding ✓
- Join Scopes ✓
- Set Token Manually ✓

---

## Known Limitations

### SQLite Driver
- In-memory only (no file persistence yet)
- No complex JOINs or subqueries
- No ORDER BY clause
- Basic WHERE clause (single condition)

### JWT Library
- Uses simplified HMAC (not cryptographically secure for production)
- Only HS256 algorithm supported
- Requires manual timestamp management

### OAuth2 Library
- No automatic HTTP token exchange (requires manual token setting)
- No PKCE flow implementation
- Requires external HTTP client for actual OAuth2 flow

---

## Future Improvements

1. **SQLite**: File persistence, complex queries, indexes
2. **JWT**: Proper SHA256-HMAC, RS256 support, auto-refresh
3. **OAuth2**: HTTP client integration, automatic token exchange

---

*Last updated: 2026-01-12*
