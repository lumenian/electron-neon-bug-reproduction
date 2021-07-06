use neon::prelude::*;

pub fn start_task(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let callback = cx.argument::<JsFunction>(0)?.root(&mut cx);
    let queue = cx.queue();

    std::thread::spawn(move || {
        let result = 42;
        queue.send(move |mut cx| {
            let callback = callback.into_inner(&mut cx);
            let this = cx.undefined();
            let args = vec![cx.null().upcast::<JsValue>(), cx.number(result).upcast()];
            callback.call(&mut cx, this, args)?;
            Ok(())
        });
    });

    Ok(cx.undefined())
}

register_module!(mut m, {
    m.export_function("startTask", start_task)?;
    Ok(())
});