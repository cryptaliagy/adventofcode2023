use std::collections::HashMap;

type Workflows = HashMap<String, Vec<Rule>>;

fn convert_workflows(workflows: Vec<Workflow>) -> Workflows {
    workflows
        .into_iter()
        .map(|workflow| (workflow.0, workflow.1))
        .collect()
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Constraint(pub u64, pub u64);

impl Constraint {
    pub fn size(&self) -> u64 {
        self.1 - self.0 + 1
    }

    pub fn overlap(&self, other: &Self) -> Option<Self> {
        let min = std::cmp::max(self.0, other.0);
        let max = std::cmp::min(self.1, other.1);

        if min > max {
            None
        } else {
            Some(Self(min, max))
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Operation {
    LessThan,
    LessThanEqual,
    GreaterThan,
    GreaterThanEqual,
    True,
    False,
}

impl Operation {
    pub fn eval(&self, a: u64, b: u64) -> bool {
        match self {
            Self::LessThan => a < b,
            Self::LessThanEqual => a <= b,
            Self::GreaterThan => a > b,
            Self::GreaterThanEqual => a >= b,
            Self::True => true,
            Self::False => false,
        }
    }

    pub fn constrain(&self, constraint: Constraint, value: u64) -> Option<Constraint> {
        match self {
            Self::LessThan => {
                if value < constraint.0 {
                    None
                } else if value < constraint.1 {
                    Some(Constraint(constraint.0, value - 1))
                } else {
                    Some(constraint)
                }
            }
            Self::LessThanEqual => {
                if value < constraint.0 {
                    None
                } else if value <= constraint.1 {
                    Some(Constraint(constraint.0, value))
                } else {
                    Some(constraint)
                }
            }
            Self::GreaterThan => {
                if value > constraint.1 {
                    None
                } else if value > constraint.0 {
                    Some(Constraint(value + 1, constraint.1))
                } else {
                    Some(constraint)
                }
            }
            Self::GreaterThanEqual => {
                if value > constraint.1 {
                    None
                } else if value >= constraint.0 {
                    Some(Constraint(value, constraint.1))
                } else {
                    Some(constraint)
                }
            }
            Self::True => Some(constraint),
            Self::False => None,
        }
    }

    pub fn invert(&self) -> Self {
        match self {
            Self::GreaterThan => Self::LessThanEqual,
            Self::GreaterThanEqual => Self::LessThan,
            Self::LessThan => Self::GreaterThanEqual,
            Self::LessThanEqual => Self::GreaterThan,
            Self::True => Self::False,
            Self::False => Self::True,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Rating {
    Cool,
    Musical,
    Aerodynamic,
    Shiny,
}

impl Rating {
    pub fn get<T: Copy>(&self, part: &Part<T>) -> T {
        match self {
            Self::Cool => part.cool,
            Self::Musical => part.musical,
            Self::Aerodynamic => part.aerodynamic,
            Self::Shiny => part.shiny,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Condition {
    Always,
    Never,
    When {
        rating: Rating,
        operation: Operation,
        value: u64,
    },
}

impl Condition {
    pub fn new(rating: Rating, operation: Operation, value: u64) -> Self {
        Self::When {
            operation,
            rating,
            value,
        }
    }

    pub fn eval(&self, part: &Part<u64>) -> bool {
        match self {
            Self::Always => true,
            Self::Never => false,
            Self::When {
                rating,
                operation,
                value,
            } => operation.eval(rating.get(part), *value),
        }
    }

    pub fn invert(&self) -> Self {
        match self {
            Self::Always => Self::Never,
            Self::Never => Self::Always,
            Self::When {
                rating,
                operation,
                value,
            } => Self::new(*rating, operation.invert(), *value),
        }
    }

    pub fn constrain(&self, part: &Part<Constraint>) -> Option<Part<Constraint>> {
        if let Self::Always = self {
            return Some(*part);
        } else if let Self::Never = self {
            return None;
        }

        let Self::When {
            rating,
            operation,
            value,
        } = self
        else {
            unreachable!()
        };

        let Part {
            cool,
            musical,
            aerodynamic,
            shiny,
        } = part;

        let new_constraint = operation.constrain(rating.get(part), *value)?;

        match rating {
            Rating::Cool => Some(Part {
                cool: new_constraint,
                musical: *musical,
                aerodynamic: *aerodynamic,
                shiny: *shiny,
            }),
            Rating::Musical => Some(Part {
                cool: *cool,
                musical: new_constraint,
                aerodynamic: *aerodynamic,
                shiny: *shiny,
            }),
            Rating::Aerodynamic => Some(Part {
                cool: *cool,
                musical: *musical,
                aerodynamic: new_constraint,
                shiny: *shiny,
            }),
            Rating::Shiny => Some(Part {
                cool: *cool,
                musical: *musical,
                aerodynamic: *aerodynamic,
                shiny: new_constraint,
            }),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Part<T: Copy> {
    pub cool: T,
    pub musical: T,
    pub aerodynamic: T,
    pub shiny: T,
}

impl<T: Copy> Part<T> {
    pub fn new(cool: T, musical: T, aerodynamic: T, shiny: T) -> Self {
        Self {
            cool,
            musical,
            aerodynamic,
            shiny,
        }
    }
}

impl Part<u64> {
    pub fn sum(&self) -> u64 {
        self.cool + self.musical + self.aerodynamic + self.shiny
    }
}

impl Part<Constraint> {
    pub fn product(&self) -> u64 {
        self.cool.size() * self.musical.size() * self.aerodynamic.size() * self.shiny.size()
    }

    pub fn overlap(&self, other: &Self) -> Option<Self> {
        let cool = self.cool.overlap(&other.cool)?;
        let musical = self.musical.overlap(&other.musical)?;
        let aerodynamic = self.aerodynamic.overlap(&other.aerodynamic)?;
        let shiny = self.shiny.overlap(&other.shiny)?;

        Some(Self {
            cool,
            musical,
            aerodynamic,
            shiny,
        })
    }
}

#[derive(Debug)]
pub enum WorkflowLabel {
    Accepted,
    Rejected,
    Named(String),
}

pub struct Workflow(pub String, pub Vec<Rule>);

#[derive(Debug)]
pub struct Rule(pub Condition, pub WorkflowLabel);

impl Rule {
    pub fn to_node(&self, workflows: &Workflows) -> Node {
        let children = match &self.1 {
            WorkflowLabel::Accepted => vec![Node::Accepted],
            WorkflowLabel::Rejected => vec![Node::Rejected],
            WorkflowLabel::Named(name) => {
                let rules = workflows.get(name).unwrap();

                rules.iter().map(|rule| rule.to_node(workflows)).collect()
            }
        };

        Node::Rule(self.0, children)
    }
}

pub struct Engine {
    parts: Vec<Part<u64>>,
    root: Node,
}

impl Engine {
    pub fn new(workflows: Vec<Workflow>, parts: Vec<Part<u64>>) -> Self {
        let workflows = convert_workflows(workflows);

        let root = Node::Root(
            workflows
                .get("in")
                .unwrap()
                .iter()
                .map(|rule| rule.to_node(&workflows))
                .collect(),
        );

        Self { parts, root }
    }

    pub fn solve(&self) -> u64 {
        self.parts
            .iter()
            .filter(|&part| self.root.is_accepted(part))
            .map(|part| part.sum())
            .sum()
    }

    pub fn ways_to_win(&self, constraint: Constraint) -> u64 {
        let parts = self
            .root
            .eval(&Part::new(constraint, constraint, constraint, constraint));

        let mut overlaps = Vec::new();

        for i in 0..parts.len() {
            for j in (i + 1)..parts.len() {
                let overlap = parts[i].overlap(&parts[j]);

                if let Some(overlap) = overlap {
                    overlaps.push(overlap);
                }
            }
        }

        parts.into_iter().map(|x| x.product()).sum::<u64>()
            - overlaps.into_iter().map(|x| x.product()).sum::<u64>()
    }
}

#[derive(Debug)]
pub enum Node {
    Accepted,
    Rejected,
    Rule(Condition, Vec<Node>),
    Root(Vec<Node>),
}

impl Node {
    pub fn invert(&self, part: &Part<Constraint>) -> Option<Part<Constraint>> {
        match self {
            Self::Rule(condition, _) => condition.invert().constrain(part),
            Self::Accepted => None,
            Self::Rejected => Some(*part),
            Self::Root(_) => Some(*part),
        }
    }

    fn process_nodes(part: &Part<Constraint>, nodes: &[Node]) -> Vec<Part<Constraint>> {
        let mut constrained = Some(*part);

        let mut results = Vec::new();
        for node in nodes.iter() {
            if constrained.is_none() {
                break;
            }

            let part = constrained.unwrap();
            let result = node.eval(&part);
            results.extend(result);
            constrained = node.invert(&part);
        }

        results
    }

    pub fn is_accepted(&self, part: &Part<u64>) -> bool {
        self.is_accepted_inner(part).unwrap_or(false)
    }

    fn is_accepted_inner(&self, part: &Part<u64>) -> Option<bool> {
        match self {
            Self::Accepted => Some(true),
            Self::Rejected => Some(false),
            Self::Rule(condition, children) => {
                if !condition.eval(part) {
                    return None;
                }

                for child in children {
                    let result = child.is_accepted_inner(part);

                    if result.is_some() {
                        return result;
                    }
                }

                None
            }
            Self::Root(children) => {
                for child in children {
                    let result = child.is_accepted_inner(part);
                    if result.is_some() {
                        return result;
                    }
                }

                None
            }
        }
    }

    pub fn eval(&self, part: &Part<Constraint>) -> Vec<Part<Constraint>> {
        match self {
            Self::Accepted => vec![*part],
            Self::Rejected => vec![],
            Self::Rule(condition, nodes) => {
                let constrained = condition.constrain(part);

                if let Some(part) = constrained {
                    Self::process_nodes(&part, nodes)
                } else {
                    vec![]
                }
            }
            Self::Root(nodes) => Self::process_nodes(part, nodes),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constraint_size() {
        assert_eq!(Constraint(1, 10).size(), 10);
    }

    #[test]
    fn test_parts_product() {
        let constraint = Constraint(1, 10);

        let part = Part::new(constraint, constraint, constraint, constraint);

        assert_eq!(part.product(), 10000);
    }

    fn test_constraint(
        constraint: Constraint,
        value: u64,
        operation: Operation,
        expected: Constraint,
    ) {
        let new_constraint = operation.constrain(constraint, value);

        assert!(new_constraint.is_some());

        let new_constraint = new_constraint.unwrap();

        assert_eq!(new_constraint.0, expected.0);
        assert_eq!(new_constraint.1, expected.1);
        assert_eq!(new_constraint.size(), expected.size());
    }

    #[test]
    fn test_lessthan_constraint() {
        let constraint = Constraint(1, 10);
        let value = 7;
        let operation = Operation::LessThan;
        let expected = Constraint(1, 6);

        test_constraint(constraint, value, operation, expected)
    }

    #[test]
    fn test_greaterthan_constraint() {
        let constraint = Constraint(1, 10);
        let value = 3;
        let operation = Operation::GreaterThan;
        let expected = Constraint(4, 10);

        test_constraint(constraint, value, operation, expected)
    }

    #[test]
    fn test_lessthan_equal_constraint() {
        let constraint = Constraint(1, 10);
        let value = 7;
        let operation = Operation::LessThanEqual;
        let expected = Constraint(1, 7);

        test_constraint(constraint, value, operation, expected)
    }

    #[test]
    fn test_greaterthan_equal_constraint() {
        let constraint = Constraint(1, 10);
        let value = 7;
        let operation = Operation::GreaterThanEqual;
        let expected = Constraint(7, 10);

        test_constraint(constraint, value, operation, expected)
    }

    #[test]
    fn test_lessthan_greater() {
        let constraint = Constraint(1, 10);
        let value = 11;
        let operation = Operation::LessThan;
        let expected = constraint;

        test_constraint(constraint, value, operation, expected)
    }

    #[test]
    fn test_lessthan_smaller() {
        let constraint = Constraint(3, 10);
        let value = 1;
        let operation = Operation::LessThan;

        assert!(operation.constrain(constraint, value).is_none());
    }

    #[test]
    fn test_greaterthan_greater() {
        let constraint = Constraint(1, 10);
        let value = 0;
        let operation = Operation::GreaterThan;
        let expected = constraint;

        test_constraint(constraint, value, operation, expected)
    }

    #[test]
    fn test_greaterthan_smaller() {
        let constraint = Constraint(1, 10);
        let value = 11;
        let operation = Operation::GreaterThan;

        assert!(operation.constrain(constraint, value).is_none());
    }

    #[test]
    fn test_condition_constraint() {
        let constraint = Constraint(1, 10);
        let part = Part::new(constraint, constraint, constraint, constraint);
        let condition = Condition::new(Rating::Cool, Operation::LessThan, 5);

        let constrained = condition.constrain(&part);

        assert!(constrained.is_some());

        let part = constrained.unwrap();
        assert_eq!(part.cool.0, 1);
        assert_eq!(part.cool.1, 4);
        assert_eq!(part.product(), 4 * 10 * 10 * 10);
    }

    #[test]
    fn test_condition_invert() {
        let condition = Condition::new(Rating::Cool, Operation::LessThan, 5);

        let constraint = Constraint(1, 10);
        let part = Part::new(constraint, constraint, constraint, constraint);

        let inverted = condition.invert().constrain(&part);

        assert!(inverted.is_some());

        let part = inverted.unwrap();

        assert_eq!(part.cool.0, 5);
        assert_eq!(part.cool.1, 10);
        assert_eq!(part.product(), 6 * 10 * 10 * 10);
    }

    #[test]
    fn test_node_with_multiple_conditions() {
        let constraint = Constraint(1, 10);
        let part = Part::new(constraint, constraint, constraint, constraint);

        let condition1 = Condition::new(Rating::Cool, Operation::LessThan, 5);
        let condition2 = Condition::new(Rating::Musical, Operation::GreaterThan, 5);

        let node1 = Node::Rule(condition1, vec![Node::Accepted]);
        let node2 = Node::Rule(condition2, vec![Node::Accepted]);

        let root = Node::Root(vec![node1, node2]);

        let expected_part_one = Part::new(Constraint(1, 4), constraint, constraint, constraint);
        let expected_part_two =
            Part::new(Constraint(5, 10), Constraint(6, 10), constraint, constraint);

        let results = root
            .eval(&part)
            .into_iter()
            .map(|part| part.product())
            .collect::<Vec<_>>();

        assert_eq!(results.len(), 2);
        assert_eq!(results[0], expected_part_one.product());
        assert_eq!(results[1], expected_part_two.product());
    }

    #[test]
    fn test_engine_nonoverlap() {
        let constraint = Constraint(1, 10);
        let condition = Condition::new(Rating::Cool, Operation::LessThan, 5);

        let workflows = vec![Workflow(
            "in".to_string(),
            vec![
                Rule(condition, WorkflowLabel::Accepted),
                Rule(Condition::Always, WorkflowLabel::Rejected),
            ],
        )];

        let engine = Engine::new(workflows, vec![]);

        let result = engine.ways_to_win(constraint);

        assert_eq!(result, 4 * 10 * 10 * 10);
    }
}
