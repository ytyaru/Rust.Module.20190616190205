mod client;
mod network;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);

        client::connect();
        network::connect();
        network::client::connect();
    }
}
