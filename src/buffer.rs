use neon::{
    prelude::*,
};


pub fn array_double(mut cx: FunctionContext) -> JsResult<JsArray> {
    let arr = cx.argument::<JsArray>(0)?;
    let len = arr.len(&mut cx);
    for i in 0..len {
        let val = arr.get_value(&mut cx, i)?
            .downcast_or_throw::<JsNumber, _>(&mut cx)?
            .value(&mut cx);
        let new_val = cx.number(val * 2 as f64);
        arr.set(&mut cx, i, new_val)?;
    }
    Ok(arr)
}

