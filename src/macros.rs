#[macro_export]
macro_rules! node_group {
        {family: $family:ty, nodes: [$($node:ty$(,)*)+], meta: $meta:ty} => {
            impl<V: $crate::Visitor<$family>> $crate::NodeFamily<V> for $family {
                type Data<'d> = $meta;
            }

            $(impl<V: $crate::Visitor<$family> + $crate::Visit<Self>> $crate::Node<V> for $node {
                type Family = $family;
            })+
        };
    }

#[macro_export]
macro_rules! impl_visitor {
        {$self:ty, family: $family:ty, output: $output:ty, [$($node:ty => $visits:expr$(,)*)+]} => {
            impl $crate::Visitor<$family> for $self {
                type Output = $output;
            }

            $(impl $crate::Visit<$node> for $self {
                #[allow(unused_variables)]
                fn visit(&mut self, node: &mut $node, metadata: &$crate::Data<Self, $node>) -> Self::Output {
                    let closure: fn(&mut Self, &mut $node, &$crate::Data<Self, $node>) -> Self::Output = $visits;
                    #[allow(clippy::redundant_closure_call)]
                    closure(self, node, metadata)
                }
            })+
        };
    }
