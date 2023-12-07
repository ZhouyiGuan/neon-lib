use neon::prelude::*;

mod btree;
use btree::*;

mod hello;
use hello::*;

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("hello", hello)?;
    cx.export_function("btreeNew", btree_new)?;
    cx.export_function("btreeInsert", btree_insert)?;
    cx.export_function("btreeFind", btree_find)?;
    cx.export_function("btreeRemove", btree_remove)?;
    Ok(())
}
