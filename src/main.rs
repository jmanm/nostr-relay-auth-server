use std::vec;

use nostr_sdk::prelude::*;
use tonic::{transport::Server, Request, Response, Status};

use nauthz_grpc::authorization_server::{Authorization, AuthorizationServer};
use nauthz_grpc::{Decision, EventReply, EventRequest};

pub mod nauthz_grpc {
    tonic::include_proto!("nauthz");
}

#[derive(Default)]
pub struct EventAuthz {
    allowed_kinds: Vec<u64>,
    allowed_authors: Vec<String>,
}

#[tonic::async_trait]
impl Authorization for EventAuthz {
    async fn event_admit(
        &self,
        request: Request<EventRequest>,
    ) -> Result<Response<EventReply>, Status> {
        let reply;
        let req = request.into_inner();
        let event = req.event.unwrap();
        let content_prefix: String = event.content.chars().take(40).collect();
        let author_id = XOnlyPublicKey::from_slice(&event.pubkey)
            .unwrap()
            .to_bech32()
            .unwrap();
        
        println!("recvd event, [kind={}, origin={:?}, author={:?}, nip05_domain={:?}, tag_count={}, content_sample={:?}]",
                 event.kind, req.origin, author_id, req.nip05.map(|x| x.domain), event.tags.len(), content_prefix);
        
        let kind_permitted = self.allowed_kinds.contains(&event.kind);
        let author_permitted = self.allowed_authors.contains(&author_id);

        if kind_permitted && author_permitted {
            println!("This looks fine!");

            reply = nauthz_grpc::EventReply {
                decision: Decision::Permit as i32,
                message: None,
            };
        } else {
            let message = if !kind_permitted {
                format!("Kind {} not permitted", event.kind)
            }
            else {
                format!("Author {} not permitted", author_id)
            };

            println!("{}", message);

            reply = nauthz_grpc::EventReply {
                decision: Decision::Deny as i32,
                message: Some(message),
            };
        }

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();

    let checker = EventAuthz {
        allowed_kinds: vec![0, 1, 2, 3, 30023],
        allowed_authors: vec!["<pub key>".to_string()],
    };
    
    println!("EventAuthz Server listening on {}", addr);
    
    // Start serving
    Server::builder()
        .add_service(AuthorizationServer::new(checker))
        .serve(addr)
        .await?;
    Ok(())
}
