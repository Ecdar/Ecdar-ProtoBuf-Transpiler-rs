use std::fs;
use std::path::Path;

const STRUCTS: &str = r#"
#[derive(Debug)]
pub struct Service{
    pub name: &'static str,
    pub endpoints : &'static [Endpoint]
}

#[derive(Debug)]
pub struct Endpoint{
    pub name: &'static str,
    pub input_type: ProtobuffTypes,
    pub output_type: ProtobuffTypes,
}

#[derive(Debug)]
pub struct ProtobuffTypes {
    pub name : &'static str
}

"#;

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let root_file = "./Ecdar-ProtoBuf/services.proto";

    fs::write(
        format!("{out_dir}/services.rs"),
        format!(
            "{STRUCTS}pub const SERVICES: &'static[Service]= &[{}];",
            find_service(Path::new(root_file))
        ),
    )
    .unwrap();

    println!("cargo:rerun-on-change=./build.rs");
    println!("cargo:rerun-on-change=./Ecdar-ProtoBuf/");
}

#[derive(Default)]
struct Serializer<'a> {
    pub file: &'a str,
}

impl<'a> Iterator for Serializer<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        let mut start = 0;
        let mut end = 0;
        let mut chars = self.file.chars();

        if self.file.is_empty() {
            return None;
        }

        macro_rules! step {
            () => {{
                start += 1;
                end += 1;
                if let Some(c) = chars.next() {
                    c
                } else {
                    break;
                }
            }};
        }

        macro_rules! step_end {
            () => {{
                end += 1;
                if let Some(c) = chars.next() {
                    c
                } else {
                    break;
                }
            }};
        }

        macro_rules! handle_whitespace {
            () => {{
                if start != end {
                    break;
                }
                start += 1;
                end += 1;
            }};
        }

        macro_rules! handle_comment {
            () => {{
                start += 1;
                end += 1;
                let next = step!();
                match next {
                    '/' => loop {
                        if '\n' == step!() {
                            break;
                        }
                    },
                    '*' => loop {
                        if '*' == step!() {
                            if '/' == step!() {
                                break;
                            }
                        }
                    },
                    _ => {
                        break;
                    }
                }
            }};
        }

        macro_rules! handle_string {
            () => {{
                end += 1;
                loop {
                    if '"' == step_end!() {
                        break;
                    }
                }
            }};
        }

        macro_rules! handle_special_char {
            () => {{
                if start == end {
                    end += 1;
                }
                break;
            }};
        }

        while let Some(c) = chars.next() {
            match c {
                _ if c.is_whitespace() => handle_whitespace!(),
                '/' => handle_comment!(),
                '"' => handle_string!(),
                '{' | '}' | '(' | ')' | '[' | ']' | ';' | '=' => handle_special_char!(),
                _ => end += 1,
            }
        }

        let rtn = &self.file[start..end];
        self.file = &self.file[end..];
        if rtn.is_empty() {
            return None;
        }

        Some(rtn)
    }
}

fn find_service(path: &Path) -> String {
    let root = path.parent().unwrap().to_str().unwrap().trim_matches('"');
    let file = &fs::read_to_string(path).unwrap()[..];
    let mut serializer = Serializer { file };
    let mut rtn = String::new();

    'outer: loop {
        macro_rules! next {
            () => {{
                match serializer.next() {
                    Some(token) => {
                        println!("{token}");
                        token
                    }
                    None => {
                        break 'outer;
                    }
                }
            }};
        }

        macro_rules! expect {
            ($e:expr, $g:expr) => {{
                let token = $g;
                if $e != token {
                    panic!(
                        "Wront syntax in .protofile: {path:?} expected {} got {}",
                        $e, token
                    )
                }
            }};
        }

        let token = next!();

        match token {
            "import" => {
                let new_file = next!();
                let mut path_iter = new_file.trim_matches('"').split('/');
                let mut folder = path_iter
                    .next()
                    .expect("empty string in import")
                    .to_string();
                println!("folder : {folder}");
                if folder == "google" {
                    continue 'outer;
                }
                while folder.strip_suffix(".proto").is_none() {
                    folder += path_iter.next().expect("no .proto file in import");
                }
                rtn += find_service(Path::new(format!("{root}/{folder}").as_str())).as_str();
            }
            "service" => {
                let name = next!();
                rtn = rtn + "Service{name:\"" + name + "\",endpoints:&[";
                expect!("{", next!());
                loop {
                    let token = next!();
                    if token == "}" {
                        break;
                    }
                    expect!("rpc", token);
                    let endpoint = next!();
                    rtn = rtn + "Endpoint{name:" + "\"" + endpoint + "\"" + ",";
                    expect!("(", next!());
                    let input_type = next!();
                    rtn = rtn + "input_type:ProtobuffTypes{name:\"" + input_type + "\"}" + ",";
                    expect!(")", next!());
                    expect!("returns", next!());
                    expect!("(", next!());
                    let output_type = next!();
                    rtn = rtn + "output_type:ProtobuffTypes{name:\"" + output_type + "\"}" + "},";
                    expect!(")", next!());
                    expect!(";", next!());
                }
                rtn += "]},";
            }
            _ => {
                // Ignore entire statement
                let mut indent = 0;
                loop {
                    let token = next!();
                    match token {
                        "{" => indent += 1,
                        "}" => {
                            if indent == 0 {
                                break;
                            } else {
                                indent -= 1
                            }
                        }
                        ";" => {
                            if indent == 0 {
                                break;
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    rtn
}
