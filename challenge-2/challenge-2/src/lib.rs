use serde::{Deserialize, Serialize};
use serde_json::json;
use spin_sdk::{
    http::{IntoResponse, Method, Request, Response},
    http_component,
    key_value::Store,
    llm,
};

#[derive(Debug, Serialize, Deserialize)]
struct PlanRequest {
    #[serde(rename(deserialize = "tag"))]
    tag: Option<String>,
    #[serde(rename(deserialize = "destination"))]
    destination: String,
    #[serde(rename(deserialize = "duration"))]
    duration: String,
    #[serde(rename(deserialize = "num_people"))]
    people: String,
    #[serde(rename(deserialize = "activities"))]
    activities: Vec<String>,
}

#[http_component]
fn hello_world(req: Request) -> anyhow::Result<impl IntoResponse> {
    let model = llm::InferencingModel::Llama2Chat;
    let store = Store::open_default()?;

    match req.method() {
        Method::Post => {
            if req.path().contains("plan-my-trip") {
                let body_str = String::from_utf8_lossy(req.body()).to_string();
                let mut plan: PlanRequest =
                    serde_json::from_str(&body_str).expect("fail to parse into PlanRequest");
                let mut tag = plan.tag;
                if tag.is_none() {
                    tag = Some(String::from(format!("{}{}", plan.destination, plan.people)));
                }
                let activities = plan.activities.join(",");
                let prompt = format!("Create a summer vacation detailed itinerary trip to go to {} for a {}. {} people will be going on this trip planning to do {}", plan.destination, plan.duration, plan.people, activities);
                let inference = llm::infer(model, &prompt).expect("fail to use llm");
                let infer = format!("{:?}", inference.text);

                plan.tag = tag.clone();
                let key = tag.expect("can't extract value");
                let _ = store.set(&key, infer.as_bytes());
                let body_str = json!({
                    "itinerary": {
                        "tag": plan.tag.expect("get tag value"),
                        "details": infer,
                    }
                });
                return Ok(Response::builder()
                    .status(201)
                    .header("content-type", "application/json")
                    .body(body_str.to_string())
                    .build());
            } else {
                return Ok(Response::builder().status(405).build());
            }
        }
        Method::Get => {
            let key = req.path().trim_start_matches('/');
            let cache = store.get(key)?;
            if cache.is_some() {
                let cache_str = cache.expect("unable to get value from cache");
                let prompt = String::from_utf8_lossy(&cache_str);
                let body_str = json!({
                    "itinerary": {
                        "tag": key,
                        "details": prompt,
                    }
                });
                return Ok(Response::builder()
                    .status(200)
                    .header("content-type", "application/json")
                    .body(body_str.to_string())
                    .build());
            }
            return Ok(Response::builder()
                .status(200)
                .header("content-type", "text/plain")
                .body("data not found.")
                .build());
        }
        _ => {
            return Ok(Response::builder()
                .status(200)
                .header("content-type", "text/plain")
                .body("Only POST | Get Allowed")
                .build())
        }
    }
}
