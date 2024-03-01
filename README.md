# nu_plugin_codegen

A plugin for generating code using Shopifys' Liquid files.

To install:

```
  cargo install --path .
```

To register (from inside Nushell):

```
  register <path to installed plugin>
```

Usage example:

Given a CSV file like

```
❯ open people.csv | take 4
╭───┬─────────────────┬─────────┬─────────────────────────────┬───────────────────┬─────────┬──────────┬──────────────╮
│ # │      name       │   id    │            email            │       role        │ manager │ nickname │ favorite_pet │
├───┼─────────────────┼─────────┼─────────────────────────────┼───────────────────┼─────────┼──────────┼──────────────┤
│ 0 │ Michael Brown   │ EMP7703 │ michael.brown@outlook.com   │ Data Analyst      │ false   │ Maverick │ Guinea Pig   │
│ 1 │ Linda Rodriguez │ EMP5810 │ linda.rodriguez@hotmail.com │ Designer          │ false   │ Virtuoso │ Hamster      │
│ 2 │ David Jones     │ EMP4896 │ david.jones@outlook.com     │ Software Engineer │ true    │ Maverick │ Turtle       │
│ 3 │ John Jackson    │ EMP8744 │ john.jackson@hotmail.com    │ Software Engineer │ false   │ Guru     │ Turtle       │
╰───┴─────────────────┴─────────┴─────────────────────────────┴───────────────────┴─────────┴──────────┴──────────────╯
```

and a .liquid file with some Python code

```
❯ cat python.liquid
───────┬───────────────────────────────────────────────────────────────────────────────────────────────────────────────
       │ File: python.liquid
───────┼───────────────────────────────────────────────────────────────────────────────────────────────────────────────
   1   │ #!/usr/bin/env python
   2   │
   3   │ def do_something(name, email):
   4   │   print("Hello " + name + ", Email: " + email)
   5   │
   6   │ def main():
   7   │   {%- for item in items %}
   8   │     do_something("{{ item.name }}", "{{ item.email }}")
   9   │   {%- endfor %}
  10   │
  11   │ if __name__ == "__main__":
  12   │   main()
───────┴────────────────────────────────────────────────────────────────────────────────────────────────────────────────
```

You can do this:

```
❯ open people.csv | take 4 | codegen python.liquid
#!/usr/bin/env python

def do_something(name, email):
  print("Hello " + name + ", Email: " + email)

def main():
    do_something("Michael Brown", "michael.brown@outlook.com")
    do_something("Linda Rodriguez", "linda.rodriguez@hotmail.com")
    do_something("David Jones", "david.jones@outlook.com")
    do_something("John Jackson", "john.jackson@hotmail.com")

if __name__ == "__main__":
  main()
```
