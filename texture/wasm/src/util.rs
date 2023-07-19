use std::{
    future::Future,
    pin::Pin,
    task::{ Context, Poll },
};
use wasm_bindgen::{
    prelude::*,
    JsCast,
};
use web_sys::HtmlImageElement;

pub struct ImageLoader {
    image: Option<HtmlImageElement>,
}

impl ImageLoader {
    pub fn new(path: &str) -> Self {
        let img = HtmlImageElement::new().unwrap();
        img.set_src(path);

        Self {
            image: Some(img),
        }
    }
}

impl Future for ImageLoader {
    type Output = Result<HtmlImageElement, String>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match &self.image {
            Some(image) => {
                if image.complete() {
                    Poll::Ready(Ok(self.image.take().unwrap()))
                } else {
                    let waker = cx.waker().clone();
                    let on_load_closure = Closure::once(move || {
                        waker.wake_by_ref();
                    });
                    image.set_onload(Some(on_load_closure.as_ref().unchecked_ref()));
                    on_load_closure.forget();

                    Poll::Pending
                }
            },
            _ => Poll::Ready(Err(String::from("Failed to load image"))),
        }
    }
}
