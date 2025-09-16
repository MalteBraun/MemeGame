#[macro_use]
extern crate diesel;
extern crate dotenv;

mod models;
mod schema;
mod db;

use actix_web::{
    web, App, HttpResponse, HttpServer, Responder, Result,
    http::header::LOCATION,
};
use rand::{Rng, distr::Alphanumeric};
use std::{collections::{HashSet, HashMap}, sync::Mutex};
use actix_web::web::Form;

#[derive(serde::Deserialize)]
struct JoinSessionForm {
    session_id: String,
}

#[derive(serde::Deserialize)]
struct AddPlayerForm {
    username: String,
}

// Shared app state for storing session IDs
struct AppState {
    sessions: Mutex<HashSet<String>>,
    players: Mutex<HashMap<String, HashSet<String>>>, // sessionID -> players set
}

//generate a unique random 5-char string not in sessions
fn generate_id(sessions: &mut HashSet<String>) -> String {
    loop {
        let id: String = rand::rng()
            .sample_iter(&Alphanumeric)
            .take(5)
            .map(char::from)
            .collect();

        if !sessions.contains(&id) {
            sessions.insert(id.clone());
            return id;
        }
    }
}

async fn main_page( data: web::Data<AppState>) -> impl Responder {
    let sessions = data.sessions.lock().unwrap();
    let sessions_list =sessions.iter()
        .map(|s| format!(r#"<li><a href="/{}">{}</a></li>"#, s, s))
        .collect::<String>();

    //let players = data.players.lock().unwrap();
    //let players_list = players.iter()
    //    .map(|p| format!(r#"<li>{}</a></li>"#,p))
    //    .collect::<String>();

    let html= format!( r#"
    <html>
        <head><title>Main Page</title></head>
        <body>
            <form method="post" action="/add_player">
                <input type="text" name="username" placeholder="Enter username" required />
                <button type="submit">Add Player</button>
            </form>
            <form method="post" action="/create_session">
                <button type="submit">Create Session</button>
            </form>
            <br/>
            <form method="post" action="/join_session">
                <input type="text" name="session_id" placeholder="Enter Session ID"/>
                <button type="submit">Join Session</button>
            </form>
            <h2>Active Sessions</h2>
            <ul>{sessions_list}</ul>
            <h2>Players</h2>
            
        </body>
    </html>
    "#, );
    HttpResponse::Ok().content_type("text/html").body(html)
}

// Handler to create a new session and redirect to it
async fn create_session( data: web::Data<AppState>) -> Result<HttpResponse> {
    let mut sessions = data.sessions.lock().unwrap();
    let session_id = generate_id(&mut sessions);
    let location = format!("/{}", session_id);
    Ok(HttpResponse::Found()
        .insert_header((LOCATION, location))
        .finish())
}

// Handler to join existing session by redirecting to session_id path
async fn join_session(form: Form<JoinSessionForm>, data: web::Data<AppState>) -> Result<HttpResponse> {
    let sessions= data.sessions.lock().unwrap();
    if sessions.contains(&form.session_id){
        let location = format!("/{}", form.session_id.trim());
        Ok(HttpResponse::Found()
            .insert_header((LOCATION, location))
            .finish())
    } else {
        Ok(HttpResponse::BadRequest()
            .body("Session ID does not exist")
        )
    }
}

// Handler for session page at /{session_id}
async fn session_page(path: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    let session_id = path.into_inner();
    let sessions = data.sessions.lock().unwrap();

    if sessions.contains(&session_id) {
        let html = format!(
            r#"
            <html>
                <head><title>Session {}</title></head>
                <body>
                    <h1>Session ID: {}</h1>
                    <p>You are now in the session.</p>
                    <a href="/">Back to main</a>
                </body>
            </html>
            "#,
            session_id, session_id
        );
        HttpResponse::Ok().content_type("text/html").body(html)
    } else {
        HttpResponse::NotFound().body("Session not found")
    }
}

// Handler for adding Players
async fn add_player(form: Form<AddPlayerForm>, data: web::Data<AppState>) -> Result<HttpResponse> {
    let mut players = data.players.lock().unwrap();
    
    Ok(HttpResponse::Found()
        .insert_header((LOCATION, "/"))
        .finish())
}

async fn add_player_to_session(data: web::Data<AppState>, session_id: &str, username: &str) -> bool {
    let mut players_map = data.players.lock().unwrap();
    let player_set = players_map.entry(session_id.to_string())
        .or_insert_with(HashSet::new);
    player_set.insert(username.to_string())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        sessions: Mutex::new(HashSet::new()),
        players: Mutex::new(HashMap::new())
    });

    println!("Starting server at http://localhost:8080");
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/", web::get().to(main_page))
            .route("/create_session", web::post().to(create_session))
            .route("/join_session", web::post().to(join_session))
            .route("/{session_id}", web::get().to(session_page))
            .route("/add_player", web::post().to(add_player))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}