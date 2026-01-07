# Enums in Vix

## What is an Enum?

An enum (enumeration) is a type that contains a limited number of named constants. Each constant has a different name and can optionally store associated data. An enum represents a value that can be **one of several options**, but only **one at a time**.

## Enum Types

Vix has two types of enums:

- **Public** - Accessible from all files in your project
- **Private** - Only accessible within the current file (default)

## Basic Syntax

```ruby
enum Color:
    Red,
    Green,
    Blue,
end

color = Color.Red
```

---

## Why Use Enums?

Instead of using raw numbers or strings that are hard to understand:

```ruby
status = 1  # What does 1 mean?
status = 2  # What does 2 mean?
status = 3  # What does 3 mean?
```

Use enums for clarity and safety:

```ruby
enum Status:
    Loading,
    Success,
    Error,
end

status = Status.Loading
status = Status.Success
status = Status.Error
```

### Benefits of Enums

- **Cleaner code** - Easier to read and understand
- **Type safety** - Compiler catches mistakes
- **Better maintainability** - Changes are easier to track
- **Self-documenting** - Code explains itself

---

## Simple Enums

Simple enums represent a fixed set of options without additional data:

```ruby
enum Color:
    Red,
    Green,
    Blue,
end

blue = Color.Blue
green = Color.Green
red = Color.Red
```

### Real-World Example

```ruby
enum Direction:
    North,
    South,
    East,
    West,
end

player_direction = Direction.North
```

---

## Enums with Associated Data

Each enum variant can store different types of data:

```ruby
enum Player:
    Move(x: int32, y: int32),
    Speed(speed: usize),
    Position(x: int32, y: int32),
    Avatar(skin_color: (int32, int32, int32), height: usize, weight: usize),
end

# Create instances with data
move = Player.Move(x: 10, y: 20)
speed = Player.Speed(speed: 10)
position = Player.Position(x: 0, y: 0)
avatar = Player.Avatar(
    skin_color: (255, 200, 150),
    height: 180,
    weight: 75
)
```

### Why Different Data Per Variant?

Different variants need different information:

```ruby
enum HttpResponse:
    Success(data: str, status_code: int),
    NotFound(url: str),
    ServerError(error_message: str, error_code: int),
    Redirect(new_url: str),
end

# Each variant carries the data it needs
response = HttpResponse.Success(data: "Hello", status_code: 200)
error = HttpResponse.NotFound(url: "/missing-page")
```

---

## Pattern Matching with Enums

Enums work perfectly with pattern matching to handle different cases:

```ruby
enum Action:
    Quit,
    Join(game: str),
    Read(data: any),
    Write(data: any),
end

action = Action.Join(game: "Minecraft")

match action:
    case Action.Quit do
        print("Player: Quit")
    end
    
    case Action.Join(game) do
        print("Player: Joined " + game)
    end
    
    case Action.Read(data) do
        print("Player: Read Data")
    end
    
    case Action.Write(data) do
        print("Player: Write Data")
    end
end
```

### Extracting Data from Enums

```ruby
enum Result:
    Success(value: int),
    Failure(error: str),
end

result = Result.Success(value: 42)

match result:
    case Result.Success(value) do
        print("Got value: " + value)  # Prints: Got value: 42
    end
    
    case Result.Failure(error) do
        print("Error: " + error)
    end
end
```

---

## Public vs Private Enums

### Public Enums

Public enums are visible to all files in your project.

**main.vix:**
```ruby
public enum Action:
    Quit,
    Join(game: str),
    Read(data: any),
    Write(data: any),
end
```

**example.vix:**
```ruby
import Action from "src/main.vix"

action = Action.Join(game: "Chess")

match action:
    case Action.Quit do
        print("Player: Quit")
    end
    
    case Action.Join(game) do
        print("Player: Joined " + game)
    end
    
    case Action.Read(data) do
        print("Player: Read Data")
    end
    
    case Action.Write(data) do
        print("Player: Write Data")
    end
end
```

### Private Enums

Private enums (default) are only visible within their own file:

```ruby
enum Action:
    Quit,
    Join(game: str),
end

# Only accessible in this file
```

---

## Enum vs Struct: When to Use Which?

### Use Enums When:

✅ You have a **fixed set of options**  
✅ A value can only be **one thing at a time**  
✅ You're modeling **states** or **variants**  
✅ Different options may carry different data  
✅ You need **pattern matching** on different cases

**Example - Enum is perfect here:**
```ruby
enum OrderStatus:
    Pending,
    Processing(worker_id: int),
    Shipped(tracking_number: str),
    Delivered(date: str),
    Cancelled(reason: str),
end

order_status = OrderStatus.Shipped(tracking_number: "ABC123")
# An order can only have ONE status at a time
```

### Use Structs When:

✅ You need to group **related data** together  
✅ All fields should exist at the same time  
✅ You're modeling something with multiple properties  
✅ You need to store and access multiple values together

**Example - Struct is perfect here:**
```ruby
struct Order:
    id: int
    customer: str
    items: [str]
    total: float
end

order = Order(id: 1, customer: "Alice", items: ["Book"], total: 29.99)
# An order always has ALL these properties
```

### Side-by-Side Comparison

| Feature | Enum | Struct |
|---------|------|--------|
| **Purpose** | Represent one of several options | Group related data |
| **Values** | Only one variant at a time | All fields exist together |
| **Use Case** | Model states (Status, Action) | Model entities (Person, Book) |
| **Example** | `Status.Loading` or `Status.Error` | `Book(title, author, pages)` |
| **Pattern Matching** | Perfect for matching cases | Not designed for this |

### Real-World Example

```ruby
# Enum - Order can only be in ONE state at a time
enum OrderStatus:
    Pending,
    Shipped(tracking: str),
    Delivered,
    Cancelled,
end

# Struct - Order has ALL these properties at once
struct Order:
    id: int
    customer: str
    status: OrderStatus  # Uses enum above
    total: float
end

# Usage
order = Order(
    id: 123,
    customer: "Alice",
    status: OrderStatus.Shipped(tracking: "XYZ789"),
    total: 99.99
)
```

**Key Difference:** An `Order` struct has all its fields at once, but the `status` field can only be ONE enum variant at a time.

### Common Mistake to Avoid

**❌ Don't use a struct when you need an enum:**
```ruby
# WRONG - This suggests an order could have multiple statuses
struct OrderStatus:
    is_pending: bool
    is_shipped: bool
    is_delivered: bool
end
```

**✅ Use an enum instead:**
```ruby
# CORRECT - An order has exactly ONE status
enum OrderStatus:
    Pending,
    Shipped(tracking: str),
    Delivered,
end
```

---

## Summary

- **Enums** represent a value that can be one of several options
- Only **one variant** is active at a time
- Perfect for **states, actions, and choices**
- Excellent with **pattern matching**
- Use `public` for cross-file access
- **Choose enums** when a value can only be one of several options at a time
- **Choose structs** when you need to store multiple related values together
