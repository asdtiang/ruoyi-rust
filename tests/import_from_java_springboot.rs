use convert_case::{Case, Casing};
use ruoyi_rust::utils::string::substring;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[tokio::test]
async fn from_java_domain() -> ruoyi_rust::error::Result<()> {
    let java_paths=vec!["D:\\Project\\xccgj-oa\\ruoyi-common\\src\\main\\java\\com\\ruoyi\\common\\core\\domain\\entity","D:\\Project\\xccgj-oa\\ruoyi-system\\src\\main\\java\\com\\ruoyi\\system\\domain"];
    for java_path in java_paths {
        for entry in fs::read_dir(java_path)? {
            let entry = entry?;
            let path = entry.path();
            let file_name = path.file_name().unwrap().to_str().unwrap().to_string();
            let idx = file_name.rfind(".").unwrap_or(file_name.len());
            let file_name = substring(file_name.as_str(), 0, idx);

            if let Some(ext) = path.extension() {
                if ext == "java" {
                    read_java_file_vo(file_name, path).await;
                }
            }
        }
    }
    Ok(())
}
#[derive(Debug)]
pub struct NewFile {
    excel: String,
    validate: Vec<String>,
    comment: String,
}
impl Default for NewFile {
    fn default() -> Self {
        NewFile {
            excel: "".to_string(),
            validate: vec![],
            comment: "".to_string(),
        }
    }
}

async fn read_java_file_vo(file_name: String, file: PathBuf) {
    let s = fs::read_to_string(&file).unwrap();
    let lines = s.split("\n").collect::<Vec<&str>>();

    let mut map = HashMap::new();
    let mut excel = "".to_string();
    let mut validates = vec![];
    let mut comment = "".to_string();
    for  line in lines {
        let line = line.trim();
        if line.starts_with("@Excel") {
            let line = substring(line, 7, line.len() - 1);
            let line = format!("#[excel({})]", &line.replace("name =", ""));
            excel = line.replace("type = Type.", "attrType = crate::AttrType::");
        }else if line.starts_with("@") {
            let a_idx=line.find('(').unwrap_or_default();
            if line.starts_with("@NotBlank") {
                validates.push(format!("#[validate(required({}))]", &line[a_idx+1..line.len()-1]));
                
            }else if line.starts_with("@Size"){
                validates.push(format!("#[validate(length({}))]", &line[a_idx+1..line.len()-1])); 
            }else if line.starts_with("@Min") {
                validates.push(format!("#[range(length({}))]", &line[a_idx+1..line.len()-1]));
            }else if line.starts_with("@Max") {
                validates.push(format!("#[range(length({}))]", &line[a_idx+1..line.len()-1]));
            }else if line.starts_with("@NotNull") {
                validates.push(format!("#[validate(required({}))]", &line[a_idx+1..line.len()-1]));

            }else if line.starts_with("@Email") {
                validates.push(format!("#[validate(email({}))]", &line[a_idx+1..line.len()-1]));

            }else if line.starts_with("@Override") {
                
            }else if line.starts_with("@Pattern") {
                validates.push(format!("//todo {}))]",line));
            }else if line.starts_with("@JsonIgnore") {

            }else if line.starts_with("@JsonFormat") {

            }else if line.starts_with("@Xss") {
                validates.push(format!("//todo {}))]",line));
            }else{
                println!("not in {}",line);
            }
        }
        
        if line.starts_with("private") {
            let ss = line.split(" ").collect::<Vec<&str>>();
            if ss.len() >= 3 {
                map.insert(
                    ss[2].to_string().replace(";", "").to_case(Case::Snake),
                    NewFile {
                        excel: excel.clone(),
                        validate: validates.clone(),
                        comment: comment.clone(),
                    },
                );
                excel = "".to_string();
                comment = "".to_string();
            }
        }
        if line.starts_with("public ")&&line.contains("get") {
            let ss = line.split(" ").collect::<Vec<&str>>();
            if ss.len() >= 3 {
                let key= ss[2].to_string().replace("get", "").replace("()", "").to_case(Case::Snake);
                if map.contains_key(&key) {
                    let new_file = map.get_mut(&key).unwrap();
                    new_file.validate = validates.clone();
                }
                validates = vec![];
                
            }
        }
        if line.starts_with("/**") {
            comment = line.to_string();
        }
    }
    println!("{:?}", map);

    let rust_paths = vec![
        "D:\\Project\\A-Ruoyi\\axum\\ruoyi-rust\\src\\modules\\system\\domain\\vo",
        "D:\\Project\\A-Ruoyi\\axum\\ruoyi-rust\\src\\modules\\system\\domain\\dto",
        "D:\\Project\\A-Ruoyi\\axum\\ruoyi-rust\\src\\modules\\system\\domain\\mapper",
    ];
    let new_file_name = file_name.replace("Sys", "").to_case(Case::Snake);
    for rust_path in rust_paths {
        let is_vo = rust_path.ends_with("vo");
        let is_dto = rust_path.ends_with("dto");
        let is_mapper = rust_path.ends_with("mapper");
        let rs_path_buf = if is_mapper {
            PathBuf::from(rust_path).join(format!("{}.rs", file_name.to_case(Case::Snake)))
        } else {
            PathBuf::from(rust_path).join(format!("{}.rs", new_file_name.to_case(Case::Snake)))
        };
        if fs::exists(rs_path_buf.as_path()).unwrap() {
            let s = fs::read_to_string(rs_path_buf.as_path()).unwrap();
            let lines = s.split("\n").collect::<Vec<&str>>();
            let mut start_struct = false;
            let mut outs = Vec::new();
            for line in lines {
                let new_line = line.trim();
                if new_line.starts_with("pub struct ") &&!new_line.contains("Page"){
                    start_struct = true;
                }
                if start_struct {
                    if new_line.starts_with("pub") {
                        let idx = new_line.find(':');
                        if let Some(idx) = idx {
                            let attr_name = &new_line[4..idx];
                            if map.contains_key(attr_name) {
                                let new_file = map.get(attr_name).unwrap();
                                let new_ = if is_vo {
                                    format!("{}\n{}", new_file.comment, new_file.excel)
                                } else if is_dto {
                                    format!("{}\n{}", new_file.comment, new_file.validate.join("\n"))
                                }else{
                                    format!("{}", new_file.comment)
                                };
                                println!("{new_}");
                                outs.push(new_.trim().replace("\n\n", "\n"));
                            }
                        }
                    }
                }
                if new_line.starts_with("}") {
                    start_struct = false;
                }
                outs.push(line.to_string());
            }
             if is_mapper {
                 fs::write(
                     PathBuf::from(rust_path).join(format!("{}.txt", file_name.to_case(Case::Snake)),),
                     outs.join("\n"),
                 )
                     .unwrap();
            } else {
                 fs::write(
                     PathBuf::from(rust_path).join(format!("{}.txt", new_file_name)),
                     outs.join("\n"),
                 )
                     .unwrap();
            };
           
        }
    }
}
use regex::Regex;

