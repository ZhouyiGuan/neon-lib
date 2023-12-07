use neon::prelude::*;

mod btree;
use btree::*;

mod buffer;
use buffer::*;

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("btreeNew", btree_new)?;
    cx.export_function("btreeNewRandom", btree_new_random)?;
    cx.export_function("btreeInsert", btree_insert)?;
    cx.export_function("btreeFind", btree_find)?;
    cx.export_function("btreeRemove", btree_remove)?;

    cx.export_function("arrayDouble", array_double)?;
    Ok(())
}
