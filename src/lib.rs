use worker::*;
mod content;
mod utils;

#[event(fetch)]
pub async fn main(_req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
  utils::set_panic_hook();
  Router::new()
    .get_async("/", |_req, _ctx| content::serve(_req, _ctx))
    .get_async("/:tttt", |_req, _ctx| content::serve(_req, _ctx))
    .get_async("/post/:bbbb", |_req, _ctx| content::serve_post(_req, _ctx))
    .post_async("/blog_post", |_req, _ctx| content::post(_req, _ctx))
    .run(_req, env).await
}