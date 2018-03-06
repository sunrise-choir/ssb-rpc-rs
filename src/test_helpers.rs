use std::convert::TryInto;
use std::fmt::Debug;
use std::net::{SocketAddr, Ipv6Addr};

use box_stream::BoxDuplex;
use futures::prelude::*;
use muxrpc::{muxrpc, RpcOut, Rpc};
use secret_stream::OwningClient;
use serde::de::DeserializeOwned;
use sodiumoxide;
use sodiumoxide::crypto::box_;
use ssb_common::*;
use ssb_keyfile::load_or_create_keys;
use tokio::executor::current_thread;
use tokio::net::TcpStream;
use tokio_io::AsyncRead;
use tokio_io::io::{ReadHalf, WriteHalf};

// Hands a `RpcOut` to a Fun and expects it to `current_thread::spawn` the stuff to test.
pub fn run_test<Fun: 'static + FnOnce(RpcOut<ReadHalf<BoxDuplex<TcpStream>>, WriteHalf<BoxDuplex<TcpStream>>>) -> ()>(fun: Fun){
    sodiumoxide::init();

    let (pk, sk) = load_or_create_keys().unwrap();
    let pk = pk.try_into().unwrap();
    let sk = sk.try_into().unwrap();
    let (ephemeral_pk, ephemeral_sk) = box_::gen_keypair();

    let addr = SocketAddr::new(Ipv6Addr::localhost().into(), DEFAULT_TCP_PORT);

    let do_stuff = TcpStream::connect(&addr)
            .and_then(move |tcp| {
                OwningClient::new(tcp,
                                  &MAINNET_IDENTIFIER,
                                  &pk,
                                  &sk,
                                  &ephemeral_pk,
                                  &ephemeral_sk,
                                  &pk)
                        .map_err(|(err, _)| err)
            })
            .map_err(|err| {
                println!("Tests only work with an ssb server accepting connections on the default port over localhost\n");
                println!("{:?}", err);
                panic!("\nAbort test");
            })
            .map(move |connection| {
                let (read, write) = connection.unwrap().split();
                let (_, rpc_out, _) = muxrpc(read, write);
                fun(rpc_out);
            });

    current_thread::run(|_| current_thread::spawn(do_stuff));
}

// Sends a sync rpc and checks that a non-error response is received.
pub fn test_sync<RPC: 'static + Rpc,
                 Response: 'static + DeserializeOwned,
                 Err: 'static + DeserializeOwned + Debug>
    (req: RPC) {
    run_test(move |mut rpc_out| {
        let (send_request, response) = rpc_out.sync::<RPC, Response, Err>(&req);

        current_thread::spawn(send_request.map_err(|err| panic!("Failed to send sync request:\n\n{:?}", err)));
        current_thread::spawn(response
                                  .map(|_| ())
                                  .map_err(|err| panic!("Got error receiving: {:?}", err)));
    });
}
