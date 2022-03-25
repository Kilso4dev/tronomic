use super::port::*;
use crate::error::DmGuiError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
//use egui_nodes::NodeArgs;
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub title: &'static str,
    pub inp: Vec<GVal>,

    #[serde(skip)]
    pub driver: Driver,
    node_inst: NodeTempl,
    //fn ui(&mut self, ui: egui::Ui);
}

impl Node {
    //pub fn nodeargs(&self) -> NodeArgs {
    //    NodeArgs::default() // TODO
    //}
}

#[derive(Clone)]
pub enum Driver {
    None,
    Func(fn(&HashMap<&'static str, GVal>, Vec<GVal>) -> Result<Vec<GVal>, DmGuiError>),
    Ctx(fn(&HashMap<&'static str, GVal>, Vec<GVal>) -> Result<Vec<(usize, GVal)>, DmGuiError>),
}

impl fmt::Debug for Driver {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Func(_) => write!(f, "Driver::Func(<fn omitted>)"),
            Self::Ctx(_) => write!(f, "Driver::Ctx(<fn omitted>)"),
            Self::None => write!(f, "Driver::None: <THIS SHOULD NOT BE HERE>"),
        }
    }
}

impl Default for Driver {
    fn default() -> Self {
        Driver::None
    } // TODO(Other way to (De)serialize Driver)
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConnectionTo {
    pub start_port: usize,
    pub end_node: usize,
    pub end_port: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NodeTempl {
    Frame,
    Time,
    Output,
    Sine,
}

// A trait for the node kinds, which tells the library how to build new nodes
// from the templates in the node finder
impl egui_node_graph::NodeTemplateTrait for NodeTempl {
    type NodeData = Node;
    type DataType = GType;
    type ValueType = GVal;

    fn node_finder_label(&self) -> &str {
        match self {
            Self::Output => "Output",
            Self::Frame => "Frame since start",
            Self::Sine => "Sine",
            Self::Time => "Time",
        }
    }

    fn node_graph_label(&self) -> String {
        // It's okay to delegate this to node_finder_label if you don't want to
        // show different names in the node finder and the node itself.
        self.node_finder_label().into()
    }

    fn user_data(&self) -> Self::NodeData {
        match self {
            Self::Frame => Node {
                title: "Frame",
                driver: Driver::Func(|props, _inp| {
                    Ok(vec![props
                        .get("frame")
                        .ok_or_else(|| {
                            DmGuiError::evaluation("property Frame could not be extracted")
                        })?
                        .clone()])
                }),
                node_inst: NodeTempl::Frame,
                inp: vec![GVal::INum(0, 0..=i64::MAX)],
            },
            Self::Time => Node {
                title: "Time",
                driver: Driver::Func(|props, _inp| {
                    Ok(vec![props
                        .get("time")
                        .ok_or_else(|| {
                            DmGuiError::evaluation("property Frame could not be extracted")
                        })?
                        .clone()])
                }),
                node_inst: NodeTempl::Time,
                inp: vec![],
            },

            Self::Output => Node {
                title: "Output",
                driver: Driver::Ctx(|_props, mut inp| {
                    let universe = graph_values::gval_as_inum(inp.remove(0), Some(0..=i64::MAX))?;
                    let channels = graph_values::gval_as_ivec(inp.remove(0), Some(0..=255))?;

                    Ok(vec![(
                        universe as usize,
                        GVal::IVec(channels.clone(), 0..=255),
                    )])
                }),
                node_inst: NodeTempl::Output,
                inp: vec![],
            },
            Self::Sine => Node {
                title: "Sine",
                driver: Driver::Func(|props, _| {
                    Ok(vec![props
                        .get("")
                        .ok_or_else(|| {
                            DmGuiError::evaluation("property Frame could not be extracted")
                        })?
                        .clone()])
                }),
                node_inst: NodeTempl::Time,
                inp: vec![],
            },
        }
    }

    fn build_node(
        &self,
        graph: &mut egui_node_graph::Graph<Self::NodeData, Self::DataType, Self::ValueType>,
        node_id: egui_node_graph::NodeId,
    ) {
        macro_rules! input {
            (i $name:expr, $range:expr, $stat:expr) => {
                graph.add_input_param(
                    node_id,
                    $name.to_string(),
                    GType::INum($range),
                    GVal::INum(0, $range),
                    eng::InputParamKind::ConnectionOrConstant,
                    !($stat),
                );
            };
            (i $name:expr, $range:expr) => {input!(i $name, $range, false)};

            (ivec $name:expr, $range:expr, $stat:expr) => {
                graph.add_input_param(
                    node_id,
                    $name.to_string(),
                    GType::IVec(*$range.start(), *$range.end()),
                    GVal::IVec(Vec::new(), $range),
                    egui_node_graph::InputParamKind::ConnectionOrConstant,
                    !($stat),
                );
            };
            (ivec $name:expr, $range:expr) => {input!(ivec $name, $range, false)};

            (f $name:expr, $range:expr, $stat:expr) => {
                graph.add_input_param(
                    node_id,
                    $name.to_string(),
                    GType::FNum(*$range.start(), *$range.end()),
                    GVal::FNum(0.0, $range),
                    egui_node_graph::InputParamKind::ConnectionOrConstant,
                    !($stat),
                );
            };
            (f $name:expr, $range:expr) => {input!(f $name, $range, false)};

            (fvec $name:expr, $range:expr, $stat:expr) => {
                graph.add_input_param(
                    node_id,
                    $name.to_string(),
                    GType::FVec($range),
                    GVal::FVec(Vec::new(), $range),
                    eng::InputParamKind::ConnectionOrConstant,
                    !($stat),
                );
            };
            (fvec $name:expr, $range:expr) => {input!(fvec $name, $range, false)};

            (color $name:expr, $range:expr, $stat:expr) => {
                graph.add_input_param(
                    node_id,
                    $name.to_string(),
                    GType::Color,
                    GVal::Color(color::Rgba::default(), $range),
                    eng::InputParamKind::ConnectionOrConstant,
                    !($stat),
                );
            };
            (color $name:expr, $range:expr) => {input!(color $name, $range, false)};
        }

        macro_rules! output {
            (i $name:expr, $range:expr) => {
                graph.add_output_param(
                    node_id,
                    $name.to_string(),
                    GType::INum(*$range.start(), *$range.end()),
                );
            };
            (f $name:expr, $range:expr) => {
                graph.add_output_param(
                    node_id,
                    $name.to_string(),
                    GType::FNum(*$range.start(), *$range.end()),
                );
            };
        }

        match self {
            Self::Frame => {
                // The first input param doesn't use the macro so we can comment
                // it in more detail.
                output!(i "", 0..=i64::MAX);
            }
            Self::Output => {
                input!(ivec "channels", 0..=255, true);
                input!(ivec "Universe", 0..=512, false);
            }
            Self::Time => {
                output!(f "", 0.0..=f64::INFINITY);
            }
            Self::Sine => {
                input!(f "x", 0.0..=f64::INFINITY);
                output!(f "sin", 0.0..=f64::INFINITY);
            }
        }
    }
}
