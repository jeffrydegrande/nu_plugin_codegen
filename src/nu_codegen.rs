// use liquid::model::to_value;
use liquid::model::Value as LiquidValue;
use nu_plugin::{EvaluatedCall, LabeledError, Plugin};
use nu_protocol::{Category, PluginExample, PluginSignature, SyntaxShape, Value as NuValue};
// use std::collections::HashMap;
use std::fs;

pub struct NuCodeGen;

impl Plugin for NuCodeGen {
    fn signature(&self) -> Vec<PluginSignature> {
        // pass template file here as a mandatory argument
        vec![PluginSignature::build("codegen")
            .usage("Generate code from a liquid template")
            .required("template", SyntaxShape::String, "liquid template")
            .plugin_examples(vec![PluginExample {
                example: "cat file.csv | from csv | codegen".to_string(),
                description: "Renders any data to code".to_string(),
                result: None,
            }])
            .category(Category::Experimental)]
    }

    fn run(
        &mut self,
        _name: &str,
        call: &EvaluatedCall,
        input: &NuValue,
    ) -> Result<NuValue, LabeledError> {
        let filename: String = call.req(0)?;

        let content = fs::read_to_string(filename).expect("Failed to read file");

        let template = liquid::ParserBuilder::with_stdlib()
            .build()
            .unwrap()
            .parse(&content)
            .unwrap();

        let liquid_object = liquid::object!({ "items": convert_to_liquid(input.clone()) });
        let output = template.render(&liquid_object).unwrap();

        return Ok(NuValue::String {
            val: output.to_string(),
            internal_span: call.head,
        });
    }
}

fn convert_to_liquid(val: NuValue) -> LiquidValue {
    match val {
        NuValue::Nothing { .. } => LiquidValue::Nil,
        NuValue::Int { val, .. } => LiquidValue::scalar(val),
        NuValue::String { val, .. } => LiquidValue::scalar(val),
        NuValue::List { vals, .. } => {
            let liquid_vals: Vec<LiquidValue> = vals.into_iter().map(convert_to_liquid).collect();
            LiquidValue::Array(liquid_vals)
        }
        NuValue::Record { val, .. } => {
            let mut liquid_map: liquid::Object = liquid::Object::new();
            for (k, v) in val.iter() {
                liquid_map.insert(k.clone().into(), convert_to_liquid(v.clone()));
            }
            LiquidValue::Object(liquid_map)
        }
        _ => panic!("Unsupported type in liquid conversion: {:?}", val),
    }
}
