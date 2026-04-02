use std::{env, fs, path::Path};

fn main() {
    let spec_path = "openapi.yaml";
    println!("cargo:rerun-if-changed={}", spec_path);

    let src = fs::read_to_string(spec_path)
        .unwrap_or_else(|e| panic!("Failed to read {spec_path}: {e}"));

    // Convert YAML → serde_json::Value for manipulation
    let mut spec_value: serde_json::Value = serde_yaml::from_str(&src)
        .unwrap_or_else(|e| panic!("Failed to parse YAML in {spec_path}: {e}"));

    // --- Pre-process spec to avoid known progenitor limitations ---
    // Progenitor asserts that all success responses for an operation share
    // the same Rust type. When an operation has (e.g.) a 201 with a body AND
    // a 204 with no body, or two 2xx codes with different schemas, the
    // assertion fires. We normalise: for each operation, keep only the FIRST
    // 2xx response and drop the rest, then verify only one 2xx remains.
    preprocess_spec(&mut spec_value);

    let spec: openapiv3::OpenAPI = serde_json::from_value(spec_value)
        .unwrap_or_else(|e| panic!("Failed to interpret OpenAPI spec: {e}"));

    let mut settings = progenitor::GenerationSettings::default();
    settings
        .with_interface(progenitor::InterfaceStyle::Builder)
        .with_tag(progenitor::TagStyle::Merged)
        // Rename the generated Orderbook so our override wrapper can keep the
        // public name `Orderbook` with the extra dollar-level serde fields.
        .with_patch(
            "Orderbook",
            progenitor::TypePatch::default().with_rename("GeneratedOrderbook"),
        );

    let tokens = progenitor::Generator::new(&settings)
        .generate_tokens(&spec)
        .unwrap_or_else(|e| panic!("progenitor generation failed: {e}"));

    let ast = syn::parse2(tokens)
        .unwrap_or_else(|e| panic!("Failed to parse generated tokens: {e}"));
    let content = prettyplease::unparse(&ast);

    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");
    let out_file = Path::new(&out_dir).join("codegen.rs");
    fs::write(&out_file, content)
        .unwrap_or_else(|e| panic!("Failed to write codegen to {}: {e}", out_file.display()));
}

/// Normalise the spec so each operation has at most one 2xx response with a
/// body. This works around progenitor's `assert!(response_types.len() <= 1)`.
///
/// Strategy:
/// 1. For each operation, collect 2xx response codes.
/// 2. If there is more than one, keep only the one with the richest content
///    (prefer JSON body over empty body), dropping the others.
/// 3. If an operation mixes a body-bearing 2xx with a body-less 2xx (e.g.
///    200-with-schema + 201-empty), we keep the body-bearing one.
fn preprocess_spec(spec: &mut serde_json::Value) {
    let paths = match spec
        .get_mut("paths")
        .and_then(|p| p.as_object_mut())
    {
        Some(p) => p,
        None => return,
    };

    let http_methods = ["get", "post", "put", "delete", "patch", "head", "options", "trace"];

    for (_path, path_item) in paths.iter_mut() {
        let path_obj = match path_item.as_object_mut() {
            Some(o) => o,
            None => continue,
        };

        for method in &http_methods {
            let op = match path_obj.get_mut(*method).and_then(|o| o.as_object_mut()) {
                Some(o) => o,
                None => continue,
            };

            let responses = match op.get_mut("responses").and_then(|r| r.as_object_mut()) {
                Some(r) => r,
                None => continue,
            };

            // Collect 2xx keys (as strings: "200", "201", etc.)
            let codes_2xx: Vec<String> = responses
                .keys()
                .filter(|k| k.starts_with('2') && k.len() == 3)
                .cloned()
                .collect();

            // Normalize non-2xx (error) responses: replace every error response
            // with a plain description-only object (no content, no $ref).
            // This ensures all error arms generate ResponseValue::empty() →
            // Error<()>. Without this, operations where some error codes use a
            // $ref with a body (e.g. 404 → NotFoundError) and others have no
            // body (400/500) produce mismatched ResponseValue<ErrorType> vs
            // ResponseValue<()> that the compiler rejects.
            let error_codes: Vec<String> = responses
                .keys()
                .filter(|k| !k.starts_with('2') && k.len() == 3)
                .cloned()
                .collect();
            for code in error_codes {
                responses.insert(
                    code.clone(),
                    serde_json::json!({ "description": "Error" }),
                );
            }

            if codes_2xx.len() <= 1 {
                continue;
            }

            // Pick the "best" 2xx: prefer the one with application/json content.
            let best = codes_2xx
                .iter()
                .find(|code| {
                    responses
                        .get(*code)
                        .and_then(|r| r.get("content"))
                        .and_then(|c| c.get("application/json"))
                        .is_some()
                })
                .or_else(|| {
                    // Fall back to any code that has *any* content
                    codes_2xx.iter().find(|code| {
                        responses
                            .get(*code)
                            .and_then(|r| r.get("content"))
                            .map(|c| !c.as_object().map(|o| o.is_empty()).unwrap_or(true))
                            .unwrap_or(false)
                    })
                })
                .or_else(|| codes_2xx.first())
                .cloned();

            if let Some(best_code) = best {
                let to_remove: Vec<String> = codes_2xx
                    .into_iter()
                    .filter(|c| c != &best_code)
                    .collect();
                for code in to_remove {
                    eprintln!("build.rs: removed duplicate 2xx response {code} (keeping {best_code})");
                    responses.remove(&code);
                }
            }
        }
    }
}
