# Functions in Vix

## What is a Function?

A function is like a recipe inside your program—a named block of code that performs a specific task. Instead of writing the same logic repeatedly, you write it once, give it a name, and then "call" it by that name whenever you need it.

## Function Types

Vix has three types of functions:

- **Public** - Accessible from all files in your project
- **Private** - Only accessible within the current file (default)
- **Unsafe** - Used for operations that bypass memory safety (see memory safety documentation)

**Note:** The `main()` function is optional. If used, it will be the first function called automatically.

## Basic Syntax

```ruby
func function_name(parameter_one: Type, parameter_two: Type): ReturnType
    # your code goes here
    return something
end

# Call the function:
function_name(value1, value2)
```

---

## Parameters

Parameters hold the input values for your function. Each parameter has a name and a type.

### Example Function Definition

```ruby
func example_function(input1: str, input2: int)
    # input1 accepts only strings
    # input2 accepts only integers
end
```

### Correct Function Call

```ruby
example_function("Hello", 67)
# "Hello" is a string ✓
# 67 is an integer ✓
```

### Incorrect Function Call

```ruby
example_function(103, "Hello")
# 103 is an integer but parameter 1 expects a string ✗
# "Hello" is a string but parameter 2 expects an integer ✗
```

### Compiler Error Message

```
[Error]: Wrong param:
    |
1   | func example_function(input1: str, input2: int)
    |                       ^^^^^^^^^^^^^^^^^^^^^^^^
    |
3   | example_function(103, "Hello")
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    -> Information:
        | Calling a function with wrong input:
            func example_function(input1: str, input2: int)
        | The parameters expect (str, int) but received (int, str)
        -> Help:
            example_function("Hello", 103)
        -> Note:
            When modifying the function call, ensure the variables 
            inside the function are handled correctly.
```

---

## Parameter Ownership Types

Vix is a memory-safe language with multiple parameter ownership types:

### 1. Owned (Default)

The function takes full ownership of the data. The original variable cannot be used afterward. Data is immutable inside the function.

```ruby
func example(input1: str)
    print(input1)
    input1 += " world"  # Error: input1 is immutable
end

data: str = "Hello"
example(data)  # Ownership transferred to function
print(data)    # Error: data is no longer accessible
```

### 2. Mutable Owned

The function takes ownership and can modify the data inside the function scope.

```ruby
func example(mut input1: str)
    input1 += " world"  # Now "Hello world"
    print(input1)
end

data: str = "Hello"
example(data)  # Ownership transferred
print(data)    # Error: data is no longer accessible
```

### 3. Reference

The function reads the data without taking ownership. Data is immutable.

```ruby
func example(input1: ref str)
    print(input1)
    input1 += " world"  # Error: immutable reference
end

data: str = "Hello"
example(data)  # Function just reads the data
print(data)    # Works fine, prints "Hello"
```

### 4. Mutable Reference

The function can modify the data without taking ownership. Changes persist outside the function.

```ruby
func example(input1: mut ref str)
    input1 += " world"  # Modifies the original data
    print(input1)       # Prints "Hello world"
end

data: str = "Hello"
example(data)
print(data)    # Prints "Hello world"
```

### 5. Borrow

The function borrows the data temporarily and promises to return it. Data is immutable.

```ruby
func example(input1: brw str)  # or &str
    print(input1)
    input1 += " world"  # Error: borrowed data is immutable
end

data: str = "Hello"
example(data)  # Data is borrowed
print(data)    # Works fine, prints "Hello"
```

### 6. Mutable Borrow

The function borrows the data and can modify it. Changes persist outside the function.

```ruby
func example(input1: mut brw str)  # or mut &str
    input1 += " world"  # Modifies the data
    print(input1)
end

data: str = "Hello"
example(data)
print(data)    # Prints "Hello world"
```

---

## Return Types

Return types specify what kind of value a function can return.

### Basic Return Type

```ruby
func example(): int32
    return 10           # Correct
    return "Hello"      # Error: wrong return type
end
```

### Default Return Type

By default, all functions have an `int32` return type if not specified.

```ruby
func example()
    return 10  # Implicitly returns int32
end
```

### Automatic Return Type Detection

When returning arrays or tuples, the compiler automatically detects the return type:

```ruby
func example()
    return (10, 50)
end

# Automatically compiles to:
func example(): (int32, int32)
    return (10, 50)
end
```

### Using Result Type

The `Result` type is recommended for functions that may fail. Use `Ok()` to return successful values.

```ruby
func example(): Result[int32, str]
    return Ok(10)
end
```

### Using Option Type

Use `Option[]` for values that might be `None`:

```ruby
func example(): Option[int32]
    return Some(10)
    # or return None
end
```

---

## Public vs Private Functions

### Public Functions

Public functions are visible to all files in your project.

**main.vix:**
```ruby
public func example(i: int32): int32
    return i
end
```

**another_script.vix:**
```ruby
import example from "src/main.vix"

example(10)  # Works fine
```

### Private Functions

Private functions (default) are only visible within their own file.

**another_main.vix:**
```ruby
func example(i: int32): int32
    return i
end
```

**another_script.vix:**
```ruby
import example from "src/another_main.vix"
# Error: Function is private and cannot be imported

example(10)  # Error: Unknown function
```

---

## Summary

- Functions help you organize and reuse code
- Use appropriate ownership types to control how data is accessed
- Specify return types to ensure type safety
- Use `public` keyword to share functions across files
- Leverage `Result` and `Option` types for robust error handling
