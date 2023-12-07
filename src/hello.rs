use super::*;


pub fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    let s: Handle<JsString> = cx.string("hello from rust");
    Ok(s)
}