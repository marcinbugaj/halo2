mod prover;
mod verifier;

pub use prover::ProverGWC;
pub use verifier::VerifierGWC;

use crate::{
    arithmetic::{eval_polynomial, CurveAffine},
    poly::{
        commitment::{Params, ParamsVerifier},
        query::Query,
        Coeff, Polynomial,
    },
    transcript::ChallengeScalar,
};
use ff::Field;
use std::{
    collections::{BTreeMap, BTreeSet},
    marker::PhantomData,
};

#[derive(Clone, Copy, Debug)]
struct U {}
type ChallengeU<F> = ChallengeScalar<F, U>;

#[derive(Clone, Copy, Debug)]
struct V {}
type ChallengeV<F> = ChallengeScalar<F, V>;

#[derive(Debug)]
pub struct CommitmentData<F: Field, Q: Query<F>> {
    pub queries: Vec<Q>,
    pub point: F,
    pub _marker: PhantomData<F>,
}

pub fn construct_intermediate_sets<F: Field, I, Q: Query<F>>(queries: I) -> Vec<CommitmentData<F, Q>>
where
    I: IntoIterator<Item = Q> + Clone,
{
    let mut point_query_map: Vec<(F, Vec<Q>)> = Vec::new();
    for query in queries {
        if let Some(pos) = point_query_map
            .iter()
            .position(|(point, _)| *point == query.get_point())
        {
            let (_, queries) = &mut point_query_map[pos];
            queries.push(query);
        } else {
            point_query_map.push((query.get_point(), vec![query]));
        }
    }

    point_query_map
        .into_iter()
        .map(|(point, queries)| CommitmentData {
            queries,
            point,
            _marker: PhantomData,
        })
        .collect()
}
