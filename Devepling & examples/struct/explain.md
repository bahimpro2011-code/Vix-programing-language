# Structs in Vix

## What is a Struct?

A struct is a way to group related data together under one name. Think of it as a container that holds multiple pieces of information about something. Each piece of information inside the struct is called a **field**.

## Struct Types

Vix has three types of structs:

- **Public** - Accessible from all files in your project
- **Private** - Only accessible within the current file (default)
- **Mutable** - Fields can be modified after creation (using `mut` keyword)

## Basic Syntax

```ruby
struct Example:
    mut field1: int32
    mut field2: str
end

example_struct: Example = Example(field1: 0, field2: "")
example_struct.field1 = 100
example_struct.field2 = "Hello world"
```

---

## Fields

Fields hold the data (information) about something. Each field has a name and a type.

### Simple Field Example

```ruby
struct Example:
    field1: int32  # Stores only integers (no decimals)
end

# Create an instance with initial value
example_struct = Example(field1: 0)

# Access and modify the field
example_struct.field1 = 100
```

---

## Why Use Structs?

Instead of having separate variables scattered throughout your code:

```ruby
title = "The Hobbit"
author = "J.R.R. Tolkien"
pages = 310
```

Structs let you group related data together:

```ruby
struct Book:
    title: str
    author: str
    pages: int
end

# Use them like this:
my_book = Book(
    title: "The Hobbit",
    author: "J.R.R. Tolkien",
    pages: 310
)

print(my_book.title)   # "The Hobbit"
print(my_book.author)  # "J.R.R. Tolkien"
print(my_book.pages)   # 310
```

---

## Mutable Fields

By default, struct fields are immutable (cannot be changed). Use the `mut` keyword to make fields mutable:

### Immutable Struct (Default)

```ruby
struct Book:
    title: str
    author: str
    pages: int
end

my_book = Book(
    title: "The Hobbit",
    author: "J.R.R. Tolkien",
    pages: 310
)

my_book.title = "Something new"  # Error: field is immutable
```

### Mutable Struct

```ruby
struct Book:
    mut title: str
    mut author: str
    mut pages: int
end

mut my_book = Book(
    title: "The Hobbit",
    author: "J.R.R. Tolkien",
    pages: 310
)

# Now you can modify the fields
my_book.title = "Something new"
my_book.author = "Someone else"
my_book.pages = 360
```

---

## Structs as Types

Structs can be used as custom return types in functions:

```ruby
struct CustomReturnType:
    message: str
    number: (int | float)
end

func example(): CustomReturnType
    return CustomReturnType(message: "Hello world", number: 105.3)
end

result = example()
print(result.message)  # "Hello world"
print(result.number)   # 105.3
```

---

## Public vs Private Structs

### Public Structs

Public structs are visible to all files in your project. Fields must also be marked as `pub` to be accessible.

**main.vix:**
```ruby
public struct Example:
    pub mut name: str
    pub mut age: int
end
```

**example.vix:**
```ruby
import Example from "src/main.vix"

example = Example(name: "Alice", age: 25)
example.name = "Bob"
example.age = 30
```

**Important:** Make sure fields are public by adding the `pub` keyword.

### Private Structs

Private structs (default) are only visible within their own file:

```ruby
struct Example:
    name: str
    age: int
end

# Only accessible in this file
```

---

## Struct vs Enum: When to Use Which?

### Use Structs When:

✅ You need to group **related data** together  
✅ All fields should exist at the same time  
✅ You're modeling something with multiple properties  
✅ You need to store and access multiple values together

**Example - Struct is perfect here:**
```ruby
struct Person:
    name: str
    age: int
    email: str
end

person = Person(name: "Alice", age: 30, email: "alice@example.com")
# A person always has a name, age, and email
```

### Use Enums When:

✅ You have a **fixed set of options**  
✅ A value can only be **one thing at a time**  
✅ You're modeling **states** or **variants**  
✅ Different options may carry different data

**Example - Enum is perfect here:**
```ruby
enum Status:
    Loading,
    Success(data: str),
    Error(message: str),
end

status = Status.Loading
# Status can only be ONE of these at a time
```

### Side-by-Side Comparison

| Feature | Struct | Enum |
|---------|--------|------|
| **Purpose** | Group related data | Represent one of several options |
| **Fields** | All fields exist together | Only one variant exists at a time |
| **Use Case** | Model entities (Person, Book) | Model states (Status, Action) |
| **Example** | `Book(title, author, pages)` | `Status.Loading` or `Status.Error` |

### Real-World Example

```ruby
# Struct - Groups user information together
struct User:
    name: str
    age: int
    status: UserStatus  # Uses enum below
end

# Enum - Represents ONE status at a time
enum UserStatus:
    Online,
    Offline,
    Away(reason: str),
    Busy,
end

# Usage
user = User(
    name: "Alice",
    age: 30,
    status: UserStatus.Online  # User can only have ONE status
)
```

**Key Difference:** A `User` struct has all its fields (name, age, status) at once, but the `status` field can only be ONE enum variant at a time.

### Common Mistake to Avoid

**❌ Don't use multiple booleans when you need an enum:**
```ruby
# WRONG - This allows impossible states (both online AND offline?)
struct UserStatus:
    is_online: bool
    is_offline: bool
    is_away: bool
end
```

**✅ Use an enum instead:**
```ruby
# CORRECT - User has exactly ONE status
enum UserStatus:
    Online,
    Offline,
    Away(reason: str),
end
```

---

## Summary

- **Structs** group related data together
- Use `mut` keyword for mutable fields
- Can be used as custom types in functions
- Use `public` and `pub` for cross-file access
- **Choose structs** when you need to store multiple related values together
- **Choose enums** when a value can only be one of several options at a time
