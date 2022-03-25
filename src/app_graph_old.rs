/*use super::graph::{
    port::{Port, PortType, PortValue},
    ConnectionTo, NodeGraph,
};
use egui::Color32;
use egui::Ui;
use egui_nodes::{Context, LinkArgs, NodeConstructor, PinArgs, PinShape};

fn pinargs_from_port_type(t: PortType) -> PinArgs {
    match t {
        PortType::FNum(_) => PinArgs {
            shape: PinShape::CircleFilled,
            background: Some(Color32::from_rgb(0, 181, 217)),
            hovered: None,
            ..Default::default()
        },
        PortType::INum(_) => PinArgs {
            shape: PinShape::CircleFilled,
            background: Some(Color32::from_rgb(0, 69, 217)),
            hovered: None,
            ..Default::default()
        },
        PortType::Color => PinArgs {
            shape: PinShape::CircleFilled,
            background: Some(Color32::from_rgb(255, 127, 59)),
            hovered: None,
            ..Default::default()
        },
        PortType::FVec(_) => PinArgs {
            shape: PinShape::TriangleFilled,
            background: Some(Color32::from_rgb(0, 181, 217)),
            hovered: None,
            ..Default::default()
        },
        PortType::IVec(_) => PinArgs {
            shape: PinShape::TriangleFilled,
            background: Some(Color32::from_rgb(0, 69, 217)),
            hovered: None,
            ..Default::default()
        },
        PortType::None => PinArgs {
            shape: PinShape::Circle,
            background: Some(Color32::from_rgb(140, 140, 140)),
            hovered: None,
            ..Default::default()
        },
    }
}

fn ui_from_type(pt: &'static mut Port) -> Box<dyn FnOnce(&mut Ui) -> egui::Response> {
    match &mut pt.pval {
        PortValue::None => Box::new(|ui| ui.label(pt.name)),
        PortValue::INum(i, range) => Box::new(|ui| {
            ui.with_layout(egui::Layout::left_to_right(), |ui| {
                ui.add(egui::Label::new(format!("{}: ", pt.name)));
                ui.add(egui::Slider::new(i, range.clone()));
            })
            .response
        }),
        PortValue::FNum(f, range) => Box::new(|ui| {
            ui.with_layout(egui::Layout::left_to_right(), |ui| {
                ui.add(egui::Label::new(format!("{}: ", pt.name)));
                ui.add(egui::Slider::new(f, range.clone()));
            })
            .response
        }),
        PortValue::Color(col) => Box::new(|ui| {
            ui.with_layout(egui::Layout::left_to_right(), |ui| {
                ui.add(egui::Label::new(format!("{}: ", pt.name)));
                egui::color_picker::color_edit_button_rgba(
                    ui,
                    col,
                    egui::color_picker::Alpha::Opaque,
                );
            })
            .response
        }),
        PortValue::FVec(_, _) => Box::new(|ui| {
            ui.with_layout(egui::Layout::left_to_right(), |ui| {
                ui.add(egui::Label::new(format!("{}: ", pt.name)));
            })
            .response
        }),
        PortValue::IVec(_, _) => Box::new(|ui| {
            ui.with_layout(egui::Layout::left_to_right(), |ui| {
                ui.add(egui::Label::new(format!("{}: ", pt.name)));
            })
            .response
        }),
    }
}

fn sinp_id_from_m(nodei: usize, porti: u16, (_si, _i, _o): (u16, u16, u16)) -> usize {
    ((nodei as usize) << 16) | (porti) as usize
}

fn inp_id_from_m(nodei: usize, porti: u16, (si, _i, _o): (u16, u16, u16)) -> usize {
    ((nodei as usize) << 16) | (porti + si) as usize
}

fn outp_id_from_m(nodei: usize, porti: u16, (si, i, _o): (u16, u16, u16)) -> usize {
    ((nodei as usize) << 16) | (porti + i + si) as usize
}

fn nid_from_pid(pid: usize) -> usize {
    pid >> 16
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum PortT {
    Inp,
    SInp,
    Outp,
    None,
}
const PID_16_MASK: usize = 0xffff;
fn loc_id_from_pid(pid: usize, (si, i, o): (u16, u16, u16)) -> (usize, PortT) {
    let (si, i, o) = (si as usize, i as usize, o as usize);
    let port = pid & PID_16_MASK;
    if (0..si).contains(&port) {
        (port, PortT::SInp)
    } else if (si..(si + i)).contains(&port) {
        (port, PortT::Inp)
    } else if ((si + i)..(si + i + o)).contains(&port) {
        (port, PortT::Outp)
    } else {
        (port, PortT::None)
    }
}

pub fn node_graph(ctx: &mut Context, g: &mut NodeGraph, ui: &mut Ui) {
    let mut connections = Vec::new();
    let mut nodes = Vec::with_capacity(g.nodes.len());
    let max_tuple = g.nodes.iter().fold((0, 0, 0), |(asi, ai, ao), (_k, v)| {
        let v = v.read().unwrap();
        (
            asi.max(v.static_inp.len()),
            ai.max(v.inp.len()),
            ao.max(v.outp.len()),
        )
    });

    let max_tuple = (max_tuple.0 as u16, max_tuple.1 as u16, max_tuple.2 as u16);

    log::info!(
        "static_inp ids: {} inp ids: {} outp ids: {}",
        max_tuple.0, max_tuple.1, max_tuple.2
    );

    // remove destroyed links
    if let Some(idx) = ctx.link_destroyed() {
        // TODO
        if let Some((sn, con)) = connections.get(idx) {
            g.remove_connection(*sn, con);
        }
    }

    // add created links
    if let Some((start, end, _)) = ctx.link_created() {
        // TODO
        let start_node = nid_from_pid(start);
        let end_node = nid_from_pid(start);
        let (s_pid, s_pt) = loc_id_from_pid(start, max_tuple);
        let (e_pid, e_pt) = loc_id_from_pid(end, max_tuple);
        if s_pt == e_pt {
            g.add_connection(
                start_node,
                ConnectionTo {
                    start_port: s_pid,
                    end_node,
                    end_port: e_pid,
                },
            );
        }
    }

    for (id, cn) in g.nodes.iter_mut() {
        let cn = cn.write().unwrap();
        let mut new_node = NodeConstructor::new(*id, cn.nodeargs());
        let cn_title: &'static str = cn.title.clone();
        new_node = new_node.with_title(|ui| ui.label(cn_title.to_string()));

        for (ci, c) in cn.inp.iter().enumerate() {
            let name = c.name;
            new_node = new_node.with_input_attribute(
                inp_id_from_m(*id, ci as u16, max_tuple),
                pinargs_from_port_type((&c.pval).into()),
                move |ui| ui.label(name),
            )
        }

        for (ci, c) in cn.static_inp.iter().enumerate() {
            let name = c.name;
            new_node = new_node
                .with_static_attribute(sinp_id_from_m(*id, ci as u16, max_tuple), move |ui| {
                    ui.label(name)
                });
        }

        for (ci, c) in cn.outp.iter().enumerate() {
            let name = c.name;
            new_node = new_node.with_output_attribute(
                outp_id_from_m(*id, ci as u16, max_tuple),
                pinargs_from_port_type((&c.pval).into()),
                move |ui| ui.label(name),
            );
        }

        nodes.push(new_node);
        let mut node_cons: Vec<_> = cn.adjacents.iter().map(|c| (*id, c.clone())).collect();
        connections.append(&mut node_cons);
    }

    // add them to the ui
    let connect_tuples = connections.iter().enumerate().filter_map(
        |(
            i,
            (
                sid,
                ConnectionTo {
                    start_port,
                    end_node,
                    end_port,
                },
            ),
        )| {
            if let (Some(sn), Some(en)) = (g.nodes.get(&end_node), g.nodes.get(&sid)) {
                let start_type: PortType = (&sn.read().unwrap().outp[*start_port].pval).into();
                let end_type: PortType = (&en.read().unwrap().inp[*end_port].pval).into();
                Some((
                    i,
                    outp_id_from_m(*sid, *start_port as u16, max_tuple),
                    inp_id_from_m(*end_node, *end_port as u16, max_tuple),
                    LinkArgs {
                        base: Some(Color32::from_rgb(255, 255, 255)),
                        ..Default::default()
                    },
                ))
            } else {
                log::error!("Unknown end_node with id {end_node}");
                None
            }
        },
    );
    ctx.show(nodes, connect_tuples, ui);
}*/


