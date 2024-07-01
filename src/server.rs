type Result<T> = std::result::Result<T,Rejection>;
type Clients = Arc<Mutex<HashMap<String,Client>>>;



#[tokio::main]
async fn main (){

    let clients: Clients = Arc::new(Mutex::new(HashMap::new()));

    let health_route = warp:: path("register");
    let register_routes = register
    .and(warp::post())
    .and(warp::body_json())   
    .and(with_clients(clients.clone()))
    .and_then(handler::register_handler)
    .or(register
        .and(warp::delete())
        .and(warp::path::param())
        .and(with_clients(clients_clone()))
        .and_then(handler::unregister_handler));

    let publish = warp::path!("publish")
    .and(warp::body::json())
    .and(with_clients(clients_clone()))
    .and_then(handler::ws_handler);

    let routes = health_route
        .or(register_routes)
        .or(ws_route)
        .or(publish)
        .with(warp::cors().allow_any_origin);

        warp.serve(routes).run(([127,0,0,1], 8000)).await;
}

fn with_clients(clients:Clients)-> impl Filter<Extract = (Clients,), Error = Infalliable> + Clone{
    warp::any().map(move || clients.clone())
}