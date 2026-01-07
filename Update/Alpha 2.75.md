### Update Alpha: 2.75:
In this update a big bugs found in 2.5v version and this update fix all bugs:
- Union:
    - Union was not being parserd by the parser that what generate "_error_" in the codegen IR:
    ```ruby
        i: (int32 | str)[] = [40, 30, "Hello world"]
    ```
- Bug fixed: Before when you do:
    ```ruby
        func red(input: str): str
            return something
        end

        i: str = i.red
        # or
        i: str = i.red()
    ```
    - it generate:
    ```c
        typedef struct { char* ptr; int64_t len; } String;

            String vix_main() {
            String t0 = { .ptr = "Hello", .len = 5 };
            String var_i = t0;
            int32_t t1 = _error_; 
        }
    ```
    - This bug has been fixed and now it generate:
    ```c
        typedef struct { char* ptr; int64_t len; } String;

            String vix_main() {
            String t0 = { .ptr = "Hello", .len = 5 };
            String var_i = t0;
            int32_t t1 = red(t0)
        }

    ```
