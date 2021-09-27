# Maintainer: sioodmy <a.sokolowski06@gmail.com>
pkgname=todo-bin
pkgver=2.0
pkgrel=1
pkgdesc="Super fast and simple tasks organizer written in rust"
url="https://github.com/sioodmy/todo"
license=('GPL')
conflicts=('todo-git')
depends=()
makedepends=()
arch=("x86_64")
source=("https://github.com/sioodmy/todo/releases/download/${pkgver}/todo"
		"https://raw.githubusercontent.com/sioodmy/todo/master/LICENSE")

package() {
	mkdir -p ${pkgdir}/usr/bin
	mkdir -p ${pkgdir}/usr/share/licenses/${pkgname}

	install -Dm 755 ${srcdir}/rsfetch ${pkgdir}/usr/bin/todo
	install -Dm 644 ${srcdir}/LICENSE "${pkgdir}/usr/share/licenses/${pkgname}/LICENSE"
}

