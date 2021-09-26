# This is an example PKGBUILD file. Use this as a start to creating your own,
# and remove these comments. For more information, see 'man PKGBUILD'.
# NOTE: Please fill out the license field for your package! If it is unknown,
# then please put 'unknown'.

# Maintainer: sioodmy <a.sokolowski06@gmail.com>
pkgname=todo-git
pkgver=1.0
pkgrel=1
pkgdesc="Simple and super fast task organizer written in rust under 200 sloc"
arch=(x86_64 i686)
url="https://github.com/sioodmy/todo.git"
license=('GPL')
makedepends=(cargo git)
source=("git+$url")
md5sums=(SKIP)

build() {
	cd todo
	cargo build --release
}

package() {
	mkdir -p "${pkgdir}/usr/bin"
	cd todo
	install -Dm 755 "target/release/todo" "${pkgdir}/usr/bin/todo"
}
