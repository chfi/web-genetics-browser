use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

pub struct CoordinateSystem {
    name: String,

    chr_names: Vec<String>,
    chr_lens: Vec<usize>,
}

impl CoordinateSystem {
    pub async fn fetch_and_parse(url: &str) -> Result<Self, JsValue> {
        use wasm_bindgen_futures::JsFuture;
        use web_sys::{Request, RequestInit, Response};

        let window = web_sys::window().unwrap();

        let mut opts = RequestInit::new();
        opts.method("GET");

        // TODO handle errors correctly
        let request = Request::new_with_str_and_init(&url, &opts).unwrap();

        let resp_value = JsFuture::from(window.fetch_with_request(&request))
            .await
            .unwrap();

        let resp: Response = resp_value.dyn_into().unwrap();
        let json = JsFuture::from(resp.json().unwrap()).await.unwrap();

        Self::parse_js(json)
    }

    pub fn parse_js(obj: JsValue) -> Result<Self, JsValue> {
        let name = js_sys::Reflect::get(&obj, &"name".into())?;
        let name = name.as_string().unwrap();

        let chrs = js_sys::Reflect::get(&obj, &"chrs".into())?;
        let chrs_array: js_sys::Array = chrs.dyn_into()?;
        // let chrs_array: js_sys::Array = chrs.dyn_into().ok().unwrap();

        let mut chr_names: Vec<String> = Vec::new();
        let mut chr_lens: Vec<usize> = Vec::new();

        for chr in chrs_array.iter() {
            let name = js_sys::Reflect::get(&obj, &"name".into())?;
            let name = chr.as_string().unwrap();

            let len = js_sys::Reflect::get(&obj, &"len".into())?;
            let len = (len.as_f64().unwrap()) as usize;

            chr_names.push(name);
            chr_lens.push(len)
        }

        Ok(Self {
            name,
            chr_names,
            chr_lens,
        })
    }
}
