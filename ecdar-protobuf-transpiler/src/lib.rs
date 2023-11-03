use convert_case::*;


mod services {
    ecdar_protobuf_transpiler_macros::get_services!();
}

impl services::ProtobuffTypes {
    pub fn to_rust_type(&self) -> String{
        let is_google = self.name.split('.').next().unwrap() == "google";
        match self.name {
            _ if !is_google => format!("ecdar_protobuf::services::{}", self.name.to_case(Case::Pascal)),
            "google.protobuf.Empty" => "()".into(),
            _ => { panic!("{} is not maped, please map it in crate ecdar-protobuf-transpiler", self.name)}
        }
    } 
}

pub use services::*;





