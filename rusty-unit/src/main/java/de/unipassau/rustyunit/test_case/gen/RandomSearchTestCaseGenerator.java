package de.unipassau.rustyunit.test_case.gen;

import de.unipassau.rustyunit.Constants;
import de.unipassau.rustyunit.generators.TestIdGenerator;
import de.unipassau.rustyunit.hir.TyCtxt;
import de.unipassau.rustyunit.metaheuristics.chromosome.ChromosomeGenerator;
import de.unipassau.rustyunit.metaheuristics.operators.Crossover;
import de.unipassau.rustyunit.metaheuristics.operators.Mutation;
import de.unipassau.rustyunit.mir.MirAnalysis;
import de.unipassau.rustyunit.test_case.TestCase;
import de.unipassau.rustyunit.util.Rnd;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

public class RandomSearchTestCaseGenerator implements ChromosomeGenerator<TestCase> {

  private static final Logger logger = LoggerFactory.getLogger(RandomTestCaseGenerator.class);

  private final MirAnalysis<TestCase> mir;
  private final TyCtxt hir;
  private final Mutation<TestCase> mutation;
  private final Crossover<TestCase> crossover;

  public RandomSearchTestCaseGenerator(MirAnalysis<TestCase> mir, TyCtxt hir,
      Mutation<TestCase> mutation, Crossover<TestCase> crossover) {
    this.mir = mir;
    this.hir = hir;
    this.mutation = mutation;
    this.crossover = crossover;
  }

  @Override
  public TestCase get() {
    var testCase = new TestCase(TestIdGenerator.get(), hir, mutation, crossover, mir);
    var size =
        Rnd.get().nextDouble() * (Constants.CHROMOSOME_LENGTH - Constants.INITIAL_CHROMOSOME_LENGTH)
            + Constants.INITIAL_CHROMOSOME_LENGTH;
    while (testCase.size() < size) {
      testCase.insertRandomStmt();
    }
    logger.info("Generated test:\n{}", testCase);

    return testCase;
  }
}
