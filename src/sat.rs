use crate::{
    parser::SwInt,
    structures::{FormulaConjunctiveBasic, SwUint},
    FormulaTranslator,
};

pub trait SatFormula {
    // Core algorithmic logic lives here
    fn is_sat(&self) -> bool;
}

impl<T: SwUint> SatFormula for FormulaConjunctiveBasic<T> {
    fn is_sat(&self) -> bool {
        todo!()
    }
}

impl<K: SwInt, V: SwUint> SatFormula for FormulaTranslator<K, V> {
    fn is_sat(&self) -> bool {
        self.cnf.is_sat()
    }
}
