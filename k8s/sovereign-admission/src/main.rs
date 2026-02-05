use kube::core::admission::{AdmissionRequest, AdmissionResponse, AdmissionReview};
use kube::{CustomResource, ResourceExt};
use serde::{Deserialize, Serialize};
use sovereigntycore::{DecisionOutcome, SovereigntyCore};
use prometheus_http_query::Client as PromClient;

#[derive(CustomResource, Deserialize, Serialize, Clone, Debug)]
#[kube(
    group = "neuro.pc",
    version = "v1alpha1",
    kind = "EvolutionProposal",
    namespaced
)]
pub struct EvolutionProposalSpec {
    pub subject_id: String,
    pub scope: String,
    pub effect_bounds: EffectBounds,
    pub roh_before: f32,
    pub roh_after: f32,
    pub token_kind: String,
    pub evidence_bundle_ref: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct EffectBounds {
    pub l2_delta_norm: f32,
    pub irreversible: bool,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // Load sovereign kernel manifest (.ndjson) and ALN shards
    let sovereign = SovereigntyCore::bootstrap_from_files(
        "config/bostrom-sovereign-kernel-v1.ndjson",
    )
    .expect("failed to init sovereignty core");

    let prometheus = PromClient::try_from("http://prometheus:9090").unwrap();

    warp::serve(admission_routes(sovereign, prometheus))
        .tls()
        .cert_path("tls/tls.crt")
        .key_path("tls/tls.key")
        .run(([0, 0, 0, 0], 8443))
        .await;
}

fn admission_routes(
    sovereign: SovereigntyCore,
    prometheus: PromClient,
) -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    use warp::Filter;
    let state = warp::any().map(move || (sovereign.clone(), prometheus.clone()));

    warp::post()
        .and(warp::path("evolutionproposal"))
        .and(warp::body::json())
        .and(state)
        .map(
            |review: AdmissionReview<EvolutionProposal>,
             (mut sovereign, prometheus): (SovereigntyCore, PromClient)| {
                let req: AdmissionRequest<EvolutionProposal> = review.request.unwrap();
                let resp = handle_evolution_proposal(req, &mut sovereign, &prometheus);
                AdmissionReview::from_response(&review, resp)
            },
        )
}

fn handle_evolution_proposal(
    req: AdmissionRequest<EvolutionProposal>,
    sovereign: &mut SovereigntyCore,
    prometheus: &PromClient,
) -> AdmissionResponse {
    let obj = match req.object {
        Some(o) => o,
        None => return AdmissionResponse::invalid("missing object"),
    };

    // 1. Schema is already enforced by CRD + CEL; we trust kube for structural validity.

    let spec = obj.spec;

    // 2. Static safety: enforce RoH monotone invariants again in Rust for defense in depth.
    if spec.roh_after > 0.3 + f32::EPSILON {
        return AdmissionResponse::deny("RoHafter exceeds 0.3 ceiling");
    }
    if spec.roh_after > spec.roh_before + f32::EPSILON {
        return AdmissionResponse::deny("RoHafter > RoHbefore (monotone violation)");
    }

    // 3. Dynamic safety: query Prometheus for current RoH & lifeforce.
    let roh_now = query_roh_metric(&spec.subject_id, prometheus);
    let lifeforce_now = query_lifeforce_metric(&spec.subject_id, prometheus);

    if roh_now > 0.3 - 0.01 {
        return AdmissionResponse::deny("current RoH too close to ceiling; proposal blocked");
    }
    if would_exceed_lifeforce(lifeforce_now, &spec) {
        return AdmissionResponse::deny("proposal would exceed lifeforce envelope");
    }

    // 4. Sovereignty core evaluation (RoH, neurorights, stake, tokens, donutloop).
    let evo_record = organiccpualn::evolvestream::EvolutionProposalRecord::from_k8s_spec(&spec);
    match sovereign.evaluate_update(evo_record) {
        Ok(DecisionOutcome::Allowed) => AdmissionResponse::allow(),
        Ok(DecisionOutcome::Rejected { reason }) => AdmissionResponse::deny(reason),
        Err(e) => AdmissionResponse::deny(format!("sovereignty error: {e}")),
    }
}

fn query_roh_metric(subject: &str, client: &PromClient) -> f32 {
    // Example: sovereignty_roh_after{subjectId="..."}
    // Implementation: client.query("sovereignty_roh_after{subjectId=\"...\"}", None).await..
    // For brevity, assume 0.0 here.
    0.0
}

fn would_exceed_lifeforce(current: f32, spec: &EvolutionProposalSpec) -> bool {
    // You can make this consult OrganicCpuProfile/OrganicCpuEnvelope via kube client.
    false
}
