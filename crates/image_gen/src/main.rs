use lamedh_http::{
    http::StatusCode,
    lambda::{lambda, Context, Error},
    IntoResponse, Request, Response,
};
use playwright::Playwright;

#[lambda(http)]
#[tokio::main]
async fn main(_request: Request, _: Context) -> Result<impl IntoResponse, Error> {
    let playwright = Playwright::initialize().await.unwrap();
    playwright.install_chromium().unwrap(); // Install browsers
    let chromium = playwright.chromium();
    let browser = chromium.launcher().headless(true).launch().await.unwrap();
    let context = browser.context_builder().build().await.unwrap();
    let page = context.new_page().await.unwrap();
    page.set_viewport_size(playwright::api::Viewport {
        width: 1200,
        height: 630,
    })
    .await
    .unwrap();
    page.set_content_builder("<html><body><h1>Hello World</h1><p>This is a test</p></body></html>")
        .set_content()
        .await
        .unwrap();
    /*page.add_script_tag_builder(&format!(
        "window.image = \"{}\";\nwindow.username = \"{}\";",
        &res.avatar_url, &res.login
    ))
    .add_script_tag()
    .await
    .unwrap();*/
    let data = page.screenshot_builder().screenshot().await.unwrap();

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "image/png")
        .header("Content-Length", data.len().to_string())
        .body(base64::encode(data))
        .unwrap())
}
