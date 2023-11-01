#[derive(Debug)]
pub struct Service{
    pub name: &'static str,
    pub endpoints : Vec<Endpoint>
}

#[derive(Debug)]
pub struct Endpoint{
    pub name: &'static str,
    pub input_type: &'static str,
    pub output_type: &'static str,
}

pub use ecdar_protobuf_transpiler_macros::*;

#[cfg(test)]
mod test {
    #[test]
    pub fn test() {
        let service = ecdar_protobuf_transpiler_macros::get_services!();
    }
}

