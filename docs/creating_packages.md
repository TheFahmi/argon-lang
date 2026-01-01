# Membuat Package Cryo

## 1. Buat Project Baru

```bash
apm init my-package
cd my-package
```

## 2. Struktur Package

```
my-package/
├── cryo.toml      # Manifest (WAJIB)
├── lib/
│   └── lib.ar      # Kode library utama
├── src/
│   └── main.ar     # Contoh penggunaan (opsional)
├── tests/
│   └── test.ar     # Tests
└── README.md       # Dokumentasi
```

## 3. Edit cryo.toml

```toml
[package]
name = "my-package"
version = "1.0.0"
description = "Deskripsi singkat package Anda"
author = "Nama Anda <email@example.com>"
license = "MIT"
repository = "https://github.com/username/my-package"
keywords = ["cryo", "utility"]

[dependencies]
# Tambahkan dependencies jika ada
# other-pkg = "1.0.0"
```

## 4. Tulis Kode Library

**lib/lib.ar:**
```cryo
// ============================================
// My Package - Utility functions
// ============================================

fn greet(name) {
    return "Hello, " + name + "!";
}

fn add(a, b) {
    return a + b;
}

fn multiply(a, b) {
    return a * b;
}
```

## 5. Buat Repository di GitHub

1. Buka https://github.com/new
2. Nama repo: `my-package` (sama dengan nama di cryo.toml)
3. Pilih Public
4. Klik "Create repository"

## 6. Push ke GitHub

```bash
git init
git add .
git commit -m "Initial release v1.0.0"
git branch -M main
git remote add origin https://github.com/username/my-package.git
git push -u origin main
```

## 7. Buat Release Tag

```bash
# Menggunakan APM
apm publish

# Atau manual:
git tag v1.0.0
git push origin v1.0.0
```

## 8. Daftarkan ke Registry (Opsional)

Untuk mendaftarkan package ke central registry, edit `registry/index.json`:

```json
{
  "packages": {
    "my-package": {
      "description": "Deskripsi package",
      "repository": "https://github.com/username/my-package",
      "versions": {
        "1.0.0": {
          "commit": "abc123...",
          "dependencies": {}
        }
      },
      "latest": "1.0.0"
    }
  }
}
```

Lalu buat Pull Request ke repo Cryo.

---

## Cara Menggunakan Package

### Dari Git (Langsung)

```bash
# Tanpa registry
apm add username/my-package --git

# Dengan versi spesifik
apm add username/my-package@v1.0.0 --git
```

### Dari Registry

```bash
# Jika sudah didaftarkan di registry
apm add my-package
```

### Import di Kode

```cryo
import "deps/my-package/lib/lib.cryo";

fn main() {
    print(greet("World"));
    print(add(2, 3));
    return 0;
}
```

---

## Tips

1. **Versioning**: Gunakan Semantic Versioning (MAJOR.MINOR.PATCH)
   - MAJOR: Breaking changes
   - MINOR: Fitur baru (backward compatible)
   - PATCH: Bug fixes

2. **Nama Package**: Gunakan lowercase dengan dash (my-package)

3. **Testing**: Selalu sertakan tests di folder `tests/`

4. **README**: Dokumentasikan cara install dan penggunaan

5. **License**: Sertakan file LICENSE (MIT, Apache-2.0, dll)
