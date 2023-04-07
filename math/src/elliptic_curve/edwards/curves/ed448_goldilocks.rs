use crate::{
    elliptic_curve::{
        edwards::{point::EdwardsProjectivePoint, traits::IsEdwards},
        traits::IsEllipticCurve,
    },
    field::{
        element::FieldElement,
        fields::p448_goldilocks_prime_field::{P448GoldilocksPrimeField, U56x8},
    },
};

#[derive(Debug, Clone)]
pub struct Ed448Goldilocks;

impl IsEllipticCurve for Ed448Goldilocks {
    type BaseField = P448GoldilocksPrimeField;
    type PointRepresentation = EdwardsProjectivePoint<Self>;

    fn generator() -> Self::PointRepresentation {
        Self::PointRepresentation::new([
            FieldElement::from(&U56x8::from("5")),
            FieldElement::from(&U56x8::from("ff93c7b127f5bf67e11838cea6c3258a236fc4e8530d573ba9712e5ad1e07a623e499f6c6f78b8a44087f4ff0ddf8f08863912d72673b870")),
            FieldElement::one(),
        ])
    }
}

impl IsEdwards for Ed448Goldilocks {
    fn a() -> FieldElement<Self::BaseField> {
        FieldElement::from(1)
    }

    fn d() -> FieldElement<Self::BaseField> {
        -FieldElement::from(39081)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::field::element::FieldElement;

    #[test]
    fn generator_satisfies_defining_equation() {
        let g = Ed448Goldilocks::generator().to_affine();
        assert_eq!(
            Ed448Goldilocks::defining_equation(g.x(), g.y()),
            FieldElement::zero()
        );
    }
}
