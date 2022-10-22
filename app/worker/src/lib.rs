use serde_json::json;
use worker::*;

mod utils;

fn log_request(req: &Request) {
    console_log!(
        "{} - [{}], located at: {:?}, within: {}",
        Date::now().to_string(),
        req.path(),
        req.cf().coordinates().unwrap_or_default(),
        req.cf().region().unwrap_or("unknown region".into())
    );
}

pub async fn form_handler(req: &mut Request, ctx: RouteContext<()>) -> Result<Response> {
    if let Some(name) = ctx.param("field") {
        let form = req.form_data().await?;
        match form.get(name) {
            Some(FormEntry::Field(value)) => return Response::from_json(&json!({ name: value })),
            Some(FormEntry::File(_)) => {
                return Response::error("`field` param in form shouldn't be a File", 422);
            }
            None => return Response::error("Bad Request", 400),
        }
    }

    Response::error("Bad Request", 400)
}

pub struct LandingPage {
    ctx: worker::Context,
    env: Env,
    req: Request,
}

impl LandingPage {
    pub fn new(ctx: worker::Context, env: Env, req: Request) -> Self {
        Self { ctx, env, req }
    }
    pub fn log_request(&self) -> &Self {
        log_request(&self.req);
        self
    }
    pub fn panic_hook(&self) -> &Self {
        utils::set_panic_hook();
        self
    }
    pub async fn router(&mut self) -> Router<()> {
        Router::new()
            .get("/", |_, _| Response::ok("Hello from Workers!"))
            .get("/docs/research", |_, _| Response::redirect(
                Url::parse("https://link.storjshare.io/s/jxaq4elovwoufwt3y4kzg4hm6nrq/scsys/lib/documents/research/")?
            ))
            .post_async("/form/:field", |mut req, ctx| async move {
                form_handler(&mut req, ctx).await
            })
            .get("/worker-version", |_, ctx| {
                let version = ctx.var("WORKERS_RS_VERSION")?.to_string();
                Response::ok(version)
            })
    }
    pub async fn runner(&mut self) -> Result<Response> {
        let req = self.req.clone()?;
        let env: Env = self.env.clone().into();
        self.router().await.run(req, env).await
    }
}

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    let mut page = LandingPage::new(_ctx, env, req);
    page.log_request();
    page.panic_hook();
    page.runner().await
}
