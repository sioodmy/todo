# todo

A lightweight and super fast cli todo program written in rust under 200 sloc

![gif](todo.gif)

## installation

[AUR package](https://aur.archlinux.org/packages/todo-bin/): `todo-bin`

### Nix Flake

Add `todo.url = "github:sioodmy/todo";` to your inputs. And `inputs.todo.packages."x86_64-linux".todo` to `home.packages`

### other distros

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
        adds new task/s with an optional description
        Example: todo add shopping
        What is the goal of your task shopping? "buy carrots"
    - list
        lists all tasks
        Example: todo list
    - done [INDEX]
        marks task as done
        Example: todo done 2 3 (marks second and third tasks as completed)
    - rm [INDEX]
        removes a task
        Example: todo rm 4
    - reset
        deletes all tasks
    - restore
        restore recent backup after reset
    - sort
        sorts completed and uncompleted tasks
        Example: todo sort
    - raw [todo/done]
        prints nothing but done/incompleted tasks in plain text, useful for scripting
        Example: todo raw done
```
