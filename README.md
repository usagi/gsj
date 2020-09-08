# GSJ

Implementation of "Nishioka-Nagatu-2015 method" of "Geological Survey of Japan" (GSJ).

## Features

- Altitude Tile; (ja: 標高タイル)
    - Nishioka-Nagatsu-2015 method (ja: 西岡-長津-2015年法)
        - [x] Decoder
        - [x] Encoder

## Examples or Tests

- To see: [./tests/test.rs](./tests/test.rs)

## Reference

- <https://www.jstage.jst.go.jp/article/geoinformatics/26/4/26_155/_article/-char/en>; en-US
    - <https://www.jstage.jst.go.jp/article/geoinformatics/26/4/26_155/_article/-char/ja>; ja-JP
- <https://www.gsj.jp/>; ja-JP

### See also

- <https://maps.gsi.go.jp/development/demtile.html>; ja-JP

## License

- [MIT](LICENSE)

### GSI files in tests/ dir

These files are GSI's DEM tile. It's for only to use unit tests, no need essentially.

1. tests/gsi-dem-z8-x229-y94.png; from <https://cyberjapandata.gsi.go.jp/xyz/dem_png/8/229/94.png>
2. tests/gsi-dem-z8-x229-y94.txt; from <https://cyberjapandata.gsi.go.jp/xyz/dem/8/229/94.txt>

- <https://www.gsi.go.jp/kikakuchousei/kikakuchousei40182.html>; ja-JP

## Author

- USAGI.NETWORK / Usagi Ito <https://usagi.network/>
