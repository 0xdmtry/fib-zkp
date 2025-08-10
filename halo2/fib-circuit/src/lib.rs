use halo2_proofs::{
    circuit::{AssignedCell, Layouter, SimpleFloorPlanner},
    plonk::{Advice, Circuit, Column, ConstraintSystem, Error, Instance, Selector},
    pasta::Fp,
    poly::Rotation
};

/// Config of the Fibanacci circuit
#[derive(Debug, Clone)]
pub struct FibonacciConfig {
    /// 3 advice collumns for a, b, c values 
    pub advice: [Column<Advice>; 3],
    /// Instance column fpr public inputs
    pub instance: Column<Instance>,
    /// Selector to enable Fibanacci gate
    pub selector: Selector,
}

/// Chip to implement Fibonacci logic
pub struct FibonacciChip {
    config: FibonacciConfig,
}

/// A complete Fibonacci circuit
#[derive(Default)]
pub struct FibonacciCircuit {
    /// The private starting values for the sequence
    pub a: Fp,
    pub b: Fp
}

impl FibonacciChip {

    /// Create Fibonacci chip
    pub fn construct(config: FibonacciConfig) -> Self {
        Self {config}
    }

    /// Configures the constraints for circuit
    pub fn configure(meta: &mut ConstraintSystem<Fp>) -> FibonacciConfig {
        let col_a = meta.advice_column();
        let col_b = meta.advice_column();
        let col_c = meta.advice_column();
        let instance = meta.instance_column();
        let selector = meta.selector();

        // Enable equality for columns to allow for row-to-row copying
        meta.enable_equality(col_a);
        meta.enable_equality(col_b);
        meta.enable_equality(col_c);
        meta.enable_equality(instance);

        // Define the Fibonacci gate: a + b = c
        meta.create_gate("add", |meta| {
            let s = meta.query_selector(selector);
            let a = meta.query_advice(col_a, Rotation::cur());
            let b = meta.query_advice(col_b, Rotation::cur());
            let c = meta.query_advice(col_c, Rotation::cur());

            vec![s * (a + b - c)]
        });

        FibonacciConfig {
            advice: [col_a, col_b, col_c],
            instance,
            selector
        }
    }

    /// Assigns the witness values to the circuit
    pub fn assign(&self, 
                layouter: &mut impl Layouter<Fp>, 
                a_val: Fp, 
                b_val: Fp) -> Result<AssignedCell<Fp, Fp>, Error> {
        
                    layouter.assign_region(
                                            || "fibonacci sequence", 
                                            |mut region| {
                                                        // Assign the first row
                                                        self.config.selector.enable(&mut region, 0)?;
                                                        let mut prev_a = region.assign_advice(
                                                            || "a_0",
                                                            self.config.advice[0], 
                                                            0, 
                                                            || halo2_proofs::circuit::Value::known(a_val)
                                                        )?;
            
                                                        let mut prev_b = region.assign_advice(
                                                            || "b_0",
                                                            self.config.advice[1],
                                                            0,
                                                            || halo2_proofs::circuit::Value::known(b_val)
                                                        )?;
            
                                                        let mut prev_c = region.assign_advice(
                                                            || "c_0",
                                                            self.config.advice[2],
                                                            0,
                                                            || halo2_proofs::circuit::Value::known(a_val + b_val)
                                                        )?;
            
                                                        // Assign the rest of the rows (9 more steps)
                                                        for i in 1..10 {
                                                            self.config.selector.enable(&mut region, i)?;

                                                            let a = prev_b.copy_advice(|| "a_i", &mut region, self.config.advice[0], i)?;
                                                            let b = prev_c.copy_advice(|| "b_i", &mut region, self.config.advice[1], i)?;

                                                            let c_val = a.value().copied() + b.value().copied();

                                                            let c = region.assign_advice(|| "c_i", self.config.advice[2], i, || c_val)?;

                                                            // Update the "previous" cells for the next iteration
                                                            prev_a = a;
                                                            prev_b = b;
                                                            prev_c = c;
                                                        }

                                                        // Expose the final `a` value as a public input
                                                        // region.constrain_instance(prev_a.cell(), self.config.instance, 0)?;

                                                        // Expose the final `a` value as a public input
                                                        Ok(prev_a)
                                                })
    }
}

impl Circuit<Fp> for FibonacciCircuit {
    type Config = FibonacciConfig;
    type FloorPlanner = SimpleFloorPlanner;

    fn without_witnesses(&self) -> Self {
        Self::default()
    }

    fn configure(meta: &mut ConstraintSystem<Fp>) -> Self::Config {
        FibonacciChip::configure(meta)
    }

    fn synthesize(
        &self,
        config: Self::Config,
        mut layouter: impl Layouter<Fp>
    ) -> Result<(), Error> {
        let chip = FibonacciChip::construct(config.clone());
        let final_a = chip.assign(&mut layouter, self.a, self.b)?;
        layouter.constrain_instance(final_a.cell(), config.instance, 0)?;
        Ok(())
    }
}