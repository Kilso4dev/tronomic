use egui::{self, DragValue};
use egui_node_graph as eng;
use egui_node_graph::NodeId;
use super::graph::*;

pub type NodeGraphType = eng::GraphEditorState<node::Node, port::GType, port::GVal, node::NodeTempl, MyGraphState>;

// ========= First, define your user data types =============

/// The response type is used to encode side-effects produced when drawing a
/// node in the graph. Most side-effects (creating new nodes, deleting existing
/// nodes, handling connections...) are already handled by the library, but this
/// mechanism allows creating additional side effects from user code.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GResp {
}

/// The graph 'global' state. This state struct is passed around to the node and
/// parameter drawing callbacks. The contents of this struct are entirely up to
/// the user. For this example, we use it to keep track of the 'active' node.
#[derive(Debug, Clone, Default)]
pub struct MyGraphState {
    pub time: f64,
    pub frame: f64,
}

// =========== Then, you need to implement some traits ============

// A trait for the data types, to tell the library how to display them
impl eng::DataTypeTrait for port::GType {
    fn data_type_color(&self) -> egui::epaint::Color32 {
        match self {
            Self::INum(_, _) => egui::Color32::from_rgb(38, 109, 211),
            Self::IVec(_, _) => egui::Color32::from_rgb(8, 79, 181),

            Self::FNum(_, _) => egui::Color32::from_rgb(238, 207, 109),
            Self::FVec(_, _) => egui::Color32::from_rgb(208, 177, 79),
            Self::Color => egui::Color32::from_rgb(179, 255, 199),
        }
    }

    fn name(&self) -> &str {
        match self {
            Self::INum(_, _) => "integer",
            Self::FNum(_, _) => "float",
            Self::IVec(_, _) => "list of integers",
            Self::FVec(_, _) => "list of floats",
            Self::Color => "color",
        }
    }
}

#[derive(Debug, Clone)]
pub struct AllNodeTempl;
impl eng::NodeTemplateIter for AllNodeTempl {
    type Item = node::NodeTempl;

    fn all_kinds(&self) -> Vec<Self::Item> {
        // This function must return a list of node kinds, which the node finder
        // will use to display it to the user. Crates like strum can reduce the
        // boilerplate in enumerating all variants of an enum.
        vec![
            node::NodeTempl::Frame,
            node::NodeTempl::Time,
            node::NodeTempl::Sine,
            node::NodeTempl::Output,
        ]
    }
}

impl eng::WidgetValueTrait for port::GVal {
    fn value_widget(&mut self, param_name: &str, ui: &mut egui::Ui) {
        // This trait is used to tell the library which UI to display for the
        // inline parameter widgets.
        match self {
            Self::INum(v, range) => {
                ui.horizontal(|ui| {
                    ui.label(param_name);
                    ui.add(DragValue::new(v).clamp_range(range.clone()));
                });
            }
            Self::FNum(v, range) => {
                ui.horizontal(|ui| {
                    ui.label(param_name);
                    ui.add(DragValue::new(v).clamp_range(range.clone()));
                });
            }
            Self::IVec(v, range) => {
                let mut new_len: usize = v.len();
                let mut new_val: i64 = 0;
                ui.label(param_name);
                ui.horizontal(|ui| {
                    ui.label("value Count");
                    ui.add(DragValue::new(&mut new_len).clamp_range(range.clone()));
                });
                ui.horizontal(|ui| {
                    ui.label("values");
                    ui.add(DragValue::new(&mut new_val).clamp_range(range.clone()));
                });
                v.clear();

                v.resize(new_len, new_val);
            }
            Self::FVec(v, range) => {
                let mut new_len: usize = v.len();
                let mut new_val: f64 = 0.;
                ui.label(param_name);
                ui.horizontal(|ui| {
                    ui.label("value Count");
                    ui.add(DragValue::new(&mut new_len).clamp_range(range.clone()));
                });
                ui.horizontal(|ui| {
                    ui.label("values");
                    ui.add(DragValue::new(&mut new_val).clamp_range(range.clone()));
                });
                v.clear();
                v.resize(new_len, new_val);
            }
            Self::Color(c) => {
                ui.horizontal(|ui| {
                    let mut rgba = c.clone().into();
                    ui.label(param_name);
                    egui::widgets::color_picker::show_color(ui, rgba, egui::vec2(20., 20.));
                    egui::widgets::color_picker::color_edit_button_rgba(ui, &mut rgba, egui::color_picker::Alpha::Opaque);
                    *c = rgba.into();
                });
            }
        }
    }
}

impl eng::UserResponseTrait for GResp {}
impl eng::NodeDataTrait for node::Node {
    type Response = GResp;
    type UserState = MyGraphState;
    type DataType = port::GType;
    type ValueType = port::GVal;

    // This method will be called when drawing each node. This allows adding
    // extra ui elements inside the nodes. In this case, we create an "active"
    // button which introduces the concept of having an active node in the
    // graph. This is done entirely from user code with no modifications to the
    // node graph library.
    fn bottom_ui(
        &self,
        _ui: &mut egui::Ui,
        _node_id: NodeId,
        _graph: &eng::Graph<node::Node, port::GType, port::GVal>,
        _user_state: &Self::UserState,
    ) -> Vec<eng::NodeResponse<GResp>>
    where
        GResp: eng::UserResponseTrait,
    {
        // This logic is entirely up to the user. In this case, we check if the
        // current node we're drawing is the active one, by comparing against
        // the value stored in the global user state, and draw different button
        // UIs based on that.

        let responses = vec![];

        responses
    }
}

pub fn node_graph(state: &mut parking_lot::RwLockWriteGuard<NodeGraphType>, ui: &mut egui::Ui) {
    let graph_response = state.draw_graph_editor(ui, AllNodeTempl);
    for node_response in graph_response.node_responses {
        // Here, we ignore all other graph events. But you may find
        // some use for them. For example, by playing a sound when a new
        // connection is created
        if let eng::NodeResponse::User(user_event) = node_response {
            match user_event {
            }
        }
    }
}
