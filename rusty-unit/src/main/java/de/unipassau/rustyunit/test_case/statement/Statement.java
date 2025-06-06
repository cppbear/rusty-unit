package de.unipassau.rustyunit.test_case.statement;

import de.unipassau.rustyunit.test_case.Param;
import de.unipassau.rustyunit.test_case.TestCase;
import de.unipassau.rustyunit.test_case.var.VarReference;
import de.unipassau.rustyunit.test_case.callable.Callable;
import de.unipassau.rustyunit.test_case.statement.array.DirectArrayInitStmt;
import de.unipassau.rustyunit.type.Type;
import de.unipassau.rustyunit.util.Rnd;
import java.util.List;
import java.util.Optional;
import java.util.UUID;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

public interface Statement {

  Logger logger = LoggerFactory.getLogger(Statement.class);

  UUID id();

  Optional<Type> returnType();

  Optional<VarReference> returnValue();

  boolean returnsValue();

  List<VarReference> args();

  void setArgs(List<VarReference> args);

  void setArg(int pos, VarReference var);

  List<Param> params();

  List<Type> actualParamTypes();

  TestCase testCase();

  String getSrcFilePath();

  boolean isPublic();

  default boolean isRefStmt() {
    return false;
  }

  default boolean isPrimitiveStmt() {
    return false;
  }

  default boolean isTupleStmt() {
    return false;
  }

  default boolean isEnumStmt() {
    return false;
  }

  default boolean isStructInitStmt() {
    return false;
  }

  default boolean isCallableStmt() {
    return false;
  }

  default boolean isStaticMethodStmt() {
    return false;
  }

  default boolean isMethodStmt() {
    return false;
  }

  default boolean isArrayStmt() {
    return false;
  }

  default boolean isTupleAccessStmt() {
    return false;
  }

  default RefStmt asRefStmt() {
    throw new RuntimeException("Not with me");
  }

  default PrimitiveStmt asPrimitiveStmt() {
    throw new RuntimeException("Not with me");
  }

  default StructInitStmt asStructInitStmt() {
    throw new RuntimeException("Not with me");
  }

  default CallableStmt asCallableStmt() {
    throw new RuntimeException("Not with me");
  }

  default StaticMethodStmt asStaticMethodStmt() {
    throw new RuntimeException("Not with me");
  }

  default MethodStmt asMethodStmt() {
    throw new RuntimeException("Not with me");
  }

  default EnumStmt asEnumStmt() {
    throw new RuntimeException("Not with me");
  }

  default TupleStmt asTupleStmt() {
    throw new RuntimeException("Not with me");
  }

  default TupleAccessStmt asTupleAccessStmt() {
    throw new RuntimeException("Not with me");
  }

  default DirectArrayInitStmt asArrayStmt() {
    throw new RuntimeException("Not with me");
  }

  Callable getCallable();

  boolean consumes(VarReference var);

  boolean borrows(VarReference var);

  boolean mutates(VarReference var);

  default boolean uses(VarReference var) {
    return false;
  }

  default boolean mutate(TestCase testCase) {
    if (params().isEmpty()) {
      return false;
    }

    var pChangeParam = 1d / params().size();
    boolean changed = false;
    for (int iParam = 0; iParam < params().size(); iParam++) {
      if (Rnd.get().nextDouble() < pChangeParam) {
        logger.info("Mutating argument #{}", iParam);
        if (mutateParameter(iParam)) {
          changed = true;
        }
      }
    }

    return changed;
  }

  default int numParamsOfType(Type type) {
    return (int) args().stream().map(VarReference::type).filter(t -> t.equals(type)).count();
  }

  /**
   * Mutates parameter at position pos.
   *
   * @param pos The position index of the parameter.
   * @return true if the param has been mutated, false otherwise.
   */
  default boolean mutateParameter(int pos) {
    var paramType = actualParamTypes().get(pos);
    var currentVar = args().get(pos);

    List<VarReference> usableVariables;
    if (paramType.isRef() || isRefStmt()) {
      usableVariables = testCase().borrowableVariablesOfType(paramType, position());
    } else {
      usableVariables = testCase().consumableVariablesOfType(paramType, position());
    }

    // Don't use the return value of the statement as its own parameter
    returnValue().ifPresent(usableVariables::remove);

    // Don't use the original parameter
    usableVariables.remove(currentVar);

    int numParamsOfThatType = numParamsOfType(paramType);

    // If there are fewer objects than parameters of that type,
    // we consider adding an instance

    // Try it max 3 times
    int counter = 0;
    while (counter < 3 && numParamsOfThatType + 1 > usableVariables.size()) {
      logger.info("Too few usable variables, trying to generate another one");
      var var = testCase().getArg(paramType, position(), testCase().getFilePathBinding().orElse(null));
      if (var.isPresent() && var.get().position() < position()) {
        usableVariables.add(var.get());
        logger.info("Generated variable of type {}", var.get().type());
      }
      counter++;
    }

    if (usableVariables.isEmpty()) {
      logger.warn("Could not mutate param #{} of type {} ", pos, paramType);
      return false;
    }

    var replacement = Rnd.choice(usableVariables);
    logger.info("Replacing argument at position {}", pos);
    replaceAt(pos, replacement);

    return true;
  }

  void replace(VarReference oldVar, VarReference newVar);

  default void replaceAt(int pos, VarReference var) {
    /*var typeBinding = testCase().popTypeBindingsFor(args().get(pos));
    testCase().setTypeBindingsFor(var, typeBinding);*/

    setArg(pos, var);
  }

  Statement copy(TestCase testCase);

  int position();
}
