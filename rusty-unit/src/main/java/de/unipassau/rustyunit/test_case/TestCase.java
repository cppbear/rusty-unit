package de.unipassau.rustyunit.test_case;

import static de.unipassau.rustyunit.Constants.P_LOCAL_VARIABLES;
import static de.unipassau.rustyunit.util.TypeUtil.getTypeFor;
import static java.util.stream.Collectors.toCollection;

import de.unipassau.rustyunit.Constants;
import de.unipassau.rustyunit.generators.TestIdGenerator;
import de.unipassau.rustyunit.hir.TyCtxt;
import de.unipassau.rustyunit.metaheuristics.chromosome.AbstractTestCaseChromosome;
import de.unipassau.rustyunit.metaheuristics.fitness_functions.MinimizingFitnessFunction;
import de.unipassau.rustyunit.metaheuristics.operators.Crossover;
import de.unipassau.rustyunit.metaheuristics.operators.Mutation;
import de.unipassau.rustyunit.mir.BasicBlock;
import de.unipassau.rustyunit.mir.MirAnalysis;
import de.unipassau.rustyunit.test_case.callable.ArrayInit;
import de.unipassau.rustyunit.test_case.callable.Callable;
import de.unipassau.rustyunit.test_case.callable.Method;
import de.unipassau.rustyunit.test_case.callable.RefItem;
import de.unipassau.rustyunit.test_case.callable.TupleAccess;
import de.unipassau.rustyunit.test_case.callable.TupleInit;
import de.unipassau.rustyunit.test_case.metadata.MetaData;
import de.unipassau.rustyunit.test_case.metadata.TestCaseMetadata;
import de.unipassau.rustyunit.test_case.primitive.PrimitiveValue;
import de.unipassau.rustyunit.test_case.seed.SeedOptions;
import de.unipassau.rustyunit.test_case.statement.PrimitiveStmt;
import de.unipassau.rustyunit.test_case.statement.Statement;
import de.unipassau.rustyunit.type.Array;
import de.unipassau.rustyunit.type.Generic;
import de.unipassau.rustyunit.type.Ref;
import de.unipassau.rustyunit.type.Slice;
import de.unipassau.rustyunit.type.Tuple;
import de.unipassau.rustyunit.type.Type;
import de.unipassau.rustyunit.type.TypeBinding;
import de.unipassau.rustyunit.type.prim.Prim;
import de.unipassau.rustyunit.type.traits.std.default_.Default;
import de.unipassau.rustyunit.type.traits.std.marker.Copy;
import de.unipassau.rustyunit.test_case.var.Index;
import de.unipassau.rustyunit.test_case.var.VarReference;
import de.unipassau.rustyunit.test_case.visitor.LineNumberDebugVisitor;
import de.unipassau.rustyunit.test_case.visitor.TypeBindingStringVisitor;
import de.unipassau.rustyunit.test_case.visitor.Visitor;
import de.unipassau.rustyunit.util.Rnd;
import de.unipassau.rustyunit.util.TypeUtil;
import java.util.ArrayList;
import java.util.Collections;
import java.util.HashMap;
import java.util.HashSet;
import java.util.LinkedHashSet;
import java.util.List;
import java.util.Map;
import java.util.Map.Entry;
import java.util.Objects;
import java.util.Optional;
import java.util.Set;
import java.util.stream.Collectors;
import java.util.stream.IntStream;
import java.util.stream.Stream;
import org.javatuples.Pair;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

public class TestCase extends AbstractTestCaseChromosome<TestCase> {

  private static final Logger logger = LoggerFactory.getLogger(TestCase.class);
  private final TyCtxt tyCtxt;
  private int id;
  private List<Statement> statements;
  private Map<MinimizingFitnessFunction<TestCase>, Double> coverage;
  private final MirAnalysis<TestCase> mir;
  private final TestCaseMetadata metadata;

  public TestCase(int id, TyCtxt tyCtxt, Mutation<TestCase> mutation,
      Crossover<TestCase> crossover, MirAnalysis<TestCase> mir) {
    super(mutation, crossover);

    this.id = id;
    this.tyCtxt = tyCtxt;
    this.statements = new ArrayList<>();
    this.coverage = new HashMap<>();
    this.mir = mir;
    this.metadata = new TestCaseMetadata(id);
  }

  public TestCase(TestCase other) {
    super(other.getMutation(), other.getCrossover());
    this.id = TestIdGenerator.get();
    this.tyCtxt = other.tyCtxt;
    this.statements = other.statements.stream().map(s -> s.copy(this))
        .collect(toCollection(ArrayList::new));
    this.coverage = new HashMap<>();
    this.mir = other.mir;
    this.metadata = new TestCaseMetadata(id);
  }

  public void cleanup() {
    var remove = statements.stream().filter(stmt -> {
      if (stmt.isPrimitiveStmt()) {
        var returnValue = stmt.returnValue().get();
        var usedAt = returnValue.usedAt();
        return usedAt.isEmpty();
      }

      return false;
    }).toList();

    statements.removeAll(remove);
  }

  public Set<String> getUsedTraitNames() {
    return statements.stream()
        .filter(s -> s.isMethodStmt() || s.isStaticMethodStmt())
        .map(s -> {
          if (s.isStaticMethodStmt()) {
            return s.asStaticMethodStmt().ofTrait();
          } else if (s.isMethodStmt()) {
            return s.asMethodStmt().ofTrait();
          } else {
            throw new RuntimeException("No trait");
          }
        })
        .filter(Optional::isPresent)
        .map(Optional::get)
        .collect(Collectors.toSet());

  }

  public TyCtxt getHirAnalysis() {
    return tyCtxt;
  }

  @Override
  public int getId() {
    return id;
  }

  public void setId(int id) {
    this.id = id;
  }

  public MirAnalysis<TestCase> mir() {
    return mir;
  }

  @Override
  public int size() {
    return statements.size();
  }

  public List<Statement> getStatements() {
    return statements;
  }

  @Override
  public MetaData metadata() {
    return metadata;
  }

  @Override
  public Set<MinimizingFitnessFunction<TestCase>> coveredObjectives() {
    return coverage.entrySet().stream().filter(e -> e.getValue() == 0.0).map(Entry::getKey)
        .collect(Collectors.toSet());
  }

  @Override
  public Set<MinimizingFitnessFunction<TestCase>> uncoveredObjectives() {
    return coverage.entrySet().stream().filter(e -> e.getValue() != 0.0).map(Entry::getKey)
        .collect(Collectors.toSet());
  }

  public Optional<Statement> getLastCrateStmt() {
    return IntStream.range(0, statements.size())
        .mapToObj(i -> statements.get(statements.size() - i - 1))
        .filter(s -> s.getSrcFilePath() != null)
        .findFirst();
  }

  public Optional<String> getFilePathBinding() {
    var conflictingStmts = statements.stream().filter(s -> s.getSrcFilePath() != null)
        .filter(s -> !s.isPublic())
        .collect(Collectors.toSet());

    // Two statements can have the same private path binding which is valid
    var conflictingPaths = conflictingStmts.stream()
        .map(Statement::getSrcFilePath)
        .collect(Collectors.toSet());

    while (conflictingPaths.size() > 1) {
      var stmt = Rnd.choice(conflictingStmts);
      conflictingPaths.remove(stmt.getSrcFilePath());
      var toRemove = conflictingStmts.stream()
          .filter(s -> s.getSrcFilePath().equals(stmt.getSrcFilePath()))
          .collect(Collectors.toSet());
      conflictingStmts.removeAll(toRemove);
      toRemove.forEach(this::removeStmt);
    }

    return conflictingStmts.stream().findFirst().map(Statement::getSrcFilePath);
  }

  public boolean isValid() {
    return statements.stream().filter(s -> s.getSrcFilePath() != null)
        .filter(s -> !s.isPublic())
        .map(Statement::getSrcFilePath)
        .collect(Collectors.toSet())
        .size() <= 1;
  }

  public void setStatements(List<Statement> statements) {
    this.statements = statements;
  }

  public void setCoverage(BasicBlock branch, double distance) {
    coverage.put(branch, distance);
  }

  public void setCoverage(Map<MinimizingFitnessFunction<TestCase>, Double> coverage) {
    if (coverage == null) {
      return;
    }
    this.coverage = coverage;
  }

  public boolean isCutable() {
    return statements.size() > 1;
  }

  public Optional<VarReference> satisfyParameter(int pos, Type parameter) {
    List<VarReference> usableVariables;
    if (parameter.isRef()) {
      usableVariables = borrowableVariablesOfType(parameter, pos);
    } else {
      usableVariables = consumableVariablesOfType(parameter, pos);
    }

    VarReference var;
    if (!usableVariables.isEmpty()) {
      var = Rnd.choice(usableVariables);
    } else {
      var generatedArg = generateArg(parameter, null, getFilePathBinding().orElse(null));
      if (generatedArg.isPresent()) {
        var = generatedArg.get();
      } else {
        logger.warn("Could not generate any argument for " + parameter);
        return Optional.empty();
      }
    }
    return Optional.of(var);
  }

  /**
   * We assume that all the variables used in the statement do not exist in this
   * test case, because
   * the stmt comes from another one.
   */
  public List<VarReference> satisfyParameters(int pos, Statement stmt) {
    var paramTypes = stmt.actualParamTypes();

    List<VarReference> variables = new ArrayList<>(paramTypes.size());
    for (Type paramType : paramTypes) {
      var var = satisfyParameter(pos, paramType);

      // Don't use the same variable twice in an invocation
      if (var.isPresent() && !variables.contains(var.get())) {
        variables.add(var.get());
      }
    }

    return variables;
  }

  public void insertStmt(int pos, Statement stmt) {
    statements.add(pos, stmt);
  }

  public void addStmt(Statement stmt) {
    int insertPosition = 0;
    if (stmt.args().isEmpty()) {
      // Insert position is 0
      insertStmt(0, stmt);
    } else {
      insertPosition = stmt.args().stream().map(VarReference::position)
          .reduce(0, Integer::max);
      insertStmt(Integer.min(size(), insertPosition + 1), stmt);
    }
  }

  public void appendStmt(Statement stmt) {
    statements.add(stmt);
  }

  public void removeAllStmts() {
    statements.clear();
  }

  public int removeStmt(Statement stmt) {
    if (!statements.contains(stmt)) {
      // Probably removed before as it was a dependency of another stmt
      return 0;
    }

    if (!stmt.returnsValue()) {
      statements.remove(stmt);
      return 0;
    }

    var returnValue = stmt.returnValue().orElseThrow();
    var usingStmts = returnValue.usedAt()
        .stream()
        .map(this::stmtAt)
        .map(Optional::orElseThrow)
        .collect(toCollection(ArrayList::new));
    usingStmts.remove(stmt);
    Collections.reverse(usingStmts);
    usingStmts.forEach(this::removeStmt);

    statements.remove(stmt);
    return 0;
  }

  public Optional<Statement> stmtAt(int pos) {
    if (pos >= size() || pos < 0) {
      return Optional.empty();
    }

    return Optional.of(statements.get(pos));
  }

  public Optional<Integer> stmtPosition(Statement stmt) {
    var pos = statements.indexOf(stmt);
    if (pos < 0) {
      logger.warn("({}) Could not find position of a statement in test", id);
    }
    return Optional.of(pos);
  }

  public Optional<Integer> varPosition(VarReference var) {
    throw new RuntimeException("Not implemented");
  }

  public String getName() {
    return String.format("%s_%d", Constants.TEST_PREFIX, id);
  }

  public VarReference referenceVariable(VarReference variable, boolean mutable) {
    if (variable.testCase() != this) {
      throw new IllegalStateException("The test does not contain this variable");
    }

    if (variable.type().isRef()) {
      throw new RuntimeException("Referencing variable cannot be referenced");
    }

    RefItem refItem;
    if (mutable) {
      refItem = RefItem.MUTABLE;
    } else {
      refItem = RefItem.IMMUTABLE;
    }

    var typeBinding = new TypeBinding();
    typeBinding.bindGeneric(RefItem.T, variable.type());

    var returnType = new Ref(variable.type(), mutable);
    var returnValue = createVariable(returnType);
    returnValue.setBinding(typeBinding);

    var stmt = refItem.toStmt(this, Collections.singletonList(variable), returnValue);
    addStmt(stmt);
    return returnValue;
  }

  public Set<Type> instantiatedTypes() {
    return statements.stream()
        .map(Statement::returnValue)
        .filter(Optional::isPresent)
        .map(Optional::get)
        .map(VarReference::type)
        .collect(Collectors.toSet());
  }

  public List<VarReference> variables() {
    return statements.stream()
        .map(Statement::returnValue)
        .filter(Optional::isPresent)
        .map(Optional::get)
        .collect(toCollection(ArrayList::new));
  }

  public List<VarReference> borrowableVariablesOfType(Type type, int untilPos) {
    return statements.stream().limit(untilPos)
        .map(Statement::returnValue)
        .filter(Optional::isPresent)
        .map(Optional::get)
        .filter(var -> var.type().equals(type) && var.isBorrowableAt(untilPos))
        .collect(toCollection(ArrayList::new));
  }

  public List<VarReference> consumableVariablesOfType(Type type, int untilPos) {
    return statements.stream().limit(untilPos)
        .map(Statement::returnValue)
        .filter(Optional::isPresent)
        .map(Optional::get)
        .filter(var -> var.type().equals(type) && var.isConsumableAt(untilPos))
        .collect(toCollection(ArrayList::new));
  }

  public List<VarReference> unconsumedVariablesOfType(Type type) {
    return statements.stream()
        .map(Statement::returnValue)
        .filter(Optional::isPresent)
        .map(Optional::get)
        .filter(var -> var.type().equals(type) && !var.isConsumed())
        .collect(toCollection(ArrayList::new));
  }

  /**
   * Get all defined variables of a type.
   */
  public List<VarReference> variablesOfType(Type type) {
    return statements.stream()
        .map(Statement::returnValue)
        .filter(Optional::isPresent)
        .map(Optional::get)
        .filter(var -> var.type().equals(type))
        .collect(toCollection(ArrayList::new));
  }

  /**
   * Get defined variables of a type until the given position (exclusive).
   */
  public List<VarReference> variablesOfType(Type type, int pos) {
    if (pos == 0) {
      return new ArrayList<>();
    }

    return IntStream.range(0, pos)
        .mapToObj(i -> statements.get(i).returnValue())
        .filter(Optional::isPresent)
        .map(Optional::get)
        .filter(var -> var.type().equals(type))
        .collect(toCollection(ArrayList::new));
  }

  public Optional<VarReference> insertRandomStmt() {
    var filePathBinding = getFilePathBinding();
    Callable callable;

    var possibleMethods = tyCtxt.methodsOf(variables());
    if (Rnd.get().nextDouble() < P_LOCAL_VARIABLES && !possibleMethods.isEmpty()) {
      var variableAndMethod = Rnd.choice(possibleMethods);
      // var variableAndMethod = CallableSelector.select(possibleMethods);
      return insertMethodOnExistingVariable(variableAndMethod.getValue0(),
          variableAndMethod.getValue1(), filePathBinding.orElse(null));
    } else if (filePathBinding.isPresent()) {
      // callable = Rnd.choice(tyCtxt.getCallables(filePathBinding.get(), true));
      callable = CallableSelector.select(tyCtxt.getCallables(filePathBinding.get(), true));
    } else {
      // callable = Rnd.choice(tyCtxt.getCallables(null, true));
      callable = CallableSelector.select(tyCtxt.getCallables(null, true));
    }

    logger.info("({}) Inserting random stmt. Selected callable: {}", id, callable);
    String callableName = callable.getName();
    // if (callableName == "crate::header::map::VacantEntry::key(&
    // header::map::VacantEntry) -> &crate::header::name::HeaderName"
    // || callableName ==
    // "crate::header::map::OccupiedEntry::into_mut(header::map::OccupiedEntry) ->
    // &mut T"
    // || callableName == "std::fmt::Debug::fmt(& header::map::Values, mut &mut
    // std::fmt::Formatter) -> ()"
    // || callableName == "std::fmt::Debug::fmt(& header::map::ValuesMut, mut &mut
    // std::fmt::Formatter) -> ()"
    // || callableName == "std::clone::Clone::clone(& header::map::Bucket) ->
    // crate::header::map::Bucket<T: std::clone::Clone + std::clone::Clone>") {
    // return Optional.empty();
    // }

    return insertCallable(callable, filePathBinding.orElse(null));
  }

  private Optional<VarReference> insertAccessOnExistingTupleVariable(VarReference owner, int at,
      TupleAccess access) {
    LinkedHashSet<Generic> generics = new LinkedHashSet<>(
        TypeUtil.getDeepGenerics(access.getParent()));
    if (access.returnsValue()) {
      generics.addAll(TypeUtil.getDeepGenerics(access.getReturnType()));
    }

    var ownerTypeBinding = TypeBinding.fromTypes(access.getParent(), owner.type());
    var genericsTypeBinding = new TypeBinding(generics);
    var typeBinding = ownerTypeBinding.leftOuterMerge(genericsTypeBinding);
    typeBinding.getUnboundGenerics().stream().map(g -> Pair.with(g, getTypeFor(g)))
        .filter(p -> p.getValue1().isPresent())
        .forEach(p -> typeBinding.bindGeneric(p.getValue0(), p.getValue1().get()));

    if (typeBinding.hasUnboundedGeneric()) {
      logger.warn("({}) Could not bind all generics: {}", id,
          typeBinding.getUnboundGenerics());
      return Optional.empty();
    }

    var index = new Index(this, at);
    var returnValue = createVariable(access.getReturnType().bindGenerics(typeBinding));
    returnValue.setBinding(typeBinding);

    var stmt = access.toStmt(this, List.of(owner, index), returnValue);
    addStmt(stmt);
    return Optional.of(returnValue);
  }

  private Optional<VarReference> insertMethodOnExistingVariable(VarReference owner,
      Method method, String filePathBinding) {
    logger.info("({}) Inserting a method on existing variable {}", id, owner);
    var args = new ArrayList<VarReference>(method.getParams().size());
    var selfParam = method.getSelfParam();

    LinkedHashSet<Generic> generics = TypeUtil.generics(method);
    TypeBinding ownerTypeBinding = owner.getBinding();

    var genericsTypeBinding = new TypeBinding(generics);
    var typeBinding = ownerTypeBinding.leftOuterMerge(genericsTypeBinding);

    if (selfParam.isByReference()) {
      var innerSelfType = selfParam.type().asRef().getInnerType();
      if (innerSelfType.isGeneric()) {
        typeBinding.bindGeneric(innerSelfType.asGeneric(), owner.type());
      }
    } else if (selfParam.isGeneric()) {
      // Blanket trait implementations have a completely generic self type
      typeBinding.bindGeneric(selfParam.type().asGeneric(), owner.type());
    }

    typeBinding.getUnboundGenerics().stream().map(g -> Pair.with(g, getTypeFor(g)))
        .filter(p -> p.getValue1().isPresent())
        .forEach(p -> typeBinding.bindGeneric(p.getValue0(), p.getValue1().get()));

    if (typeBinding.hasUnboundedGeneric()) {
      logger.warn("Could not bind all generics for {}", typeBinding);
      return Optional.empty();
    }

    VarReference selfArgument = owner;
    if (selfParam.isByReference() && !owner.type().isRef()) {
      var type = selfParam.type().bindGenerics(typeBinding);
      selfArgument = createVariable(type);
      Statement refStmt = (selfParam.type().asRef().isMutable())
          ? RefItem.MUTABLE.toStmt(this, List.of(owner), selfArgument)
          : RefItem.IMMUTABLE.toStmt(this, List.of(owner), selfArgument);

      var refTypeBinding = TypeBinding.fromTypes(type, owner.type());
      selfArgument.setBinding(refTypeBinding);
      statements.add(refStmt);
    }

    args.add(selfArgument);

    if (selfParam.isGeneric()) {
      // We know the concrete type of the owner variable at this point, so bind it
      // We have to set all other bindings, as there might be some
      typeBinding.bindGeneric(selfParam.type().asGeneric(), owner.type());
    }

    typeBinding.getUnboundGenerics().stream()
        .map(g -> Pair.with(g, getTypeFor(g)))
        .filter(p -> p.getValue1().isPresent())
        .forEach(p -> typeBinding.bindGeneric(p.getValue0(), p.getValue1().get()));

    if (typeBinding.hasUnboundedGeneric()) {
      logger.warn("({}) Could not bind all generics: {}", id,
          typeBinding.getUnboundGenerics());
      return Optional.empty();
    }

    for (int i = 1; i < method.getParams().size(); i++) {
      var param = method.getParams().get(i);
      var boundParam = param.bindGenerics(typeBinding);
      var arg = generateArg(boundParam.type(), method.globalId(), filePathBinding);
      arg.ifPresent(args::add);
    }

    if (args.size() != method.getParams().size()) {
      return Optional.empty();
    }

    VarReference returnValue = null;
    if (method.returnsValue()) {
      // TODO: 14.02.22 there probably will be some troubles with type binding
      returnValue = createVariable(method.getReturnType().bindGenerics(typeBinding));
      returnValue.setBinding(typeBinding);
    }

    var stmt = method.toStmt(this, args, returnValue);
    statements.add(stmt);
    return Optional.ofNullable(returnValue);
  }

  public Optional<VarReference> insertCallable(Callable callable, String filePathBinding) {
    return insertCallable(callable, new TypeBinding(), filePathBinding);
  }

  private Optional<VarReference> insertCallable(Callable callable, TypeBinding typeBinding,
      String filePathBinding) {
    if (!canBeInserted(callable)) {
      return Optional.empty();
    }

    logger.debug("({}) Inserting callable {}", id, callable);
    LinkedHashSet<Generic> generics = callable.getParams().stream()
        .map(Param::type)
        .map(TypeUtil::getDeepGenerics)
        .collect(LinkedHashSet::new, LinkedHashSet::addAll, LinkedHashSet::addAll);
    if (callable.isMethod()) {
      generics.addAll(
          Stream.concat(callable.getParent().generics().stream(),
              callable.asMethod().generics().stream())
              .filter(Type::isGeneric)
              .map(Type::asGeneric)
              .collect(Collectors.toSet()));
    } else if (callable.isStaticMethod()) {
      generics.addAll(
          callable.asStaticMethod().generics().stream().filter(Type::isGeneric).map(Type::asGeneric)
              .collect(
                  Collectors.toSet()));
    }

    if (callable.returnsValue()) {
      generics.addAll(TypeUtil.getDeepGenerics(callable.getReturnType()));
    }

    logger.debug("({}) Its generics are: {}", id, generics);

    typeBinding.addGenerics(generics);

    generics.stream().map(g -> Pair.with(g, getTypeFor(g)))
        .filter(p -> p.getValue1().isPresent())
        .forEach(p -> typeBinding.bindGeneric(p.getValue0(), p.getValue1().get()));
    if (typeBinding.hasUnboundedGeneric()) {
      logger.warn("({}) Could not bind all generics: {}", id,
          typeBinding.getUnboundGenerics());
      return Optional.empty();
    }

    var args = callable.getParams().stream()
        .map(p -> {
          Type typeToGenerate = p.type().bindGenerics(typeBinding);
          logger.debug("({}) Bounded param {} to {}", id, p, typeToGenerate);

          return generateArg(typeToGenerate, callable.globalId(), filePathBinding);
        })
        .filter(Optional::isPresent)
        .map(Optional::get)
        .collect(toCollection(ArrayList::new));

    if (args.size() != callable.getParams().size()) {
      logger.warn("({}) Could not generate all arguments", id);
      return Optional.empty();
    }

    VarReference returnValue = null;
    if (callable.returnsValue()) {
      returnValue = createVariable(callable.getReturnType().bindGenerics(typeBinding));
      returnValue.setBinding(typeBinding);
    }

    var stmt = callable.toStmt(this, args, returnValue);
    logger.info("({}) Pushing " + stmt + " at the end of the test", id);
    appendStmt(stmt);
    return Optional.ofNullable(returnValue);
  }

  private boolean canBeInserted(Callable callable) {
    if (callable.isPublic()) {
      return true;
    }

    var path = callable.getSrcFilePath();
    var testBoundTo = getFilePathBinding();
    return testBoundTo.isEmpty() || testBoundTo.get().equals(path);

  }

  /**
   * Either looks for an existing usable variable or creates a new variable to use
   * as argument for
   * the given type.
   *
   * @param type             The type to get an argument for.
   * @param usableBeforeLine The line number the possible argument shall be usable
   *                         until
   *                         (exclusively).
   * @return Argument if possible.
   */
  public Optional<VarReference> getArg(Type type, int usableBeforeLine, String filePathBinding) {
    Optional<VarReference> arg = Optional.empty();
    if (type.isRef()) {
      var borrowableVariables = borrowableVariablesOfType(type, usableBeforeLine);
      if (!borrowableVariables.isEmpty()) {
        var rawArg = Rnd.choice(borrowableVariables);
        if (rawArg.type().isRef()) {
          // Don't reference a reference, just use it directly
          arg = Optional.of(rawArg);
        } else {
          arg = Optional.of(referenceVariable(rawArg, true));
        }
      }
    } else {
      var consumableVariables = consumableVariablesOfType(type, usableBeforeLine);
      if (!consumableVariables.isEmpty()) {
        arg = Optional.of(Rnd.choice(consumableVariables));
      }
    }

    if (arg.isPresent()) {
      return arg;
    } else {
      return generateArg(type, null, filePathBinding);
    }
  }

  /**
   * A convenience method, when we need to generate an arg for a type directly
   * instead of a generic
   * param
   */
  public Optional<VarReference> generateArg(Type type, String globalId, String filePathBinding) {
    return generateArg(Objects.requireNonNull(type), new HashSet<>(), globalId, filePathBinding);
  }

  private Optional<VarReference> generateArg(Type type,
      Set<Type> typesToGenerate, String globalId, String filePathBinding) {
    logger.debug("({}) Starting to generate an argument for type {}", id, type);
    if (type.isPrim()) {
      return Optional.of(generatePrimitive(type.asPrimitive(), globalId));
    } else if (type.isStruct() || type.isEnum()) {
      var generators = tyCtxt.generatorsOf(type, filePathBinding);
      logger.debug("({}) Found " + generators.size() + " generators", id);
      return generateArgFromGenerators(type, generators, typesToGenerate, filePathBinding);
    } else if (type.isRef()) {
      var reference = type.asRef();
      return generateReference(reference, typesToGenerate, filePathBinding);
      // return generateArgFromGenerators(type, generators, typesToGenerate);
    } else if (type.isArray()) {
      return generateArray(type.asArray(), typesToGenerate, filePathBinding);
    } else if (type.isTuple()) {
      var tuple = type.asTuple();
      return generateTuple(tuple, typesToGenerate, filePathBinding);
    } else if (type.isSlice()) {
      // var array = generateArray(type.asSlice().type(), typesToGenerate);
      // return array.map(a -> referenceVariable(a, true));
      return Optional.empty();
    } else {
      return Optional.empty();
      // throw new RuntimeException("Not implemented: " + type);
    }
  }

  private Optional<VarReference> generateReference(Ref ref, Set<Type> typesToGenerate,
      String filePathBinding) {
    RefItem refItem = RefItem.MUTABLE;

    var typeBinding = TypeUtil.typeBinding(ref);
    if (typeBinding.hasUnboundedGeneric()) {
      logger.warn("({}) Could not bind all generics: {}", id,
          typeBinding.getUnboundGenerics());
      return Optional.empty();
    }

    var refType = ref.getInnerType().bindGenerics(typeBinding);
    if (typesToGenerate.contains(refType)) {
      return Optional.empty();
    }

    var extendedTypesToGenerate = new HashSet<>(typesToGenerate);
    extendedTypesToGenerate.add(ref);
    var arg = generateArg(refType, extendedTypesToGenerate, null, filePathBinding);
    if (arg.isEmpty()) {
      return Optional.empty();
    }

    var returnValue = createVariable(ref.bindGenerics(typeBinding));
    returnValue.setBinding(typeBinding);

    var stmt = refItem.toStmt(this, Collections.singletonList(arg.get()), returnValue);
    addStmt(stmt);
    return Optional.of(returnValue);
  }

  private Optional<VarReference> generateTuple(Tuple tuple, Set<Type> typesToGenerate,
      String filePathBinding) {
    var params = tuple.getTypes().stream().map(t -> new Param(t, false, null)).toList();
    var tupleInit = new TupleInit(params);

    var typeBinding = TypeUtil.typeBinding(tuple);
    if (typeBinding.hasUnboundedGeneric()) {
      logger.warn("({}) Could not bind all generics: {}", id,
          typeBinding.getUnboundGenerics());
      return Optional.empty();
    }

    var extendedTypesToGenerate = new HashSet<>(typesToGenerate);
    extendedTypesToGenerate.add(tuple);
    var tupleTypes = tuple.getTypes().stream().map(t -> t.bindGenerics(typeBinding)).toList();
    var args = tupleTypes.stream()
        .map(innerType -> generateArg(innerType, extendedTypesToGenerate, null, filePathBinding))
        .filter(Optional::isPresent)
        .map(Optional::get)
        .collect(Collectors.toList());

    if (args.size() == tuple.getTypes().size()) {
      var returnValue = createVariable(tuple.bindGenerics(typeBinding));
      returnValue.setBinding(typeBinding);
      var stmt = tupleInit.toStmt(this, args, returnValue);
      addStmt(stmt);
      return Optional.of(returnValue);
    } else {
      return Optional.empty();
    }
  }

  private Optional<VarReference> generateSlice(Slice slice, Set<Type> typesToGenerate) {
    throw new RuntimeException("Not implemented");
  }

  private Optional<VarReference> generateArray(Array array, Set<Type> typesToGenerate,
      String filePathBinding) {
    // TODO: 27.02.22 1) [T; N] where T: Default (and N <= 32)
    // TODO: 27.02.22 [T; N] where T: Copy
    // TODO: 27.02.22 literal array init
    if (array.implementedTraits().contains(Default.getInstance())) {
      throw new RuntimeException("Not implemented");
    } else if (array.implementedTraits().contains(Copy.getInstance())) {
      throw new RuntimeException("Not implemented");
    } else {
      var arrayInit = new ArrayInit(array);

      var typeBinding = TypeUtil.typeBinding(array);
      if (typeBinding.hasUnboundedGeneric()) {
        logger.warn("({}) Could not bind all generics: {}", id,
            typeBinding.getUnboundGenerics());
        return Optional.empty();
      }

      var extendedTypesToGenerate = new HashSet<>(typesToGenerate);
      extendedTypesToGenerate.add(array);
      var actualElementsType = array.type().bindGenerics(typeBinding);
      var elements = IntStream.range(0, array.length())
          .mapToObj(
              i -> generateArg(actualElementsType, extendedTypesToGenerate, null, filePathBinding))
          .filter(Optional::isPresent)
          .map(Optional::get)
          .collect(toCollection(ArrayList::new));

      if (elements.size() != array.length()) {
        logger.warn("Could not generate all elements for {}", array);
        return Optional.empty();
      }

      var returnValue = createVariable(array.bindGenerics(typeBinding));
      returnValue.setBinding(typeBinding);

      var stmt = arrayInit.toStmt(this, elements, returnValue);
      addStmt(stmt);
      return Optional.of(returnValue);
    }
  }

  private VarReference generatePrimitive(Prim prim, String globalId) {
    logger.debug("({}) Starting to generate a primitive", id);
    if (SeedOptions.useConstantPool()
        && Rnd.get().nextDouble() < Constants.P_CONSTANT_POOL) {
      var constants = MirAnalysis.constantPool()
          .stream().filter(val -> val.type().equals(prim)).collect(Collectors.toSet());
      if (!constants.isEmpty()) {
        var val = Rnd.choice(constants);
        var var = createVariable(val.type());
        var stmt = new PrimitiveStmt(this, var, val);
        statements.add(0, stmt);

        logger.debug("({}) Selected a constant from constant pool: {}", id, val);
        return var;
      }
    }

    var val = prim.random();
    var var = createVariable(prim);
    var stmt = new PrimitiveStmt(this, var, val);
    statements.add(0, stmt);
    logger.debug("({}) Generated a primitive: {}", id, val);
    return var;
  }

  private VarReference insertPrimitive(Prim prim, PrimitiveValue<?> val) {
    var var = createVariable(prim);
    var stmt = new PrimitiveStmt(this, var, val);
    statements.add(0, stmt);
    return var;
  }

  private Optional<VarReference> generateArgFromGenerators(Type type, List<Callable> generators,
      Set<Type> typesToGenerate, String filePathBinding) {

    logger.debug("({}) Starting to generate a {} with {} generator options", id, type,
        generators.size());
    boolean retry = true;
    Callable generator = null;
    while (retry && !generators.isEmpty()) {
      retry = false;

      var candidateGenerator = Rnd.choice(generators);
      var paramTypes = candidateGenerator.getParams().stream().map(Param::type)
          .collect(Collectors.toSet());
      paramTypes.retainAll(typesToGenerate);
      if (!paramTypes.isEmpty()) {
        // We already try to generate a type which is needed as an argument for the call
        // Hence, this would probably lead to infinite recursive chain. Remove the
        // generator and retry with another one.
        generators.remove(candidateGenerator);
        retry = true;
      } else {
        generator = candidateGenerator;
      }
    }

    if (generator == null) {
      logger.warn("({}) Could not find appropriate generator for {}", id, type);
      return Optional.empty();
    }

    if (!generator.isPublic()) {
      filePathBinding = generator.getSrcFilePath();
    }

    logger.debug("({}) Selected generator: {} (Total: {})", id, generator, generators.size());
    // var typeBinding = TypeUtil.typeBinding(type, generator);
    TypeBinding typeBinding;
    try {
      typeBinding = TypeUtil.typeBinding(type, generator);
    } catch (Exception e) {
      generators.remove(generator);
      return generateArgFromGenerators(type, generators, typesToGenerate, filePathBinding);
    }
    if (typeBinding.hasUnboundedGeneric()) {
      logger.warn("Could not bind all generics: {}", typeBinding.getUnboundGenerics());
      generators.remove(generator);
      return generateArgFromGenerators(type, generators, typesToGenerate, filePathBinding);
      // instead of giving up, try another generator
    }

    // This needs to be final for the lambda
    String subsequentPathBinding;
    if (!generator.isPublic()) {
      subsequentPathBinding = generator.getSrcFilePath();
    } else {
      subsequentPathBinding = filePathBinding;
    }
    var globalId = generator.globalId();
    var args = generator.getParams().stream()
        .map(p -> {
          if (typesToGenerate.contains(type)) {
            return Optional.<VarReference>empty();
          }

          var extendedTypesToGenerate = new HashSet<>(typesToGenerate);
          extendedTypesToGenerate.add(type);
          return generateArg(p.type().bindGenerics(typeBinding),
              extendedTypesToGenerate, globalId, subsequentPathBinding);
        })
        .filter(Optional::isPresent)
        .map(Optional::get)
        .collect(toCollection(ArrayList::new));
    if (size() >= Constants.CHROMOSOME_LENGTH) {
      return Optional.empty();
    }

    if (args.size() != generator.getParams().size()) {
      generators.remove(generator);
      return generateArgFromGenerators(type, generators, typesToGenerate, filePathBinding);
      // instead of giving up, try another generator
    }

    VarReference returnValue = null;
    if (generator.returnsValue()) {
      if (type.isRef() && !generator.getReturnType().isRef()) {
        // Unwrap the type
        var innerType = type.asRef().getInnerType();
        returnValue = createVariable(innerType);
      } else if (!type.isRef() && generator.getReturnType().isRef()) {
        throw new RuntimeException("Not implemented");
      } else {
        returnValue = createVariable(type);
      }

      returnValue.setBinding(typeBinding);
    }

    var stmt = generator.toStmt(this, args, returnValue);
    addStmt(stmt);
    return Optional.ofNullable(returnValue);
  }

  private VarReference createVariable(Type type) {
    logger.debug("({}) Created variable of type {}", id, type);
    return new VarReference(this, type);
  }

  public Map<MinimizingFitnessFunction<TestCase>, Double> branchDistance() {
    return coverage;
  }

  public int codeLines() {
    // Macros + method head + final curly brace + empty line
    return statements.size() + 2 + 1 + 1 + 1;
  }

  public String getTypeBindingsString() {
    var sb = new StringBuilder();
    var visitor = new TypeBindingStringVisitor(this);
    /*
     * typeBindings.forEach((key, value) ->
     * sb.append(visitor.getVariableName(key)).append(": ")
     * .append(visitor.visit(value)));
     */
    return sb.toString();
  }

  public String visit(Visitor visitor) {
    return visitor.visitTestCase(this);
  }

  @Override
  public String toString() {
    var visitor = new LineNumberDebugVisitor();
    return visit(visitor);
  }

  @Override
  public TestCase copy() {
    return new TestCase(this);
  }

  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    TestCase testCase = (TestCase) o;
    return id == testCase.id && statements.equals(testCase.statements);
  }

  @Override
  public int hashCode() {
    return Objects.hash(id, statements);
  }

  @Override
  public TestCase self() {
    return this;
  }

}
