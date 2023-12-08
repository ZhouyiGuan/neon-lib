# neon-lib

## create project 
```sh
$ npm init neon neon-lib
```

## test performance btree find 
```sh
$ npm install
$ npm run build && node test/btree.js
```
tree find cost: 278.613ms

```sh
$ cargo test test_btree_find -- --nocapture
```
test time cost 263ms(rust)

## test memory use
```sh
$ npm install
$ npm run build && node test/buffer.js
```