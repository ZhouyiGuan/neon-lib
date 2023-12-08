use neon::prelude::*;
use std::{
    collections::BTreeMap,
    cell::RefCell,
    ops::Bound::{Included, Unbounded},
    time::{Duration, Instant},
};
use rand::Rng;


type BoxedMyBTree = JsBox<RefCell<MyBTree>>;

pub struct MyBTree {
    map: BTreeMap<i64, i64>,
}

impl Finalize for MyBTree {}

impl MyBTree {
    pub fn new() -> Self {
        Self {
            map: BTreeMap::new(),
        }
    }

    pub fn insert(&mut self, key: i64, value: i64) {
        self.map.insert(key, value);
    }

    pub fn find(&self, key: i64) -> Option<i64> {
        self.map.get(&key)
            .copied()
            .or_else(|| self.find_nearest_key(&key))
    }

    pub fn remove(&mut self, key: i64) {
        self.map.remove(&key);
    }

    fn find_nearest_key(&self, key: &i64) -> Option<i64> {
        let mut lower = self.map.range((Unbounded, Included(key))).rev();
        let mut upper = self.map.range((Included(key), Unbounded));

        let lower_pair = lower.next();
        let upper_pair = upper.next();
        match (lower_pair, upper_pair) {
            (Some((lower_key, lower_value)), Some((upper_key, upper_value))) => {
                if (key-lower_key) < (upper_key-key) {
                    return Some(lower_value.to_owned());
                } else {
                    return Some(upper_value.to_owned());
                }
            },
            (Some((_, lower_value)), None) => {
                return Some(lower_value.to_owned());
            },
            (None, Some((_, upper_value))) => {
                return Some(upper_value.to_owned());
            },
            (None, None) => {
                return None;
            },
        }
    }
}

pub fn btree_new(mut cx: FunctionContext) -> JsResult<BoxedMyBTree> {
    let new_btree = RefCell::new(MyBTree::new());
    Ok(cx.boxed(new_btree))
}

pub fn btree_new_random(mut cx: FunctionContext) -> JsResult<BoxedMyBTree> {
    let btree_size = cx.argument::<JsNumber>(0)?.value(&mut cx) as i64;
    let new_btree = RefCell::new(MyBTree::new());
    let mut rng = rand::thread_rng();
    for _ in 0..btree_size {
        let random_num = rng.gen_range(0..(btree_size*10));
        new_btree.borrow_mut().insert(random_num, random_num);
    }
    Ok(cx.boxed(new_btree))
}

pub fn btree_insert(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let btree = cx.argument::<BoxedMyBTree>(0)?;
    let mut btree = btree.borrow_mut();
    let key = cx.argument::<JsNumber>(1)?.value(&mut cx) as i64;
    let value = cx.argument::<JsNumber>(2)?.value(&mut cx) as i64;
    btree.insert(key, value);
    Ok(cx.undefined())
}

pub fn btree_find(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let btree = cx.argument::<BoxedMyBTree>(0)?;
    let btree = btree.borrow();
    let key = cx.argument::<JsNumber>(1)?.value(&mut cx) as i64;
    let value: i64 = btree.find(key).unwrap();
    Ok(cx.number(value as f64))
}

pub fn btree_remove(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let btree = cx.argument::<BoxedMyBTree>(0)?;
    let mut btree = btree.borrow_mut();
    let key = cx.argument::<JsNumber>(1)?.value(&mut cx) as i64;
    btree.remove(key);
    Ok(cx.undefined())
}

#[test]
fn test_btree_find() {
    let btree_size = 1_000_000;
    let key_range = btree_size * 10;
    let test_num = 100_000;

    let mut btree = MyBTree::new();
    let mut rng = rand::thread_rng();
    for _ in 0..btree_size {
        let random_num = rng.gen_range(0..key_range);
        btree.map.insert(random_num, random_num);
    }

    let start = Instant::now();
    for i in 0..test_num {
        let random_key = rng.gen_range(0..key_range);
        let result = btree.find(random_key).unwrap();
        if i % 1_000 == 0 {
            println!("Find result for key {}: {}", random_key, result);
        }
    }
    let duration = start.elapsed().as_millis();
    println!("test time cost {}ms", duration);
}
