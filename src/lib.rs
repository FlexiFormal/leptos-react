pub mod context;
pub mod functions;
pub mod utils;

use crate::{
    context::LeptosContext,
    functions::{JsFunction0, JsFunction1, JsFunction2, JsFunction3, Void},
};
use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::HtmlDivElement;

#[cfg(feature = "typescript")]
#[wasm_bindgen(typescript_custom_section)]
const TS_CONT_FUN: &'static str =
    r#"export type LeptosContinuation = (e:HTMLDivElement,o:LeptosContext) => void;"#;
pub type LeptosContinuation = JsFunction2<HtmlDivElement, LeptosContext, Void>;

macro_rules! wrapper {
    ($tp:ident$(<$($a:ident: $atp:ident),*>)? @ $trt:ident = $js:ident) => {
        pub type $tp$( < $($atp),* > )? = $js< $($($atp),* ,)? Option<LeptosContinuation> >;
        impl$(< $($atp:Into<JsValue>),*  >)? $tp$(< $($atp),*  >)? {
            pub fn wrap<T:IntoView,F:FnOnce() -> T>(&self, $( $($a:$atp),* ,)? children:F) -> impl IntoView + use<T,F $(, $($atp),*)?> {
                use leptos::either::Either::{Left, Right};
                match self.apply($($($a),*)?) {
                    Ok(Some(cont)) => {
                        let owner = Owner::current()
                            .expect("Not in a leptos reactive context!")
                            .into();
                        let rf = NodeRef::new();
                        rf.on_load(move |elem| {
                            if let Err(err) = cont.apply(elem, owner) {
                                tracing::error!("Error calling continuation: {err}");
                            }
                        });
                        Left(view! {<div node_ref=rf>{children()}</div>})
                    }
                    Ok(None) => Right(children()),
                    Err(e) => {
                        tracing::error!("Error calling continuation: {e}");
                        Right(children())
                    }
                }
            }
            pub fn insert(&self, $( $($a:$atp),* )? ) -> impl IntoView + use<$( $($atp),*)?> {
                match self.apply($($($a),*)?) {
                    Ok(None) => None,
                    Err(e) => {
                        tracing::error!("Error calling insertion callback: {e}");
                        None
                    }
                    Ok(Some(cont)) => {
                        let ret = NodeRef::new();
                        let owner = Owner::current().expect("Not in a leptos reactive context!").into();
                        ret.on_load(move |e| {
                            if let Err(e) = cont.apply(e, owner) {
                                tracing::error!("Error calling continuation: {e}");
                            }
                        });
                        Some(view!(<div node_ref = ret/>))
                    }
                }
            }
        }
    }
}

wrapper!(ReactWrapper @ Wrappable = JsFunction0);
wrapper!(ReactWrapper1<a:Arg1> @ Wrappable1 = JsFunction1);
wrapper!(ReactWrapper2<a:Arg1,b:Arg2> @ Wrappable2 = JsFunction2);
wrapper!(ReactWrapper3<a:Arg1,b:Arg2,c:Arg3> @ Wrappable3 = JsFunction3);
