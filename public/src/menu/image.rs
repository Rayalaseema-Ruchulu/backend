use worker::{Request, Response, Result, RouteContext};

const IDLI: &[u8] = include_bytes!("test_images/idli.jpg");
const KADAPA_GHEE_KARAM_DOSA: &[u8] = include_bytes!("test_images/Kadapa Ghee Karam Dosa.jpg");
const KADAPA_KARAM_DOSA: &[u8] = include_bytes!("test_images/Kadapa Karam Dosa.jpg");
const MASALA_DOSA: &[u8] = include_bytes!("test_images/Masala Dosa.png");
const ONION_DOSA: &[u8] = include_bytes!("test_images/Onion Dosa.jpg");
const ONION_MASALA_DOSA: &[u8] = include_bytes!("test_images/Onion Masala Dosa.jpg");
const PLAIN_DOSA: &[u8] = include_bytes!("test_images/Plain Dosa.png");

pub async fn get_image(_req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let id = ctx.param("id").unwrap().parse::<usize>().unwrap();

    Ok(
        Response::builder().body(worker::ResponseBody::Body(match id {
            2 => PLAIN_DOSA.to_vec(),
            3 => ONION_DOSA.to_vec(),
            4 => MASALA_DOSA.to_vec(),
            5 => ONION_MASALA_DOSA.to_vec(),
            6 => KADAPA_KARAM_DOSA.to_vec(),
            7 => KADAPA_GHEE_KARAM_DOSA.to_vec(),
            _ => IDLI.to_vec(),
        })),
    )
}

pub async fn get_thumbnail(_req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let id = ctx.param("id").unwrap().parse::<usize>().unwrap();

    Ok(
        Response::builder().body(worker::ResponseBody::Body(match id {
            2 => PLAIN_DOSA.to_vec(),
            3 => ONION_DOSA.to_vec(),
            4 => MASALA_DOSA.to_vec(),
            5 => ONION_MASALA_DOSA.to_vec(),
            6 => KADAPA_KARAM_DOSA.to_vec(),
            7 => KADAPA_GHEE_KARAM_DOSA.to_vec(),
            _ => IDLI.to_vec(),
        })),
    )
}
