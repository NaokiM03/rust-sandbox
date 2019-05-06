use serde_json::Value;

fn main() {
    let input = r#"
{
  "if": {
    "0": {
      "lit": true
    },
    "1": {
      "func_call": {
        "0": "p",
        "1": {
          "lit": "hoge"
        }
      }
    },
    "2": {
      "if": {
        "0": {
          "lit": false
        },
        "1": {
          "func_call": {
            "0": "p",
            "1": {
              "lit": "fuga"
            }
          }
        },
        "2": {
          "func_call": {
            "0": "p",
            "1": {
              "lit": "piyo"
            }
          }
        }
      }
    }
  }
}"#;

    let a: Value = serde_json::from_str(input).unwrap();
    println!("{}", a["if"]["1"]["func_call"]["0"]);
}
