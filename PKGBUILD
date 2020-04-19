pkgname=study
pkgver=1.0.0
pkgrel=1
pkgdesc="A study helper for music theory written in Rust"
arch=(x86_64)
depends=('gtk3')
makedepends=('cargo')
license=("custom")
build() {
   cargo build --release --locked --all-features
}
package() {
  install -Dm 755 ../target/release/${pkgname} -t "${pkgdir}/usr/bin"
  install -Dm 755 ../${pkgname}.desktop -t "${pkgdir}/usr/share/applications"
  install -Dm 755 ../${pkgname}.png "${pkgdir}/usr/share/icons/hicolor/256x256/apps/${pkgname}.png"
  install -Dm 755 ../${pkgname}_locolor.png "${pkgdir}/usr/share/icons/hicolor/32x32/apps/${pkgname}.png"
}
