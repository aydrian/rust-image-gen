use http::StatusCode;
use lambda_runtime::{handler_fn, Context, Error};
use og_image_writer::{style, writer::OGImageWriter};
use serde_json::{json, Value};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let handler_fn = handler_fn(handler);
    lambda_runtime::run(handler_fn).await?;
    Ok(())
}

async fn handler(_: Value, _: Context) -> Result<Value, Error> {
    let text = "This is Open Graphic Image Writer for Web Developer.";

    let mut writer = OGImageWriter::new(style::WindowStyle {
        width: 1200,
        height: 630,
        background_color: Some(style::Rgba([70, 40, 90, 255])),
        align_items: style::AlignItems::Center,
        justify_content: style::JustifyContent::Center,
        ..style::WindowStyle::default()
    })
    .unwrap();

    let font = Vec::from(include_bytes!("../fonts/Mplus1-Black.ttf") as &[u8]);

    writer
        .set_text(
            text,
            style::Style {
                margin: style::Margin(0, 20, 0, 20),
                line_height: 1.8,
                font_size: 100.,
                word_break: style::WordBreak::Normal,
                color: style::Rgba([255, 255, 255, 255]),
                text_align: style::TextAlign::Start,
                ..style::Style::default()
            },
            font,
        )
        .expect("couldn't set text");

    writer.paint().expect("coulnd't paint");

    let data = writer.into_vec().unwrap();

    Ok(json!({
        "headers": {
            "Content-Type": "image/png",
            "Content-Length": data.len().to_string()
        },
        "statusCode": StatusCode::OK.as_u16(),
        "body": base64::encode(data),
        "isBase64Encoded": true
    }))
}
