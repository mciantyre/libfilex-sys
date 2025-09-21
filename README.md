# libfilex-sys

libfilex-sys will _eventually_ provide Rust bindings for [FileX], the ThreadX
FAT file system. But rght now, the package only builds the FileX C sources.

This package depends on [libthreadx-sys]. You should be familiar with that
package before using this package.

[FileX]: https://github.com/eclipse-threadx/filex
[libthreadx-sys]: https://github.com/mciantyre/libthreadx-sys

This package may not yet build for all ports supported by libthreadx-sys.

## License

libfilex-sys is MIT licensed. See [LICENSE](./LICENSE) for more information.

libfilex-sys includes FileX soure code from the `filex` directory. See
`filex/LICENSE.txt` for the source's MIT license.
