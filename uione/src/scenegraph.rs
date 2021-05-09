use crate::vec2::*;

type NodeId = usize;
type ShaderId = usize;

struct Node {
	parent: NodeId,
	children: Vec<NodeId>,
	shader: ShaderId,
}

enum QuadSection {
	QuadNode(Box<QuadNode>),
	Node(NodeId),
}

struct QuadNode {
	centre: Vec2f,
	top_left: QuadSection,
	top_right: QuadSection,
	bottom_left: QuadSection,
	bottom_right: QuadSection,
}

pub struct SceneGraph {
	nodes: Vec<Option<Node>>
}

impl SceneGraph {
	pub fn new() -> SceneGraph {
		SceneGraph {
			nodes: vec![]
		}
	}
	
	fn allocate_node(&mut self) -> NodeId {
		// TODO
		return 0;
	}
	
	fn item_damaged(&mut self, item: &Item) {
		let id = item.get_item().id.get();
		let node;
		if let Some(index) = id {
			node = &mut self.nodes[index];
		} else {
			let new_index = self.allocate_node();
			node = &mut self.nodes[new_index];
		}
	}
}
