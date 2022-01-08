use tide::{Request, Response, Body, Server};
use tide::prelude::{Deserialize, Serialize};
use std::sync::Arc;
use async_std::sync::RwLock;
use std::collections::hash_map::{Entry, HashMap};

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Dino {
    name: String,
    weight: u16,
    diet: String,
}

#[derive(Clone, Debug)]
struct State {
    dinos: Arc<RwLock<HashMap<String, Dino>>>
}

async fn server(dinos_store: Arc<RwLock<HashMap<String, Dino>>>) -> Server<State> {
    let state = State {
        dinos: dinos_store,
    };

    let mut app = tide::with_state(state);

    app.at("/").get(|_| async move { Ok("ok") });
    app
}

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    tide::log::start();
    let dinos_store = Default::default();
    let mut app = server(dinos_store).await;

    app.at("/").get(|_| async { Ok("Hello, world!") });

    app
        .at("/dinos")
            .post(|mut req: Request<State>| async move {
                let dino: Dino = req.body_json().await?;
                let mut dinos = req.state().dinos.write().await;
                dinos.insert(String::from(&dino.name), dino.clone());
                let mut res = Response::new(201);
                res.set_body(Body::from_json(&dino)?);
                Ok(res)
            })
            .get(|req: Request<State>| async move {
                let dinos = req.state().dinos.read().await;
                let dinos_vec: Vec<Dino> = dinos.values().cloned().collect();
                let mut res = Response::new(200);
                res.set_body(Body::from_json(&dinos_vec)?);
                Ok(res)
            });
    
    app.at("/dinos/:name")
            .get(|req: Request<State>| async move {
                let mut dinos = req.state().dinos.write().await;
                let key: String = req.param("name")?.to_string();
                let res = match dinos.entry(key) {
                    Entry::Vacant(_entry) => Response::new(404),
                    Entry::Occupied(entry) => {
                        let mut res = Response::new(200);
                        res.set_body(Body::from_json(&entry.get())?);
                        res
                    }
                };
                Ok(res)
            })
            .put(|mut req: Request<State>| async move {
                let dino_update: Dino = req.body_json().await?;
                let mut dinos = req.state().dinos.write().await;
                let key: String = req.param("name")?.to_string();
                let res = match dinos.entry(key) {
                    Entry::Vacant(_entry) => Response::new(404),
                    Entry::Occupied(mut entry) => {
                        *entry.get_mut() = dino_update;
                        let mut res = Response::new(200);
                        res.set_body(Body::from_json(&entry.get())?);
                        res
                    }
                };
                Ok(res)
            })
            .delete(|req: Request<State>| async move {
                let mut dinos = req.state().dinos.write().await;
                let key: String = req.param("name")?.to_string();
                let deleted = dinos.remove(&key);
                let res = match deleted {
                    None => Response::new(404),
                    Some(_) => Response::new(204),
                };
                Ok(res)
            });
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
