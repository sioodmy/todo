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
