/*
Set category is just a discrete category where objects are the elements of a set
and morphisms are the identity morphisms for each object.
 */
use crate::core::discrete_category::DiscreteCategory;

pub type SetCategory<Element> = DiscreteCategory<Element>;