use convert_case::*;

mod services {
    ecdar_protobuf_transpiler_macros::get_services!();
}

impl services::ProtobuffTypes {
    pub fn to_rust_type(&self) -> String {
        let is_google = self.name.split('.').next().unwrap() == "google";
        match self.name {
            _ if !is_google => format!(
                "ecdar_protobuf::services::{}",
                self.name.to_case(Case::Pascal)
            ),
            "google.protobuf.Empty" => "()".into(),
            _ => {
                panic!(
                    "{} is not maped, please map it in crate ecdar-protobuf-transpiler",
                    self.name
                )
            }
        }
    }
}

pub struct CompileVariables {
    pub fn_name: String,
    pub endpoint_name: String,
    pub service_name: String,
    pub in_struct: String,
    pub in_struct_name: String,
    pub in_struct_has_body: bool,
    pub rtn_struct: String,
}

pub fn compile(foreach: impl Fn(CompileVariables) -> String) -> String {
    services::SERVICES
        .iter()
        .map(|service| {
            service
                .endpoints
                .iter()
                .map(|endpoint| {
                    let fn_name = get_fn_name(service.name, endpoint.name).to_case(Case::Snake);
                    let in_struct_has_body = endpoint.input_type.to_rust_type() != "()";
                    let in_struct_name = format!("In{}", fn_name.to_case(Case::Pascal));
                    let rtn_struct = endpoint.output_type.to_rust_type();

                    foreach(CompileVariables {
                        endpoint_name: endpoint.name.into(),
                        service_name: service.name.into(),
                        in_struct: format!(
                            "struct In{}{{ ip : String, {}}}",
                            in_struct_name,
                            if in_struct_has_body {
                                format!("body : {}", endpoint.input_type.to_rust_type())
                            } else {
                                "".into()
                            }
                        ),
                        in_struct_has_body,
                        in_struct_name,
                        rtn_struct,
                        fn_name,
                    })
                })
                .collect::<Vec<_>>()
                .join("\n")
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn get_fn_name(service_name: &str, enpoint_name: &str) -> String {
    format!(
        "{}_{}",
        service_name.to_case(Case::Snake),
        enpoint_name.to_case(Case::Snake)
    )
}

pub use services::*;
