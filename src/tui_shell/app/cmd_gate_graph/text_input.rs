use super::*;

impl App {
    pub(in crate::tui_shell) fn submit_gate_graph_text_input(
        &mut self,
        action: TextInputAction,
        value: String,
    ) {
        let raw = value.trim().to_string();
        match action {
            TextInputAction::GateGraphAddGateId => {
                let id = raw;
                if let Err(msg) = validate_gate_id_local(&id) {
                    self.push_error(msg);
                    return;
                }
                self.gate_graph_new_gate_id = Some(id.clone());
                self.open_text_input_modal(
                    "Gate Graph",
                    "new gate name> ",
                    TextInputAction::GateGraphAddGateName,
                    None,
                    vec![format!("gate id: {}", id)],
                );
            }

            TextInputAction::GateGraphAddGateName => {
                let name = raw;
                if name.is_empty() {
                    self.push_error("missing gate name".to_string());
                    return;
                }
                let Some(id) = self.gate_graph_new_gate_id.clone() else {
                    self.push_error("missing gate id".to_string());
                    return;
                };
                self.gate_graph_new_gate_name = Some(name.clone());
                self.open_text_input_modal(
                    "Gate Graph",
                    "upstream (comma-separated)> ",
                    TextInputAction::GateGraphAddGateUpstream,
                    None,
                    vec![
                        format!("gate id: {}", id),
                        format!("name: {}", name),
                        "Enter upstream gate ids, or leave blank for a root gate.".to_string(),
                    ],
                );
            }

            TextInputAction::GateGraphAddGateUpstream => {
                let Some(id) = self.gate_graph_new_gate_id.clone() else {
                    self.push_error("missing gate id".to_string());
                    return;
                };
                let Some(name) = self.gate_graph_new_gate_name.clone() else {
                    self.push_error("missing gate name".to_string());
                    return;
                };
                let upstream = parse_id_list(&raw);
                self.apply_gate_graph_edit(Some(id.clone()), |g| {
                    if g.gates.iter().any(|x| x.id == id) {
                        anyhow::bail!("gate id already exists: {}", id);
                    }
                    g.gates.push(crate::remote::GateDef {
                        id: id.clone(),
                        name: name.clone(),
                        upstream,
                        allow_releases: true,
                        allow_superpositions: false,
                        allow_metadata_only_publications: false,
                        required_approvals: 0,
                    });
                    Ok(())
                });
                self.gate_graph_new_gate_id = None;
                self.gate_graph_new_gate_name = None;
            }

            TextInputAction::GateGraphEditUpstream => {
                let Some(v) = self.current_view::<GateGraphView>() else {
                    self.push_error("not in gates mode".to_string());
                    return;
                };
                let Some(gid) = self.gate_graph_selected_gate_id(v) else {
                    self.push_error("(no selection)".to_string());
                    return;
                };
                self.apply_gate_graph_edit(Some(gid.clone()), |g| {
                    let gate = g
                        .gates
                        .iter_mut()
                        .find(|x| x.id == gid)
                        .ok_or_else(|| anyhow::anyhow!("selected gate not found"))?;
                    gate.upstream = parse_id_list(&raw);
                    Ok(())
                });
            }

            TextInputAction::GateGraphSetApprovals => {
                let n: u32 = match raw.parse() {
                    Ok(v) => v,
                    Err(_) => {
                        self.push_error("expected a non-negative integer".to_string());
                        return;
                    }
                };
                let Some(v) = self.current_view::<GateGraphView>() else {
                    self.push_error("not in gates mode".to_string());
                    return;
                };
                let Some(gid) = self.gate_graph_selected_gate_id(v) else {
                    self.push_error("(no selection)".to_string());
                    return;
                };
                self.apply_gate_graph_edit(Some(gid.clone()), |g| {
                    let gate = g
                        .gates
                        .iter_mut()
                        .find(|x| x.id == gid)
                        .ok_or_else(|| anyhow::anyhow!("selected gate not found"))?;
                    gate.required_approvals = n;
                    Ok(())
                });
            }

            _ => self.push_error("unexpected gates input".to_string()),
        }
    }
}
