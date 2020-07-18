use crate::{appstate::AppState, renderer::RenderBuilder, templates};
use tide::{Body, Redirect, Request, Response, StatusCode};

pub async fn get_posts(req: Request<AppState>) -> tide::Result {
    let state = &req.state();

    let blog = state.get_blog();

    let posts = blog
        .get_all_posts()
        .map(|key| state.get_blog().get_post(key).unwrap())
        .collect();

    let res = Response::builder(StatusCode::Ok)
        .render_html(|o| Ok(templates::posts(o, blog, posts)?))?
        .build();

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
        let res = Response::builder(StatusCode::Ok)
            .render_html(|o| Ok(templates::post(o, blog, post)?))?
            .build();
        return Ok(res);
    }

    let res = Response::builder(StatusCode::NotFound)
        .render_html(|o| Ok(templates::notfound(o)?))?
        .build();
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
            let res = Response::builder(StatusCode::Ok)
                .header("cache-control", "max-age=31536000") // 1 year as second
                .body(Body::from_file(&attachment.get_path()).await?)
                .build();
            return Ok(res);
        }
    }

    let res = Response::builder(StatusCode::NotFound)
        .render_html(|o| Ok(templates::notfound(o)?))?
        .build();
    Ok(res)
}
