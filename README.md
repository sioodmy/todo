# todo
A lightweight and super fast cli todo program written in rust under 200 sloc

![gif](todo.gif)
## installation
[AUR package](https://aur.archlinux.org/packages/todo-bin/): `todo-bin`

use `cargo build --release` to compile todo and copy `target/release/todo` to `/usr/bin`
## note
todo is still really early in development so be careful or sth

btw i know that my code is not the best but im still learing 
## usage
```Usage: todo [COMMAND] [ARGUMENTS]
Todo is a super fast and simple tasks organizer written in rust
Example: todo list
Available commands:
    - add [TASK/s]
        adds new task/s
        Example: todo add "buy carrots"
    - list
        lists all tasks
        Example: todo list
    - done [INDEX]
        marks task as done
        Example: todo done 2 3 (marks second and third tasks as completed)
    - rm [INDEX]
        removes a task
        Example: todo rm 4
    - sort
        sorts completed and uncompleted tasks
        Example: todo sort
    - raw [todo/done]
        prints nothing but done/incompleted tasks in plain text, useful for scripting
        Example: todo raw done
```
