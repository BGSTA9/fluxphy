# Maintainer: SOHEIL SANATI MOUTABAN <soheilsanatii@gmail.com>
pkgname=fluxphy
pkgver=0.1.0
pkgrel=1
pkgdesc="A file transfer tool with deep instrumentation into the physics of data flux"
arch=('x86_64')
url="https://github.com/BGSTA9/fluxphy"
license=('MIT')
depends=()
makedepends=('rust' 'cargo')
source=("$pkgname-$pkgver.tar.gz::https://github.com/BGSTA9/$pkgname/archive/v$pkgver.tar.gz")
sha256sums=('SKIP')

build() {
    cd "$pkgname-$pkgver"
    cargo build --release --locked
}

package() {
    cd "$pkgname-$pkgver"
    install -Dm755 "target/release/$pkgname" "$pkgdir/usr/bin/$pkgname"
    install -Dm644 LICENSE "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
}
