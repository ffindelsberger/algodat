pub mod http {
    use std::{
        collections::{linked_list, LinkedList},
        io::Write,
        net::{TcpListener, TcpStream},
    };

    use crate::threadpool::ThreadPool;

    struct Server {
        listener: TcpListener,
        pool: ThreadPool,
    }

    impl Server {
        pub fn new(port: u16) -> Self {
            let listener = TcpListener::bind(("127.0.0.1", port)).unwrap();
            let pool = ThreadPool::with_capacaty(20);

            Server { listener, pool }
        }

        pub fn start(self) {
            loop {
                println!("Waiting for connections");
                let Ok((mut socket, _)) = self.listener.accept() else {
                    todo!();
                };

                self.pool
                    .submit(Box::new(|| handle_socket_connection(socket)))
                //let handler = Handler { socket };
                //self.pool.submit(Box::new(|| handler.run()));
            }
        }
    }

    fn handle_socket_connection(mut socket: TcpStream) {
        socket.write_all("hello function".as_bytes());
    }

    struct Handler {
        socket: TcpStream,
    }

    impl Handler {
        fn run(mut self) {
            self.socket.write_all("Hello Partner".as_bytes());
            LinkedList::new()
        }
    }

    #[cfg(test)]
    mod test {
        use super::Server;

        #[test]
        fn http_test() {
            let server = Server::new(8080);
            server.start();
        }
    }
}
