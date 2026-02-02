if proposal.roh_after > 0.3_f32 {
    allowed = false;
    reason.push_str("RoH_after exceeds 0.3;");
}
if proposal.roh_after > proposal.roh_before + 1e-6_f32 {
    allowed = false;
    reason.push_str("RoH not monotone-safe;");
}
