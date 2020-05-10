pkg_name=distributed_inspec_notifier
pkg_origin=lancewf
pkg_version="0.1.0"
pkg_maintainer="Lance Finfrock <lancewf@gmail.com>"
pkg_license=("Apache-2.0")
pkg_bin_dirs=(bin)

pkg_deps=(
  core/glibc
  core/gcc-libs
)

pkg_build_deps=(
  core/rust
  core/gcc
  core/pkg-config
  core/make
)

do_build() {
  pushd "${PLAN_CONTEXT}/.."
    cp -r src ${CACHE_PATH}/.
    cp Cargo.toml ${CACHE_PATH}/.
  popd
}

do_install() {
  cargo install --root "${pkg_prefix}" --path "${CACHE_PATH}" --verbose
}
