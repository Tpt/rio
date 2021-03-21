use rio_api::formatter::{QuadsFormatter, TriplesFormatter};
use rio_api::model::*;
use std::collections::{HashMap, HashSet};
use std::io;
use std::io::Write;

/// A [Canonical N-Triples](https://www.w3.org/TR/n-triples/#canonical-ntriples) formatter.
///
/// It implements the `TriplesFormatter` trait.
///
/// Write some triples using the `TriplesFormatter` API into a `Vec` buffer:
/// ```
/// use rio_turtle::NTriplesFormatter;
/// use rio_api::formatter::TriplesFormatter;
/// use rio_api::model::{NamedNode, Triple};
///
/// let mut formatter = NTriplesFormatter::new(Vec::default());
/// formatter.format(&Triple {
///     subject: NamedNode { iri: "http://example.com/foo" }.into(),
///     predicate: NamedNode { iri: "http://www.w3.org/1999/02/22-rdf-syntax-ns#type" }.into(),
///     object: NamedNode { iri: "http://schema.org/Person" }.into()
/// })?;
/// let _ntriples = formatter.finish();
/// # std::io::Result::Ok(())
/// ```
pub struct NTriplesFormatter<W: Write> {
    write: W,
}

impl<W: Write> NTriplesFormatter<W> {
    /// Builds a new formatter from a `Write` implementation
    pub fn new(write: W) -> Self {
        Self { write }
    }

    /// Finishes writing and returns the underlying `Write`
    pub fn finish(self) -> W {
        self.write
    }
}

impl<W: Write> TriplesFormatter for NTriplesFormatter<W> {
    type Error = io::Error;

    fn format(&mut self, triple: &Triple<'_>) -> Result<(), io::Error> {
        writeln!(self.write, "{}", triple)
    }
}

/// A [N-Quads](https://www.w3.org/TR/n-quads/) formatter.
///
/// It implements the `QuadsFormatter` trait.
///
/// Write some triples using the `QuadsFormatter` API into a `Vec` buffer:
/// ```
/// use rio_turtle::NQuadsFormatter;
/// use rio_api::formatter::QuadsFormatter;
/// use rio_api::model::{NamedNode, Quad};
///
/// let mut formatter = NQuadsFormatter::new(Vec::default());
/// formatter.format(&Quad {
///     subject: NamedNode { iri: "http://example.com/foo" }.into(),
///     predicate: NamedNode { iri: "http://www.w3.org/1999/02/22-rdf-syntax-ns#type" }.into(),
///     object: NamedNode { iri: "http://schema.org/Person" }.into(),
///     graph_name: Some(NamedNode { iri: "http://example.com/" }.into())
/// })?;
/// let _nquads = formatter.finish();
/// # std::io::Result::Ok(())
/// ```
pub struct NQuadsFormatter<W: Write> {
    write: W,
}

impl<W: Write> NQuadsFormatter<W> {
    /// Builds a new formatter from a `Write` implementation
    pub fn new(write: W) -> Self {
        Self { write }
    }

    /// Finishes writing and returns the underlying `Write`
    pub fn finish(self) -> W {
        self.write
    }
}

impl<W: Write> QuadsFormatter for NQuadsFormatter<W> {
    type Error = io::Error;

    fn format(&mut self, quad: &Quad<'_>) -> Result<(), io::Error> {
        writeln!(self.write, "{}", quad)
    }
}

/// A [Turtle](https://www.w3.org/TR/turtle/) formatter.
///
/// It implements the `TriplesFormatter` trait.
///
/// Write some triples using the `TriplesFormatter` API into a `Vec` buffer:
/// ```
/// use rio_turtle::TurtleFormatter;
/// use rio_api::formatter::TriplesFormatter;
/// use rio_api::model::{NamedNode, Triple};
///
/// let mut formatter = TurtleFormatter::new(Vec::default());
/// formatter.format(&Triple {
///     subject: NamedNode { iri: "http://example.com/foo" }.into(),
///     predicate: NamedNode { iri: "http://www.w3.org/1999/02/22-rdf-syntax-ns#type" }.into(),
///     object: NamedNode { iri: "http://schema.org/Person" }.into()
/// })?;
/// let _turtle = formatter.finish()?;
/// # std::io::Result::Ok(())
/// ```
pub struct TurtleFormatter<W: Write> {
    write: W,
    current_subject: String,
    current_subject_type: Option<NamedOrBlankNodeType>,
    current_predicate: String,
}

impl<W: Write> TurtleFormatter<W> {
    /// Builds a new formatter from a `Write` implementation
    pub fn new(write: W) -> Self {
        Self {
            write,
            current_subject: String::default(),
            current_subject_type: None,
            current_predicate: String::default(),
        }
    }

    /// Finishes writing and returns the underlying `Write`
    pub fn finish(mut self) -> Result<W, io::Error> {
        if self.current_subject_type.is_some() {
            writeln!(self.write, " .")?;
        }
        Ok(self.write)
    }
}

impl<W: Write> TriplesFormatter for TurtleFormatter<W> {
    type Error = io::Error;

    fn format(&mut self, triple: &Triple<'_>) -> Result<(), io::Error> {
        if let Some(current_subject_type) = self.current_subject_type {
            let current_subject = current_subject_type.with_value(&self.current_subject);
            if current_subject == triple.subject {
                if self.current_predicate == *triple.predicate.iri {
                    write!(self.write, " , {}", triple.object)?;
                } else {
                    write!(self.write, " ;\n\t{} {}", triple.predicate, triple.object)?;
                }
            } else {
                write!(
                    self.write,
                    " .\n{} {} {}",
                    triple.subject, triple.predicate, triple.object
                )?;
            }
        } else {
            write!(
                self.write,
                "{} {} {}",
                triple.subject, triple.predicate, triple.object
            )?;
        }

        self.current_subject.clear();
        match triple.subject {
            NamedOrBlankNode::NamedNode(n) => {
                self.current_subject.push_str(n.iri);
                self.current_subject_type = Some(NamedOrBlankNodeType::NamedNode);
            }
            NamedOrBlankNode::BlankNode(n) => {
                self.current_subject.push_str(n.id);
                self.current_subject_type = Some(NamedOrBlankNodeType::BlankNode);
            }
        }
        self.current_predicate.clear();
        self.current_predicate.push_str(triple.predicate.iri);

        Ok(())
    }
}

/// A [TriG](https://www.w3.org/TR/trig/) formatter.
///
/// It implements the `QuadsFormatter` trait.
///
/// Write some triples using the `QuadsFormatter` API into a `Vec` buffer:
/// ```
/// use rio_turtle::TriGFormatter;
/// use rio_api::formatter::QuadsFormatter;
/// use rio_api::model::{NamedNode, Quad};
///
/// let mut formatter = TriGFormatter::new(Vec::default());
/// formatter.format(&Quad {
///     subject: NamedNode { iri: "http://example.com/foo" }.into(),
///     predicate: NamedNode { iri: "http://www.w3.org/1999/02/22-rdf-syntax-ns#type" }.into(),
///     object: NamedNode { iri: "http://schema.org/Person" }.into(),
///     graph_name: Some(NamedNode { iri: "http://example.com/" }.into())
/// })?;
/// let _trig = formatter.finish()?;
/// # std::io::Result::Ok(())
/// ```
pub struct TriGFormatter<W: Write> {
    write: W,
    current_graph_name: String,
    current_graph_name_type: Option<Option<NamedOrBlankNodeType>>,
    current_subject: String,
    current_subject_type: Option<NamedOrBlankNodeType>,
    current_predicate: String,
}

impl<W: Write> TriGFormatter<W> {
    /// Builds a new formatter from a `Write` implementation
    pub fn new(write: W) -> Self {
        Self {
            write,
            current_graph_name: String::default(),
            current_graph_name_type: None,
            current_subject: String::default(),
            current_subject_type: None,
            current_predicate: String::default(),
        }
    }

    /// Finishes writing and returns the underlying `Write`
    pub fn finish(mut self) -> Result<W, io::Error> {
        if self.current_subject_type.is_some() {
            writeln!(self.write, " .")?;
        }
        if self.current_graph_name_type.and_then(|t| t).is_some() {
            writeln!(self.write, "}}")?;
        }
        Ok(self.write)
    }
}

impl<W: Write> QuadsFormatter for TriGFormatter<W> {
    type Error = io::Error;

    fn format(&mut self, quad: &Quad<'_>) -> Result<(), io::Error> {
        if let Some(current_graph_name_type) = self.current_graph_name_type {
            let current_graph_name =
                current_graph_name_type.map(|t| t.with_value(&self.current_graph_name));
            if current_graph_name == quad.graph_name {
                if let Some(current_subject_type) = self.current_subject_type {
                    let current_subject = current_subject_type.with_value(&self.current_subject);
                    if current_subject == quad.subject {
                        if self.current_predicate == *quad.predicate.iri {
                            write!(self.write, " , {}", quad.object)?;
                        } else {
                            write!(self.write, " ;\n\t\t{} {}", quad.predicate, quad.object)?;
                        }
                    } else {
                        write!(
                            self.write,
                            " .\n\t{} {} {}",
                            quad.subject, quad.predicate, quad.object
                        )?;
                    }
                } else {
                    write!(
                        self.write,
                        "{} {} {}",
                        quad.subject, quad.predicate, quad.object
                    )?;
                }
            } else {
                if self.current_graph_name_type.and_then(|t| t).is_some() {
                    writeln!(self.write, " .\n}}")?;
                } else {
                    writeln!(self.write, " .")?;
                }
                if let Some(graph_name) = quad.graph_name {
                    write!(
                        self.write,
                        "{} {{\n\t{} {} {}",
                        graph_name, quad.subject, quad.predicate, quad.object
                    )?;
                } else {
                    write!(
                        self.write,
                        "{} {} {}",
                        quad.subject, quad.predicate, quad.object
                    )?;
                }
            }
        } else if let Some(graph_name) = quad.graph_name {
            write!(
                self.write,
                "{} {{\n\t{} {} {}",
                graph_name, quad.subject, quad.predicate, quad.object
            )?;
        } else {
            write!(
                self.write,
                "{} {} {}",
                quad.subject, quad.predicate, quad.object
            )?;
        }

        self.current_graph_name.clear();
        match quad.graph_name {
            Some(NamedOrBlankNode::NamedNode(n)) => {
                self.current_graph_name.push_str(n.iri);
                self.current_graph_name_type = Some(Some(NamedOrBlankNodeType::NamedNode));
            }
            Some(NamedOrBlankNode::BlankNode(n)) => {
                self.current_graph_name.push_str(n.id);
                self.current_graph_name_type = Some(Some(NamedOrBlankNodeType::BlankNode));
            }
            None => self.current_graph_name_type = Some(None),
        }
        self.current_subject.clear();
        match quad.subject {
            NamedOrBlankNode::NamedNode(n) => {
                self.current_subject.push_str(n.iri);
                self.current_subject_type = Some(NamedOrBlankNodeType::NamedNode);
            }
            NamedOrBlankNode::BlankNode(n) => {
                self.current_subject.push_str(n.id);
                self.current_subject_type = Some(NamedOrBlankNodeType::BlankNode);
            }
        }
        self.current_predicate.clear();
        self.current_predicate.push_str(quad.predicate.iri);

        Ok(())
    }
}

#[derive(Copy, Clone)]
enum NamedOrBlankNodeType {
    NamedNode,
    BlankNode,
}

impl NamedOrBlankNodeType {
    fn with_value<'a>(&self, value: &'a str) -> NamedOrBlankNode<'a> {
        match self {
            NamedOrBlankNodeType::NamedNode => NamedNode { iri: value }.into(),
            NamedOrBlankNodeType::BlankNode => BlankNode { id: value }.into(),
        }
    }
}

/// A [Turtle](https://www.w3.org/TR/turtle/) formatter.
///
/// Write some triples into a `Vec` buffer:
/// ```
/// use rio_turtle::format_pretty_turtle;
/// use rio_api::model::{NamedNode, Triple, BlankNode};
/// use std::rc::Rc;
///
/// let foo = Rc::new("http://example.com/foo");
/// let triples = vec![
///     Triple {
///         subject: NamedNode { iri: &foo }.into(),
///         predicate: NamedNode { iri: "http://schema.org/parent" },
///         object: BlankNode { id: "11" }.into()
///     },
///     Triple {
///         subject: BlankNode { id: "11" }.into(),
///         predicate: NamedNode { iri: "http://www.w3.org/1999/02/22-rdf-syntax-ns#type" },
///         object: NamedNode { iri: "http://schema.org/Person" }.into()
///     },
///     Triple {
///         subject: NamedNode { iri: &foo }.into(),
///         predicate: NamedNode { iri: "http://www.w3.org/1999/02/22-rdf-syntax-ns#type" },
///         object: NamedNode { iri: "http://schema.org/Person" }.into()
///     }
/// ];
///
/// let mut buffer = Vec::new();
/// format_pretty_turtle(&mut buffer, triples.iter().cloned())?;
/// # std::io::Result::Ok(())
/// ```
pub fn format_pretty_turtle<'a>(
    mut writer: impl Write,
    triples: impl IntoIterator<Item = Triple<'a>>,
) -> Result<(), io::Error> {
    let mut tree: HashMap<NamedOrBlankNode<'a>, HashMap<NamedNode<'a>, HashSet<Term<'a>>>> =
        HashMap::new(); // Subject -> predicte -> objects map
    for triple in triples {
        tree.entry(triple.subject)
            .or_insert_with(HashMap::default)
            .entry(triple.predicate)
            .or_insert_with(HashSet::default)
            .insert(triple.object);
    }
    for (subject, predicate_objects) in &tree {
        write!(writer, "{}", subject)?;
        for (i, (predicate, objects)) in predicate_objects.iter().enumerate() {
            if i > 0 {
                write!(writer, " ;\n\t")?;
            }
            write!(writer, "\t{}", predicate)?;
            for (j, object) in objects.iter().enumerate() {
                if j > 0 {
                    write!(writer, " ,\n")?;
                }
                write!(writer, " {}", object)?;
            }
        }
        write!(writer, " .\n")?;
    }
    Ok(())
}
