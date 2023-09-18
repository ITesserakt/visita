pub mod macros;
// mod vec_extensions;

pub mod extensions {
    // pub use crate::vec_extensions::*;
}

// responsible for routing the visit methods to the different nodes
pub trait NodeFamily<V>: Sized
where
    V: Visitor<Self>,
{
    type Data<'d>;

    fn accept_node<'d, N>(visitor: &mut V, node: &mut N, data: &Self::Data<'d>) -> V::Output
    where
        N: Node<Self, V>,
        V: Visit<Self, N>,
    {
        visitor.visit(node, data)
    }
}

// responsible for associating a node to a collection of nodes
pub trait Node<Family, V>: Sized
where
    V: Visitor<Family> + Visit<Family, Self>,
    Family: NodeFamily<V>,
{
    fn accept(&mut self, v: &mut V, data: &Data<Family, V>) -> <V as Visitor<Family>>::Output {
        v.visit(self, data)
    }
}

// responsible for dictating the output of traversing a group of nodes
pub trait Visitor<F>: Sized
where
    F: NodeFamily<Self>,
{
    type Output;
}

/// Implements the visiting logic for a node
pub trait Visit<Family, N>: Visitor<Family>
where
    N: Node<Family, Self>,
    Family: NodeFamily<Self>,
{
    fn visit(&mut self, node: &mut N, data: &Data<Family, Self>) -> Self::Output;
}

/// Shorthand for getting the data type from a node, as it can get quite verbose
pub type Data<'d, Family, V> = <Family as NodeFamily<V>>::Data<'d>;
