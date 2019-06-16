pub mod client;
pub mod network;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
        client::connect();
        network::connect();
        network::client::connect();
    }
}
