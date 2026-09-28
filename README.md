# seeker

CLI BMS library manager

## Usage

```
seeker help
```

## Features

 - Support bmson.
 - Support archive file. (zip, 7z, rar)
 - Support multiple encoding. (shift-jis, big5, euc-kr)
 - Support detect encoding.
 - Manage difficulty table.
 - Export bmt to beatoraja. (sort use beatoraja-config)
 - Open in STELLAVERSE IR or BMS Score Viewer.

## Configuration

There is a configuration file left in `~/.config/seeker/config.json`

```
{
    "directory": "<path to BMS directory>",
    "library": "<path to database file>",
    "beatoraja": "<path to beatoraja directory (option)>"
}
```

## Import

Import supports BMS/BMSON files, directories, and archives.

A directory should contain the files belonging to a single song. Multiple songs must not be mixed directly in the same directory.

### Importable tree

A song directory may contain multiple charts and related files:

```
bms/
└── song_a/
    ├── song_a.bms
    ├── song_a_hyper.bms
    ├── song_a_another.bms
    ├── song_a.bmson
    ├── audio.ogg
    └── image.png
```

An archive containing a song directory is also supported:

```
song_a.zip
└── song_a/
    ├── song_a.bms
    ├── song_a_another.bms
    ├── audio.ogg
    └── image.png
```

Multiple song directories can exist under the library directory:

```
bms/
├── song_a/
│   ├── song_a.bms
│   └── audio.ogg
├── song_b/
│   ├── song_b.bmson
│   └── audio.ogg
└── song_c/
    ├── song_c.bms
    └── audio.ogg
```

### Unimportable tree

Multiple songs must not be mixed directly in the same directory:

```
bms/
└── songs/
    ├── song_a.bms
    ├── song_a_another.bms
    ├── song_b.bms
    ├── song_b_another.bms
    ├── audio_a.ogg
    └── audio_b.ogg
```

An archive containing archives is also not supported:

```
songs.zip
├── song_a.zip
├── song_b.zip
└── song_c.zip
```

The same applies to deeper nesting:

```
songs.zip
└── songs/
    ├── song_a.zip
    └── song_b.zip
```

Archives should contain the files for a song directly, rather than other archives.

## Credits

[rib2bit/BeMusicSeeker](https://tumblr.ribbit.xyz/post/129562866015/bemusicseeker-%E6%AD%A3%E5%BC%8F%E7%89%88%E3%82%92%E5%85%AC%E9%96%8B%E3%81%97%E3%81%BE%E3%81%97%E3%81%9F-v034) — respect

[MikuroXia/bms-rs](https://github.com/MikuroXina/bms-rs) — BMS format parser

## License

Licensed under either of:

- [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)
- [MIT license](http://opensource.org/licenses/MIT)

at your option.
