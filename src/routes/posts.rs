use crate::{appstate::AppState, renderer::Render, templates};
use tide::{http::mime, Body, Redirect, Request, Response, StatusCode};

pub async fn get_posts(req: Request<AppState>) -> tide::Result {
    let state = &req.state();

    let blog = state.get_blog();

    let posts = blog
        .get_all_posts()
        .map(|key| state.get_blog().get_post(key).unwrap())
        .collect();

    let mut res = Response::new(StatusCode::Ok);
    res.render_html(|o| Ok(templates::posts(o, blog, posts)?))?;
    res.set_content_type(mime::HTML);

    Ok(res)
}

pub async fn get_post(req: Request<AppState>) -> tide::Result {
    let slug = req.param::<String>("slug")?;
    let normalized_slug = slug.to_lowercase();

    if slug != normalized_slug {
        return Ok(Redirect::permanent(format!("/posts/{}/", &normalized_slug)).into());
    }

    let state = &req.state();
    let blog = state.get_blog();

    if let Some(post) = blog.get_post(&slug) {
        let mut res = Response::new(StatusCode::Ok);
        res.render_html(|o| Ok(templates::post(o, blog, post)?))?;
        res.set_content_type(mime::HTML);
        return Ok(res);
    }

    let mut res = Response::new(StatusCode::NotFound);
    res.render_html(|o| Ok(templates::notfound(o)?))?;
    res.set_content_type(mime::HTML);
    Ok(res)
}

pub async fn redirect_trailing_slash(req: Request<AppState>) -> tide::Result {
    let slug = req.param::<String>("slug")?;
    Ok(Redirect::permanent(format!("/posts/{}/", &slug)).into())
}

pub async fn get_attachment(req: Request<AppState>) -> tide::Result {
    let slug = req.param::<String>("slug")?;
    let normalized_slug = slug.to_lowercase();

    if slug != normalized_slug {
        return Ok(Redirect::permanent(format!("/posts/{}/", &normalized_slug)).into());
    }

    let state = &req.state();
    let blog = state.get_blog();

    if let Some(post) = blog.get_post(&slug) {
        let attachement_name = req.param::<String>("attachment")?;
        println!("{}", attachement_name);
        if let Some(attachment) = post.get_attachment(&attachement_name) {
            let mut res = Response::new(StatusCode::Ok);
            res.insert_header("cache-control", "max-age=31536000"); // 1 year as second
            res.set_body(Body::from_file(&attachment.get_path()).await?);
            return Ok(res);
        }
    }

    let mut res = Response::new(StatusCode::NotFound);
    res.render_html(|o| Ok(templates::notfound(o)?))?;
    res.set_content_type(mime::HTML);
    Ok(res)
}
