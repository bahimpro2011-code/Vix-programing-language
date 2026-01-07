# Vix Programming Language Documentation

## Overview

Vix is a modern programming language designed to balance performance, memory safety, and developer productivity. It combines the speed of C with the safety features of modern languages, making it suitable for both low-level and high-level application development.

**Current Status:** Alpha version  
**Discord Community:** https://discord.gg/CAemjRc4ya

---

## Key Features

### Performance
- Compiles to C, then to LLVM using Clang compiler
- Achieves C-level performance
- Supports C/C++ library integration
- Uses SSA (Static Single Assignment) for optimization
- Clang -O3 optimizations enabled by default

### Memory Safety
- Immutable variables by default
- No `null` values - uses `None` instead
- Built-in `Option`, `Result`, `Some`, and `Ok` types
- Compile-time memory checks
- Optional unsafe blocks for performance-critical code

### Developer Experience
- Simple syntax using `end` keyword instead of braces
- Beginner-friendly design
- Over 40 built-in functions
- Comprehensive standard library
- Easy C/C++ library integration

---

## Language History

Vix was created by the Vix Development Team under the leadership of **MrBatata**. The project began in early 2024 and was officially released to the public on January 1, 2026.

**Key Team Members:**
- **MrBatata** - Project Leader and Founder
- **Kat** - Co-owner
- **Masashi** - Manager and Compiler Expert

The language went through six different compiler iterations during its two-year pre-alpha development phase before reaching its current alpha release.

---

## Type System

### Basic Types

Vix provides simple, memorable type keywords:

- `int` - Integer types (int8, int16, int32, int64, int128)
- `uint` - Unsigned integers (uint8, uint16, uint32, uint64, uint128)
- `float` - Floating-point types (float32, float64, float128)
- `char` - Character types
- `str` - String type
- `bool` - Boolean type
- `maybe` - Optional type
- `any` - Dynamic type

### Advanced Types

- `Option[T]` - Represents a value that may or may not exist
- `Result[T, E]` - Represents success or failure
- `ptr*` - Pointer type
- Tuples and Arrays
- Structs and Enums

---

## Memory Safety Features

### Immutable by Default

Variables are immutable unless explicitly marked as mutable:

```rust
// Immutable variable (default)
x: int32 = 10

// Mutable variable
mut y: int32 = 20
y += 5
```

### Option and Result Types

Handle potential absence of values safely:

```rust
// Option type - value may not exist
i: Option[int32] = None
x: Option[int32] = Some(30)

// Result type - operation may succeed or fail
func example(): Result[Option[int32], str]
    return Ok((100, "Hello world"))
end
```

### Unsafe Blocks

For performance-critical code that requires manual memory management:

```rust
unsafe:
    mut i = 0
    
    forever do
        i += 10
    end
    
    // Raw pointer operations allowed
    mut x = ptr* uint8
    free(x)
end

// Unsafe functions
unsafe func high_performance_operation():
    // No overflow checks or borrow checking
end
```

**Important:** Unsafe blocks disable overflow checks and memory safety guarantees. Use only when necessary for maximum performance.

---

## Compilation Process

### Compilation Pipeline

```
Source Code → Vix Compiler → C IR → Clang Compiler → Binary
```

The Vix compiler acts as a middle-end that:
1. Performs memory safety checks
2. Generates optimized C intermediate representation (IR)
3. Passes code to Clang for final compilation

### Compiler Architecture

The compiler is written in Rust and follows this flow:

1. **Lexer** - Tokenizes source code
2. **Parser** - Generates statements and expressions
3. **Type Checker** - Validates type correctness
4. **AST Generation** - Creates abstract syntax tree
5. **Code Generator** - Produces C IR code
6. **Clang** - Compiles to optimized binary

---

## Library System

### Installing Libraries

```bash
# Install a library
package install "library_name"

# Install specific version
package install "library_name" --version

# Check installed libraries
package check --library

# Check library version
package check -library_name -version

# Uninstall a library
package uninstall "library_name"
```

### Using Libraries

```ruby
# Import entire library
import library_name

# Import specific items
import something from library

# Import C/C++ libraries
import library.h
```

### Library Types

**1. Binary Compilation**
- Library compiled into application binary
- Larger executable size
- Easier to develop
- Supports `.bin` files from any language

**2. IR Generation**
- Library generates intermediate representation
- Smaller executable size
- More complex development

---

## Creating Your Own Library

### Basic Setup

```bash
# Create new library
package new "your_library_name"
```

This generates the following structure:

```
your_library/
├── src/
│   └── main.vix
├── syntax/
│   ├── error.js
│   ├── warning.js
│   └── syntax.js
└── package.json
```

### Example Library (Vix)

**main.vix:**
```ruby
func print_from_my_lib(s: str)
    print(s)
end

struct MyLibrary:
    name = str
    health = int
end

impl MyLibrary:
    MyLibrary(
        name = "",
        health = 0
    )

    func add(&mut self, input_name: str, input_health: int)
        self.name += input_name
        self.health += input_health
    end

    func return_data(&self): Result[str, Option[int]]
        return Ok((self.name, self.health))
    end
end
```

**Usage:**
```ruby
import MyLibrary

func main()
    print_from_my_lib("Hello, world")

    MyLibrary.add("player 1", 100)
    data = MyLibrary.return_data()
end
```

### Example Library (C)

**package.json:**
```json
{
    "package": {
        "name": "example_library",
        "version": "1.3.5",
        "include": {
            "Clang": "1.18.8"
        }
    }
}
```

**main.c:**
```c
#include <stdio.h>

int print_from_my_lib(char* s) {
    printf("%s", s);
    return 0;
}
```

### Compiling Libraries

```bash
# Compile your library
package compile

# Upload to marketplace (requires account)
package upload
```

---

## Multi-Language Support

Vix can integrate libraries from any language that generates `.bin` files:

### Example: Rust + Vix Library

**lib.rs:**
```rust
fn add(i: i32, x: i32) -> i32 {
    return i + x;
}
```

**main.vix:**
```ruby
func remove(i: int32, x: int32): int
    return i - x
end
```

**package.json:**
```json
{
    "package": {
        "name": "mixed_library",
        "version": "1.3.5",
        "include": {
            "RustC": "1.10.10"
        }
    }
}
```

**Usage:**
```ruby
import mixed_library

a = add(1, 3)      // From Rust
b = remove(1, 5)   // From Vix

print(a, b)
```

---

## Installing Clang (Required)

### Windows

1. Download MSYS2 from https://www.msys2.org/
2. Install and open MSYS2 terminal
3. Update packages:
   ```bash
   pacman -Syu
   ```
4. Install Clang:
   - **64-bit:** `pacman -S mingw-w64-x86_64-clang`
   - **32-bit:** `pacman -S mingw-w64-i686-clang`

### Linux

**Arch Linux / Manjaro:**
```bash
sudo pacman -S clang
```

**Fedora:**
```bash
sudo dnf install clang
```

**Ubuntu / Debian:**
```bash
sudo apt update
sudo apt install clang
```

### macOS

```bash
xcode-select --install
```

---

## Safe vs Unsafe Code

### Safe Blocks (Default)

- Automatic memory management
- Overflow checks enabled
- Ownership and borrowing enforced
- Uses `alloc` for allocations
- Slightly slower but memory-safe

### Unsafe Blocks

- Raw pointer operations allowed
- No overflow checks
- No ownership/borrowing enforcement
- Manual memory management required
- Maximum performance

**Use unsafe blocks only when:**
- Building operating systems
- Implementing low-level drivers
- Optimizing performance-critical code
- Interfacing with hardware

---

## Example Programs

### Hello World

```ruby
func main(): int
    print("Hello, world!")
    return 0
end
```

### Windows Message Box

```ruby
func main(): int
    extern "stdcall" from "user32.dll":
        func MessageBoxA(s: int, title: const str, text: const str, x: int)
    end

    MessageBoxA(0, "Test", "Hello, world", 0)
    return 0
end
```

### Struct with Implementation

```ruby
struct Player:
    name = str
    health = int
    score = int
end

impl Player:
    Player(
        name = "",
        health = 100,
        score = 0
    )

    func take_damage(&mut self, damage: int)
        self.health -= damage
    end

    func add_score(&mut self, points: int)
        self.score += points
    end

    func is_alive(&self): bool
        return self.health > 0
    end
end

func main(): int
    mut player = Player("Hero", 100, 0)
    player.take_damage(20)
    player.add_score(100)
    
    if player.is_alive() do
        print("Player is still alive!")
    end
    
    return 0
end
```

---

## Performance Considerations

### Optimization Levels

Vix uses Clang's optimization flags:
- `-O3` - Maximum optimization (default)
- Inline function expansion
- Loop unrolling
- Dead code elimination

### Memory Management

**Safe Mode:**
- Uses `alloc` for allocations
- Automatic memory tracking
- Bounds checking enabled
- Type size: int32 for arrays, usize for strings

**Unsafe Mode:**
- Raw pointers only
- No bounds checking
- Manual memory management
- Uses unions for performance

---

## Security Features

### Library Security

All libraries in the Vix marketplace undergo security checks to prevent malware. However, users should remain vigilant.

**If you discover malware in a library:**
- Report immediately at: https://vix.dev/report
- Avoid using suspicious libraries
- Check library source code when possible

**Warning:** Creating or distributing malicious libraries may result in:
- Account termination
- Legal consequences

---

## Advanced Features

### Generic Types

```ruby
func swap<T>(mut a: T, mut b: T)
    temp = a
    a = b
    b = temp
end
```

### Pattern Matching

```ruby
result: Result[int, str] = get_value()

match result do
    Ok(value) do print("Success: ", value),
    Err(error) do print("Error: ", error),
end
```

### External Function Calls

```ruby
extern "C" from "mylib.dll":
    func external_function(x: int, y: int): int
end
```

---

## Resources

### Official Links

- **Website:** https://vix.dev
- **Documentation:** https://vix.dev/information
- **Help Center:** https://vix.dev/help
- **Marketplace:** https://vix.dev/marketplace
- **Discord:** https://discord.gg/CAemjRc4ya
- **Report Issues:** https://vix.dev/report

### Learning Resources

- Prompting Guide: https://docs.vix.dev/prompt-engineering
- Memory Safety: https://vix.dev/help/memory_safety
- Library Installation: https://vix.dev/help/library_installing
- Built-in Functions: https://vix.dev/information/buildinfunctions
- Benchmarks: https://vix.dev/information/benchmarks

---

## Community and Support

### Getting Help

1. **Discord Server:** Join the community for real-time support
2. **Documentation:** Comprehensive guides and tutorials
3. **GitHub:** Report bugs and contribute
4. **Email Support:** Contact the development team

### Contributing

Vix welcomes contributions from the community:
- Report bugs and issues
- Submit feature requests
- Create libraries and tools
- Improve documentation
- Share your projects

---

## Roadmap

The alpha version receives daily updates including:
- Bug fixes
- New features
- Performance improvements
- Library additions
- Documentation updates

Stay tuned for more exciting features coming soon!

---

## License and Legal

**Important Notices:**
- Vix is currently in alpha - expect breaking changes
- Libraries must comply with security guidelines
- Malicious code distribution is prohibited
- Clang compiler required for compilation

---

*Documentation Version: Alpha 1.0*  
*Last Updated: January 2026*  
*© 2024-2026 Vix Development Team*
