use mongodb::coll::Collection;
use mongodb::db::{DatabaseInner, ThreadedDatabase};
use mongodb::{Client, ClientInner, ThreadedClient};
use std::sync::Arc;

pub type DB = Arc<DatabaseInner>;

pub struct MongoClient {
    pub client: Arc<ClientInner>,
}

impl MongoClient {
    pub fn new(host: Option<&str>, port: Option<u16>) -> MongoClient {
        let host = match host {
            Some(x) => x,
            None => "localhost",
        };
        let port = match port {
            Some(x) => x,
            None => 27017,
        };
        MongoClient {
            client: Client::connect(host, port).unwrap(),
        }
    }

    pub fn default() -> MongoClient {
        MongoClient::new(None, None)
    }

    pub fn db(&self) -> DB {
        self.client.db("cuisinio")
    }

    pub fn collection(&self, coll: &str) -> Collection {
        self.db().collection(coll)
    }
}
