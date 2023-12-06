cargo_component_bindings::generate!();

use bindings::wasi::http::types::{
    Headers, IncomingRequest, OutgoingBody, OutgoingResponse, ResponseOutparam,
};

struct Component;

impl bindings::exports::wasi::http::incoming_handler::Guest for Component {
    fn handle(_request: IncomingRequest, outparam: ResponseOutparam) {
        let hdrs = Headers::new();
        let resp = OutgoingResponse::new(hdrs);
        resp.set_status_code(200).expect("fail to set status code");
        let body = resp.body().expect("fail to get body");
        ResponseOutparam::set(outparam, Ok(resp));

        let out = body.write().expect("outgoing stream");
        out.blocking_write_and_flush(b"Hello\n")
            .expect("fail to write/flush");

        drop(out);
        OutgoingBody::finish(body, None).expect("fail to finish");
    }
}
