use std::fmt::Display;
use std::thread;
use std::time::Instant;

use headless_chrome::protocol::target::methods::CreateTarget;
use image::imageops::overlay;
use image::load_from_memory;
use image::DynamicImage;
use image::GenericImageView;
use image::Luma;
use qrcode::QrCode;

use anyhow::{anyhow, Result};
use headless_chrome::{protocol::page::ScreenshotFormat, Browser, LaunchOptionsBuilder};

fn url2image(url: &str) -> Result<DynamicImage> {
    let start = Instant::now();

    fn to_anyhow(e: impl Display) -> anyhow::Error {
        anyhow!(e.to_string())
    }

    let browser = Browser::new(
        LaunchOptionsBuilder::default()
            .window_size(Some((1200, 1600))) // A4 size
            .build()
            .unwrap(),
    )
    .map_err(to_anyhow)?;

    let tab = browser.wait_for_initial_tab().map_err(to_anyhow)?;
    let viewport = tab
        .navigate_to(url)
        .map_err(to_anyhow)?
        .wait_until_navigated()
        .map_err(to_anyhow)?
        .find_element("body")
        .map_err(to_anyhow)?
        .get_box_model()
        .map_err(to_anyhow)?
        .margin_viewport();

    dbg!(&viewport);

    let tab = browser
        .new_tab_with_options(CreateTarget {
            url,
            width: Some(viewport.width as i32),
            height: Some(viewport.height as i32 + 10),
            browser_context_id: None,
            enable_begin_frame_control: None,
        })
        .map_err(to_anyhow)?;

    let data = tab
        .capture_screenshot(ScreenshotFormat::PNG, None, true)
        .map_err(to_anyhow)?;

    println!(
        "time spent on url2image : {} ms",
        start.elapsed().as_millis()
    );
    Ok(load_from_memory(&data)?)
}

fn gen_qrcode(url: &str) -> Result<DynamicImage> {
    let start = Instant::now();
    let code = QrCode::new(url.as_bytes())?;
    let image = code.render::<Luma<u8>>().build();

    println!(
        "time spent on gen_qrcode : {} ms",
        start.elapsed().as_millis()
    );
    Ok(DynamicImage::ImageLuma8(image))
}

fn do_overlay(bottom: &mut DynamicImage, top: &DynamicImage) {
    let start = Instant::now();
    let x = bottom.width() - top.width() - 10;
    let y = bottom.height() - top.height() - 10;

    println!(
        "time spent on do_overlay: {} ms",
        start.elapsed().as_millis()
    );
    overlay(bottom, top, x, y);
}

pub fn web2image(url: &str, output: &str) -> Result<()> {
    let url = url.to_owned();
    let url1 = url.clone();

    let bottom_handle = thread::spawn(move || url2image(&url).unwrap());

    // let mut bottom = url2image(url)?;
    let qrcode_handle = thread::spawn(move || gen_qrcode(&url1).unwrap());

    // let qrcode = gen_qrcode(url)?;
    let mut bottom = bottom_handle.join().unwrap();
    let qrcode = qrcode_handle.join().unwrap();
    do_overlay(&mut bottom, &qrcode);
    let start = Instant::now();
    // bottom.save_with_format(output, image::ImageFormat::Png)?;
    bottom.save(output)?;
    println!(
        "time spent on save_with_format: {} ms",
        start.elapsed().as_millis()
    );
    Ok(())
}
