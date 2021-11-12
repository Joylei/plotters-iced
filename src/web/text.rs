// plotters-iced
//
// Iced backend for Plotters
// Copyright: 2021, Joylei <leingliu@gmail.com>
// License: MIT

use crate::utils::RotateAngle;
use plotters_backend::BackendTextStyle;
use std::cell::RefCell;
use wasm_bindgen::JsValue;

const PLACEHOLDER: &'static str = "plotters-iced-placeholder";

pub(crate) struct Measurement {
    inner: RefCell<Inner>,
}

impl Measurement {
    #[inline]
    pub fn new() -> Self {
        Self {
            inner: RefCell::new(Inner {
                div: None,
                lru: lru::LruCache::new(100),
            }),
        }
    }

    #[inline]
    pub fn measure<S: BackendTextStyle>(
        &self,
        text: &str,
        style: &S,
    ) -> Result<(u32, u32), JsValue> {
        let mut inner = self.inner.borrow_mut();
        inner.ensure_div()?;
        inner.measure(text, style)
    }
}

struct Inner {
    div: Option<web_sys::Element>,
    lru: lru::LruCache<std::string::String, (u32, u32)>,
}

impl Inner {
    fn measure<S: BackendTextStyle>(
        &mut self,
        text: &str,
        style: &S,
    ) -> Result<(u32, u32), JsValue> {
        let angle = style.transform().angle().unwrap_or_default();
        let key = format!(
            "{}{}{}{}{}",
            style.size(),
            style.family().as_str(),
            style.style().as_str(),
            angle,
            text
        );
        if let Some(v) = self.lru.get(&key) {
            return Ok(*v);
        }
        let style = format!(
            "border:0;padding:0;margin:0;position:fixed;left:-10000px;\
            display:block;width:auto;height:auto;z-index:-100;\
            font-size:{}px;font-family:{};font-style:{};\
            transform: rotate({}deg);",
            style.size(),
            style.family().as_str(),
            style.style().as_str(),
            angle
        );
        let div = self.div.as_ref().unwrap();
        div.set_attribute("style", &style)?;
        div.set_text_content(Some(text));
        let rect = div.get_bounding_client_rect();
        let size = (rect.width().ceil() as u32, rect.height().ceil() as u32);
        //super::log(&format!("{},{}:{}", size.0, size.1, text));
        self.lru.put(key, size);
        Ok(size)
    }

    #[inline]
    fn ensure_div(&mut self) -> Result<(), JsValue> {
        if self.div.is_none() {
            let document = web_sys::window()
                .map(|w| w.document())
                .flatten()
                .ok_or_else(|| JsValue::from("unable to get document"))?;
            let body = document
                .body()
                .ok_or_else(|| JsValue::from("unable to get body"))?;
            self.div = document.get_element_by_id(PLACEHOLDER);
            if self.div.is_none() {
                let div = document.create_element("div")?;
                div.set_attribute(
                    "style",
                    "border:0;padding:0;margin:0;display:none;z-index:-100;",
                )?;
                body.append_child(&div)?;
                self.div = Some(div);
            }
        }
        Ok(())
    }
}

impl Drop for Inner {
    #[inline]
    fn drop(&mut self) {
        if let Some(div) = self.div.take() {
            web_sys::window()
                .map(|w| w.document())
                .flatten()
                .map(|doc| doc.body())
                .flatten()
                .map(|body| body.remove_child(&div));
        }
    }
}
