use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

pub struct CoordinateSystem {
    name: String,

    chrs: Vec<(String, usize)>,
}

impl CoordinateSystem {
    pub fn chr_offsets(&self, padding: usize) -> Vec<(String, usize)> {
        let mut offset = 0;

        self.chrs
            .iter()
            .map(|(name, len)| {
                let offset_ = offset;
                offset += len + padding;
                (name.to_string(), offset_)
            })
            .collect()
    }

    pub fn chr_ranges(&self, padding: usize) -> Vec<(String, (usize, usize))> {
        let mut offset = 0;

        self.chrs
            .iter()
            .map(|(name, len)| {
                let start = offset;
                let end = offset + len;
                offset += len + padding;
                (name.to_string(), (start, end))
            })
            .collect()
    }

    pub fn chr_names(&self) -> impl Iterator<Item = &str> + '_ {
        self.chrs.iter().map(|(name, _)| name.as_str())
    }

    pub fn chrs(&self) -> &[(String, usize)] {
        &self.chrs
    }

    pub fn chr_len(&self, chr: &str) -> Option<usize> {
        let (_chr, len) = self.chrs.iter().find(|(name, _)| name == chr)?;
        Some(*len)
    }

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

        let mut chrs: Vec<(String, usize)> = Vec::new();

        for chr in chrs_array.iter() {
            let name = js_sys::Reflect::get(&chr, &"name".into())?;
            let name = name.as_string().unwrap();

            let len = js_sys::Reflect::get(&chr, &"len".into())?;
            let len = (len.as_f64().unwrap()) as usize;

            chrs.push((name, len));
        }

        Ok(Self { name, chrs })
    }
}
