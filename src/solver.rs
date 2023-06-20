use alloc::vec::Vec;
use alloc::string::String;

use petgraph::Graph;
use thiserror_no_std::Error;

use crate::{obligation::Obligations, Money, Obligation, Person};

#[derive(Error, Debug)]
pub enum SolverError {
    #[error("could not find endpoints for edge index {0}")]
    NoEndpointForEdge(usize),

    #[error("could not find weight for node index {0}")]
    NoWeightForNode(usize),

    #[error("could not find weight for edge index {0}")]
    NoWeightForEdge(usize),
}

pub struct Solver(pub(crate) petgraph::Graph<String, i32>);

impl Solver {
    #[inline(always)]
    pub fn new(item: Obligations) -> Self {
        let mut g = Graph::<String, i32>::new();

        for obligation in item.raw() {
            let from = obligation.from.raw().clone();
            let to = obligation.to.raw().clone();
            let amount = obligation.amount.raw();

            let from_exists = g
                .node_indices()
                .filter(|node| g[*node] == from)
                .collect::<Vec<_>>();

            let to_exists = g
                .node_indices()
                .filter(|node| g[*node] == to)
                .collect::<Vec<_>>();

            match (from_exists.first(), to_exists.first()) {
                (Some(from), None) => {
                    let to = g.add_node(to);

                    g.add_edge(*from, to, amount);
                }
                (None, Some(to)) => {
                    let from = g.add_node(from);

                    g.add_edge(from, *to, amount);
                }
                (Some(from), Some(to)) => match g.find_edge(*from, *to) {
                    Some(existing_edge) => {
                        let existing_weight = g.edge_weight(existing_edge).unwrap_or(&0);

                        g.update_edge(*from, *to, existing_weight + amount);
                    }
                    None => {
                        g.add_edge(*from, *to, obligation.amount.raw());
                    }
                },
                (None, None) => {
                    let from = g.add_node(from);
                    let to = g.add_node(to);

                    g.add_edge(from, to, obligation.amount.raw());
                }
            }
        }

        Self(g)
    }

    #[inline(always)]
    pub fn solve(&mut self) -> Result<Obligations, SolverError> {
        self.pass_remove_doubly_connected_edges();
        self.pass_simplify_double_target();
        self.pass_remove_same_weight_target();
        self.pass_remove_zero_edges();
        self.format_out()
    }

    /// First Pass
    /// Reduce doubly connected edges to a single edge connection.
    /// The resulting direction is dictated by subtracting the edges' weights.
    /// In case the result is zero, then both edges are removed.
    #[inline(always)]
    fn pass_remove_doubly_connected_edges(&mut self) {
        for edge in self.0.edge_indices() {
            // Waiting for https://github.com/rust-lang/rust/issues/53667 to be stabilized...
            if let Some((e1_source, e1_target)) = self.0.edge_endpoints(edge) {
                if let Some(e2) = self.0.find_edge(e1_target, e1_source) {
                    if let Some(e1) = self.0.find_edge(e1_source, e1_target) {
                        if let (Some(w1), Some(w2)) =
                            (self.0.edge_weight(e1), self.0.edge_weight(e2))
                        {
                            match (w1, w2) {
                                _ if w1 > w2 => {
                                    self.0.update_edge(e1_source, e1_target, w1 - w2);
                                    self.0.remove_edge(e2);
                                }
                                _ if w1 < w2 => {
                                    self.0.update_edge(e1_target, e1_source, w2 - w1);
                                    self.0.remove_edge(e1);
                                }
                                _ => {
                                    self.0.remove_edge(e1);
                                    self.0.remove_edge(e2);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    /// Second Pass
    /// With an edge, see if the target has an edge to yet another node that the source has too.
    /// H -> C; H -> A; C -> A
    /// Remove that alienated edge (H -> C)
    /// Add its weight to the edge from the source (H -> A)
    /// Subtract it from the edge from the target (C -> A)
    #[inline(always)]
    fn pass_simplify_double_target(&mut self) {
        loop {
            let edge_count = self.non_zero_edges_count();

            self.simplify_double_target();

            if self.non_zero_edges_count() >= edge_count
                || self.non_zero_edges_count() == 0
                || edge_count == 0
            {
                break;
            }
        }
    }

    #[inline(always)]
    fn simplify_double_target(&mut self) {
        for edge in self.0.edge_indices() {
            // This should always return something? :thinking:
            let (source, target) = self
                .0
                .edge_endpoints(edge)
                .expect("endpoints from edge are none");

            for node in self.0.node_indices() {
                if node == source || node == target {
                    continue;
                }

                if let Some(source_node_edge) = self.0.find_edge(source, node) {
                    if let Some(target_node_edge) = self.0.find_edge(target, node) {
                        let (edge_weight, source_node_weight, target_node_weight) = match (
                            self.0.edge_weight(edge),
                            self.0.edge_weight(source_node_edge),
                            self.0.edge_weight(target_node_edge),
                        ) {
                            (
                                Some(edge_weight),
                                Some(source_edge_weight),
                                Some(target_edge_weight),
                            ) if edge_weight != &0
                                && source_edge_weight != &0
                                && target_edge_weight != &0 =>
                            {
                                (*edge_weight, *source_edge_weight, *target_edge_weight)
                            }
                            _ => {
                                continue;
                            }
                        };

                        self.0
                            .update_edge(source, node, source_node_weight + edge_weight);

                        if target_node_weight - edge_weight > 0 {
                            self.0
                                .update_edge(target, node, target_node_weight - edge_weight);
                        } else {
                            self.0.update_edge(target, node, 0);
                            self.0
                                .add_edge(node, target, edge_weight - target_node_weight);
                        }

                        self.0.update_edge(source, target, 0);

                        break;
                    }
                }
            }
        }
    }

    /// Third Pass
    // If there's an edge A --[X]--> B and another B --[X]--> C, it can be reduced to A --[X]--> C
    fn pass_remove_same_weight_target(&mut self) {
        for edge in self.0.edge_indices() {
            if let Some((source, target)) = self.0.edge_endpoints(edge) {
                let weight = self.0[edge];

                if weight == 0 {
                    continue;
                }

                for node in self.0.node_indices() {
                    if let Some(found) = self.0.find_edge(target, node) {
                        let next_weight = self.0[found];

                        if next_weight == weight {
                            self.0.update_edge(source, target, 0);
                            self.0.update_edge(target, node, 0);

                            self.0.add_edge(source, node, weight);
                        }
                    }
                }
            }
        }
    }

    #[inline(always)]
    fn non_zero_edges_count(&mut self) -> i32 {
        self.0.edge_weights().fold(0, |acc, weight| {
            if *weight != 0 {
                return acc + 1;
            }
            acc
        })
    }

    #[inline(always)]
    // https://github.com/petgraph/petgraph/issues/299
    fn pass_remove_zero_edges(&mut self) {
        let mut g = Graph::<String, i32>::new();

        for edge in self.0.edge_indices() {
            if let Some((source, target)) = self.0.edge_endpoints(edge) {
                if let Some(edge_weight) = self.0.edge_weight(edge) {
                    if *edge_weight == 0 {
                        continue;
                    }

                    if let (Some(source_weight), Some(target_weight)) =
                        (self.0.node_weight(source), self.0.node_weight(target))
                    {
                        let source = g.add_node(source_weight.clone());
                        let target = g.add_node(target_weight.clone());
                        g.add_edge(source, target, *edge_weight);
                    }
                }
            }
        }

        self.0 = g;
    }

    #[inline(always)]
    fn format_out(&self) -> Result<Obligations, SolverError> {
        let mut obligations = Obligations::builder();

        for edge in self.0.edge_indices() {
            let endpoint = self
                .0
                .edge_endpoints(edge)
                .ok_or_else(|| SolverError::NoEndpointForEdge(edge.index()))?;

            let from = self
                .0
                .node_weight(endpoint.0)
                .ok_or_else(|| SolverError::NoWeightForNode(endpoint.0.index()))?;

            let to = self
                .0
                .node_weight(endpoint.1)
                .ok_or_else(|| SolverError::NoWeightForNode(endpoint.1.index()))?;

            let weight = self
                .0
                .edge_weight(edge)
                .ok_or_else(|| SolverError::NoWeightForEdge(edge.index()))?;

            obligations.record(
                Obligation::builder()
                    .from(Person::new(from))
                    .to(Person::new(to))
                    .amount(Money::new(*weight))
                    .build(),
            );
        }

        Ok(obligations.build())
    }
}
