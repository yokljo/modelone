/*
Nodes could be serialized by converting id and gen to a string like "5+9", so they can be stored as
a JSON map of string -> object in the graph, and they deserialize directly to the node reference's
data.

--

To do a reference system where all references update, the UI can be initalised with the graph, and
any references can create an ApplyHandle and give it to some kind of graph handler. When a
GraphSignal is emitted, the UI can look up all ApplyHandles associated with the signalling node, and
call them. This means the graph itself doesn't need to deal with the reference updates, but instead,
the UI handles all "cached" data, and all connections between nodes that need to be updated.
*/

struct NodeRef<T> {
	id: usize,
	gen: usize,
}

impl<T> NodeRef<T> {
	fn get(&self, graph: &Graph) -> &T {
		graph.get(self);
	}

	fn get_mut(&self, graph: &mut Graph) -> Option<&mut T> {
		graph.get_mut(self);
	}
}

/*pub struct TypedRef<T>(NodeRef);

impl<T> TypedRef<T> {
	fn get(&self, graph: &Graph) -> &T {
		self.0.get(graph).as_any().downcast_ref().unwrap()
	}

	fn get_mut(&self, graph: &mut Graph) -> Option<&mut T> {
		self.0.get_mut(graph).as_any_mut().downcast_mut().unwrap()
	}
}*/

struct NodeSlot<T: Node, JoinT: Join> {
	gen: usize,
	node: RefCell<T>,
	joins: Vec<(NodeRef<T>, JoinT)>,
}

trait GraphVisitor {
	fn visit(&mut self, node: NodeRef);
}

trait Node {
	fn visit(&self, visitor: &mut dyn GraphVisitor);

	//fn as_any(&self) -> &Any;
	//fn as_any_mut(&mut self) -> &mut Any;
}

trait Join : PartialEq {
	fn is_strong(&self) -> bool;
}

struct Graph<T: Node, JoinT: Join> {
	current_gen: usize,
	slots: Vec<Option<NodeSlot<T, JoinT>>>,
	roots: Vec<NodeRef>,
}

impl<T: Node> Graph<T> {
	fn get(&self, node_ref: NodeRef) -> &Node {

	}

	fn get_mut(&mut self, node_ref: NodeRef) -> &mut Node {

	}
}

#[derive(Debug, Clone, PartialEq)]
pub struct GraphSignal<ST> {
	Add(NodeRef<T>),
	Remove(NodeRef<T>),
	Change(NodeRef<T>, ST),
}

#[derive(Debug, Clone, PartialEq)]
pub struct GraphChange<T: Changeable<C>, C: Change> {
	Add(NodeRef<T>),
	Remove(NodeRef<T>),
	Change(NodeRef<T>, C),
}

impl<T: 'static + Changeable<C> + Send, C: Change> Change for GraphChange<T, C> {
	type SignalType = GraphSignal<C::SignalType>;
}

impl<T: 'static + Changeable<C> + Send, C: Change> Changeable<VecChange<T, C>> for Graph<T> {
	fn changeable_apply(&mut self, change: VecChange<T, C>, watcher: &mut Watcher<VecSignal<C::SignalType>>) {
		use self::VecChange::*;
		match change {
			Add(node_ref) => {
				self.insert(index, item);
				watcher.send_signal(GraphSignal::Add(node_ref));
			},
			Remove(node_ref) => {
				self.remove(index);
				watcher.send_signal(GraphSignal::Remove(node_ref));
			},
			Change(node_ref, subchange) => {
				let mut watcher_fn = |signal| {
					watcher.send_signal(GraphSignal::Change(node_ref, signal));
				};
				self[index].changeable_apply(subchange, &mut SubWatcher::new(&mut watcher_fn));
			},
		}
	}

	fn reset_view_signals(&self) -> Vec<VecSignal<C::SignalType>> {
		let mut signals = vec![];
		for slot in &self.slots {

		}
		signals
	}
}

//////////////////////////

// Any element should be able to query the graph for nodes that join via a particular type.
// Eg. if A (a reference) joins to B (a document) via the Reference type, then B should be able to
// ask for all Reference joins to itself and get back A.
#[derive(PartialEq)]
enum ElementJoin {
	Child,
	Content,
	Reference,
}

impl Join for ElementJoin {
	fn is_strong(&self) -> bool {
		match self {
			ElementJoin::Child | ElementJoin::Content => true,
			_ => false,
		}
	}
}

struct Document {

}

struct TextRun {
	text: String,
}

struct TextReference {
	ref_element: NodeRef<Element>,
}

impl TextReference {
	fn visit_refs(&self, visitor: &mut dyn GraphVisitor) {
		visitor.visit(self.ref_element, ElementJoin::Reference);
	}
}

struct TextFootnote {
	content_element: NodeRef<Element>,
}

impl TextFootnote {
	fn visit_refs(&self, visitor: &mut dyn GraphVisitor) {
		visitor.visit(self.content_element, ElementJoin::Content);
	}
}

enum ParaPart {
	Run(TextRun),
	Reference(TextReference),
	Footnote(TextFootnote),
}

impl ParaPart {
	fn visit_refs(&self, visitor: &mut dyn GraphVisitor) {
		match self {
			ParaPart::Run(run) => {}
			ParaPart::Reference(reference) => reference.visit_refs(visitor)
			ParaPart::Footnote(footnote) => footnote.visit_refs(visitor)
		}
	}
}

struct Paragraph {
	parts: Vec<ParaPart>,
}

struct ElementData {
	Document(Document),
	Paragraph(Paragraph),
}

impl ElementData {
	fn visit_refs(&self, visitor: &mut dyn GraphVisitor) {
		match self {
			ElementData::Document(document) => {}
			ElementData::Paragraph(paragraph) => paragraph.visit_refs(visitor)
		}
	}
}

struct Element {
	children: Vec<NodeRef<Element>>,
	id: u128,
	data: ElementData,
}

impl Node for Element {
	// TODO: This is for GC, but maybe the Graph type doesn't need to deal with it?
	fn visit_refs(&self, visitor: &mut dyn GraphVisitor) {
		for child in &children {
			visitor.visit(child, ElementJoin::Child);
			child.visit_refs(visitor);
		}
	}
}
