pub async fn register_handler(body: RegisterRequest, clients: Clients) -> Result<impl Reply> {
    let user_id = body.user_id;
    let uuid = Uuid::new_v4().simple().to_string();

    register_client(uuid.clone(), user_id, clients ).await;

    Ok(json(&RegisterResponse{

        url: format!("ws://127:0.0.1:8000/ws/{}", uuid),
    }))
}


// Registering

async fn register_client(id: String, user_id: usize, clients: Clients){

    Clients.lock().await.insert(
        id, 
        Client{
            user_id,
            topics: vec![String::from("cats")],
            sender: None,
        }
    )
}

pub fn health_handler() ->  impl Future<Output = Result<impl Reply>>{
    futures:: future::ready(Ok(StatusCode::Ok))
}

//De-register
pub async fn unregister_handler(id: String, clients: Clients)-> Result<impl Reply> {
    clients.lock().await.remove(&id);
    Ok(StatusCode::Ok)
}


pub async fn ws_handler(ws:warp::ws::Ws,id: String, clients:Clients)-> Result<impl Reply>{

    let client = clients.lock(). await.get(&id).cloned();

    match client {
        Some(c) => Ok(ws.on_upgrade(move | socket| ws::client_connection(socket, id, cliens, c))),
        None=> Err(warp::reject::not_found)
    }
}


pub async fn client_connection(ws:WebSocket, id: String, clients: Clients, mut client: Client){

    let (client_ws_sender, mut client_ws_rcv) = ws.split();
    let (client_sender, client_rcv) = mpsc:: unbounded_channel();

    let client_rcv = UnboundedReceiverStream:: new(client_rcv);
    tokio:: task ::spawn(client_rcv.forward(client_ws_sender).map(|result|{

        if let Err(e) = result{
            eprintln!("error sending websocket msg: {}", e )
        }
    }));
}












