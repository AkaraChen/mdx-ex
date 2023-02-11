#![deny(clippy::all)]

use napi::bindgen_prelude::ToNapiValue;

#[napi]
pub enum JsxRuntime {
  Automatic,
  Classic,
}

#[derive(Clone)]
#[napi(object)]
pub struct Opts {
  pub provider_import_source: Option<String>,
  pub jsx_runtime: Option<JsxRuntime>,
  pub jsx_import_source: Option<String>,
  pub pragma: Option<String>,
  pub pragma_frag: Option<String>,
  pub pragma_import_source: Option<String>,
  pub filepath: Option<String>,
}

impl Opts {
  pub fn to_mdxjs_options(self) -> mdxjs::Options {
    return mdxjs::Options {
      provider_import_source: self.provider_import_source,
      jsx_runtime: self.jsx_runtime.map(|v| match v {
        JsxRuntime::Automatic => mdxjs::JsxRuntime::Automatic,
        JsxRuntime::Classic => mdxjs::JsxRuntime::Classic,
      }),
      jsx_import_source: self.jsx_import_source,
      pragma: self.pragma,
      pragma_frag: self.pragma_frag,
      pragma_import_source: self.pragma_import_source,
      filepath: self.filepath,
      ..Default::default()
    };
  }
}

#[napi]
pub fn compile(mdx: String, user_opts: Option<Opts>) -> Result<String, napi::Error> {
  let ops = match user_opts {
    Some(v) => v.to_mdxjs_options(),
    None => Default::default(),
  };
  match mdxjs::compile(mdx.as_str(), &ops) {
    Ok(v) => {
      return Ok(v);
    }
    Err(e) => {
      return Err(napi::Error::new(napi::Status::Unknown, e));
    }
  }
}

#[napi(constructor)]
pub struct Processor {
  pub opts: Opts,
}

#[napi]
impl Processor {
  #[napi]
  pub fn process(&self, mdx: String) -> Result<String, napi::Error> {
    return compile(mdx, Some(self.opts.clone()));
  }
}
