use super::*;
use std::{
    collections::BTreeMap,
    cell::RefCell,
    ops::Bound::{Included, Unbounded},
};


type BoxedTestStruct = JsBox<RefCell<MyBTree>>;

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
    pub fn find_nearest_key(&self, key: &i64) -> Option<i64> {
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

pub fn btree_new(mut cx: FunctionContext) -> JsResult<BoxedTestStruct> {
    let new_test_struct = RefCell::new(MyBTree::new());
    Ok(cx.boxed(new_test_struct))
}

pub fn btree_insert(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let test_struct = cx.argument::<BoxedTestStruct>(0)?;
    let mut test_struct = test_struct.borrow_mut();
    let key = cx.argument::<JsNumber>(1)?.value(&mut cx) as i64;
    let value = cx.argument::<JsNumber>(2)?.value(&mut cx) as i64;
    test_struct.insert(key, value);
    Ok(cx.undefined())
}

pub fn btree_find(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let test_struct = cx.argument::<BoxedTestStruct>(0)?;
    let test_struct = test_struct.borrow();
    let key = cx.argument::<JsNumber>(1)?.value(&mut cx) as i64;
    let value: i64 = test_struct.find(key).unwrap();
    Ok(cx.number(value as f64))
}

pub fn btree_remove(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let test_struct = cx.argument::<BoxedTestStruct>(0)?;
    let mut test_struct = test_struct.borrow_mut();
    let key = cx.argument::<JsNumber>(1)?.value(&mut cx) as i64;
    test_struct.remove(key);
    Ok(cx.undefined())
}
