# neon-lib

create project 
```sh
$ npm init neon neon-lib
```

test performance btree find 
```sh
$ npm install
$ npm run build && node test/btree.js
```
```sh
$ cargo test test_btree_find -- --nocapture
```

test memory use
```sh
$ npm install
$ npm run build && node test/buffer.js
```