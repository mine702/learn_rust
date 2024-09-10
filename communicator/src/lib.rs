pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

mod outermost {
    pub fn middle_function() {}
    pub fn middle_secret_function() {}
    pub mod inside {
        pub fn inner_function() {}
        pub fn secret_function() {}
    }
}

fn try_me() {
    outermost::middle_function();
    outermost::middle_secret_function();
    outermost::inside::inner_function();
    outermost::inside::secret_function();
}
pub mod client;
pub mod network;

#[cfg(test)]
mod tests2 {
    use super::client;

    #[test]
    fn it_works() {
        client::connect();
    }
}