// #[get("/auth/google")]
// fn google_login() -> Redirect {
//     let client_id = "your_client_id";
//     let redirect_uri = "your_redirect_uri";
//     let oauth_url = format!(
//         "https://accounts.google.com/o/oauth2/v2/auth?client_id={}&redirect_uri={}&response_type=code&scope=email",
//         client_id, redirect_uri
//     );
//     Redirect::to(oauth_url)
// }

// #[get("/auth/google/callback?code=<code>")]
// async fn google_callback(code: String, db: DbConn) -> Result<Redirect, Status> {
//     // Exchange code for token
//     // Verify token and get user info
//     // Check if user exists in DB, if not create a new one
//     // Log the user in (create session or JWT token)
//     Ok(Redirect::to("/"))
// }
